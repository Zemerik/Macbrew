use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use colored::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Running,
    Stopped,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: usize,
    pub command: String,
    pub pid: Option<u32>,
    pub status: JobStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub output: String,
    pub error: String,
}

pub struct JobManager {
    jobs: Arc<Mutex<HashMap<usize, Job>>>,
    next_id: Arc<Mutex<usize>>,
    background_processes: Arc<Mutex<HashMap<usize, Child>>>,
}

impl JobManager {
    pub fn new() -> Self {
        JobManager {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
            background_processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn start_job(&self, command: &str, args: &[&str], background: bool) -> Result<usize> {
        let job_id = {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let job = Job {
            id: job_id,
            command: format!("{} {}", command, args.join(" ")),
            pid: None,
            status: JobStatus::Running,
            start_time: Utc::now(),
            end_time: None,
            exit_code: None,
            output: String::new(),
            error: String::new(),
        };

        {
            let mut jobs = self.jobs.lock().unwrap();
            jobs.insert(job_id, job);
        }

        if background {
            self.run_background_job(job_id, command, args)?;
        } else {
            self.run_foreground_job(job_id, command, args)?;
        }

        Ok(job_id)
    }

    fn run_background_job(&self, job_id: usize, command: &str, args: &[&str]) -> Result<()> {
        let jobs = Arc::clone(&self.jobs);
        let background_processes = Arc::clone(&self.background_processes);
        
        // Clone the strings to own them before moving into thread
        let command = command.to_string();
        let args: Vec<String> = args.iter().map(|&s| s.to_string()).collect();

        thread::spawn(move || {
            let child = match Command::new(&command)
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(child) => child,
                Err(e) => {
                    eprintln!("Failed to start background job: {}", e);
                    return;
                }
            };

            let pid = child.id();
            
            // Update job with PID
            {
                let mut jobs = jobs.lock().unwrap();
                if let Some(job) = jobs.get_mut(&job_id) {
                    job.pid = Some(pid);
                }
            }

            // Store the child process
            {
                let mut processes = background_processes.lock().unwrap();
                processes.insert(job_id, child);
            }

            // Monitor the job
            Self::monitor_background_job(job_id, jobs, background_processes);
        });

        Ok(())
    }

    fn run_foreground_job(&self, job_id: usize, command: &str, args: &[&str]) -> Result<()> {
        let output = Command::new(command)
            .args(args)
            .output()?;

        let status = if output.status.success() {
            JobStatus::Completed
        } else {
            JobStatus::Failed
        };

        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(&job_id) {
            job.status = status;
            job.end_time = Some(Utc::now());
            job.exit_code = output.status.code();
            job.output = String::from_utf8_lossy(&output.stdout).to_string();
            job.error = String::from_utf8_lossy(&output.stderr).to_string();
        }

        Ok(())
    }

    fn monitor_background_job(
        job_id: usize,
        jobs: Arc<Mutex<HashMap<usize, Job>>>,
        background_processes: Arc<Mutex<HashMap<usize, Child>>>,
    ) {
        let mut last_check = Instant::now();
        
        loop {
            thread::sleep(Duration::from_millis(100));

            let mut processes = background_processes.lock().unwrap();
            if let Some(child) = processes.get_mut(&job_id) {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        // Job completed
                        let mut jobs = jobs.lock().unwrap();
                        if let Some(job) = jobs.get_mut(&job_id) {
                            job.status = if status.success() {
                                JobStatus::Completed
                            } else {
                                JobStatus::Failed
                            };
                            job.end_time = Some(Utc::now());
                            job.exit_code = status.code();
                        }
                        processes.remove(&job_id);
                        break;
                    }
                    Ok(None) => {
                        // Job still running
                        if last_check.elapsed() > Duration::from_secs(5) {
                            // Update job status periodically
                            last_check = Instant::now();
                        }
                    }
                    Err(_) => {
                        // Error checking job status
                        let mut jobs = jobs.lock().unwrap();
                        if let Some(job) = jobs.get_mut(&job_id) {
                            job.status = JobStatus::Failed;
                            job.end_time = Some(Utc::now());
                        }
                        processes.remove(&job_id);
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn list_jobs(&self) -> Vec<Job> {
        let jobs = self.jobs.lock().unwrap();
        jobs.values().cloned().collect()
    }

    pub fn get_job(&self, job_id: usize) -> Option<Job> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(&job_id).cloned()
    }

    pub fn kill_job(&self, job_id: usize) -> Result<()> {
        let mut processes = self.background_processes.lock().unwrap();
        if let Some(mut child) = processes.remove(&job_id) {
            child.kill()?;
            
            let mut jobs = self.jobs.lock().unwrap();
            if let Some(job) = jobs.get_mut(&job_id) {
                job.status = JobStatus::Failed;
                job.end_time = Some(Utc::now());
            }
        }

        Ok(())
    }

    pub fn stop_job(&self, job_id: usize) -> Result<()> {
        let mut processes = self.background_processes.lock().unwrap();
        if let Some(_child) = processes.get_mut(&job_id) {
            // On Unix systems, we would send SIGSTOP
            // For now, we'll just mark it as stopped
            let mut jobs = self.jobs.lock().unwrap();
            if let Some(job) = jobs.get_mut(&job_id) {
                job.status = JobStatus::Stopped;
            }
        }

        Ok(())
    }

    pub fn resume_job(&self, job_id: usize) -> Result<()> {
        let mut processes = self.background_processes.lock().unwrap();
        if let Some(_child) = processes.get_mut(&job_id) {
            // On Unix systems, we would send SIGCONT
            let mut jobs = self.jobs.lock().unwrap();
            if let Some(job) = jobs.get_mut(&job_id) {
                job.status = JobStatus::Running;
            }
        }

        Ok(())
    }

    pub fn bring_to_foreground(&self, job_id: usize) -> Result<()> {
        let mut processes = self.background_processes.lock().unwrap();
        if let Some(mut child) = processes.remove(&job_id) {
            // Wait for the process to complete
            let status = child.wait()?;
            
            let mut jobs = self.jobs.lock().unwrap();
            if let Some(job) = jobs.get_mut(&job_id) {
                job.status = if status.success() {
                    JobStatus::Completed
                } else {
                    JobStatus::Failed
                };
                job.end_time = Some(Utc::now());
                job.exit_code = status.code();
            }
        }

        Ok(())
    }

    pub fn cleanup_completed_jobs(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.retain(|_, job| {
            matches!(job.status, JobStatus::Running | JobStatus::Stopped)
        });
    }

    pub fn get_job_count(&self) -> usize {
        let jobs = self.jobs.lock().unwrap();
        jobs.len()
    }

    pub fn get_running_jobs(&self) -> Vec<Job> {
        let jobs = self.jobs.lock().unwrap();
        jobs.values()
            .filter(|job| matches!(job.status, JobStatus::Running))
            .cloned()
            .collect()
    }

    pub fn get_stopped_jobs(&self) -> Vec<Job> {
        let jobs = self.jobs.lock().unwrap();
        jobs.values()
            .filter(|job| matches!(job.status, JobStatus::Stopped))
            .cloned()
            .collect()
    }

    pub fn print_jobs(&self) {
        let jobs = self.list_jobs();
        if jobs.is_empty() {
            println!("No jobs");
            return;
        }

        println!("{}", "Jobs:".bold());
        for job in jobs {
            let status_str = match job.status {
                JobStatus::Running => "Running".green(),
                JobStatus::Stopped => "Stopped".yellow(),
                JobStatus::Completed => "Completed".blue(),
                JobStatus::Failed => "Failed".red(),
            };

            let pid_str = job.pid.map(|pid| pid.to_string()).unwrap_or_else(|| "N/A".to_string());
            let exit_code_str = job.exit_code.map(|code| code.to_string()).unwrap_or_else(|| "N/A".to_string());

            println!("[{}] {} {} {} {}", 
                job.id, 
                status_str, 
                pid_str, 
                exit_code_str, 
                job.command
            );
        }
    }
}

impl Default for JobManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_creation() {
        let manager = JobManager::new();
        let job_id = manager.start_job("echo", &["hello"], false).unwrap();
        assert_eq!(job_id, 1);
    }

    #[test]
    fn test_job_listing() {
        let manager = JobManager::new();
        manager.start_job("echo", &["hello"], false).unwrap();
        let jobs = manager.list_jobs();
        assert_eq!(jobs.len(), 1);
    }
} 
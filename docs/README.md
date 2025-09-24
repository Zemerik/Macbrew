# Macbrew Documentation

This is the documentation website for Macbrew, built with Astro.

## Features

- 📚 Comprehensive documentation
- 🎨 Modern, responsive design
- 🌙 Dark/light mode support
- 🔍 Full-text search
- 📱 Mobile-friendly
- ⚡ Fast performance

## Getting Started

### Prerequisites

- Node.js 16+ 
- npm or yarn

### Installation

1. Install dependencies:
```bash
npm install
```

2. Start the development server:
```bash
npm run dev
```

3. Open [http://localhost:4321](http://localhost:4321) in your browser.

## Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run astro` - Run Astro CLI commands

### Project Structure

```
docs/
├── src/
│   ├── components/     # Reusable components
│   ├── layouts/        # Page layouts
│   ├── pages/          # Documentation pages
│   └── styles/         # Global styles
├── public/             # Static assets
├── astro.config.mjs    # Astro configuration
├── tailwind.config.mjs # Tailwind CSS configuration
└── package.json        # Dependencies and scripts
```

### Adding New Pages

1. Create a new `.md` or `.mdx` file in `src/pages/docs/`
2. Add frontmatter with layout and metadata
3. Write your content using Markdown

Example:
```markdown
---
layout: ../../layouts/Layout.astro
title: My Page - Macbrew Documentation
description: Description of the page
---

# My Page

Content goes here...
```

### Styling

The site uses Tailwind CSS for styling. You can:

- Use Tailwind utility classes directly
- Add custom styles in `src/styles/`
- Modify the design system in `tailwind.config.mjs`

### Components

Reusable components are in `src/components/`. They can be:

- Astro components (`.astro`)
- React components (`.tsx`)
- Vue components (`.vue`)

## Deployment

### Build for Production

```bash
npm run build
```

The built site will be in the `dist/` directory.

### Deploy to Netlify

1. Connect your repository to Netlify
2. Set build command: `npm run build`
3. Set publish directory: `dist`

### Deploy to Vercel

1. Connect your repository to Vercel
2. Vercel will automatically detect Astro and configure the build

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally with `npm run dev`
5. Submit a pull request

## License

This documentation is licensed under the MIT License. 
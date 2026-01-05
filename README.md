# MDViewer

A clean, minimal Markdown viewer for macOS built with Tauri.

## Features

- **GitHub-flavored Markdown** - Full GFM support with syntax highlighting for code blocks
- **Dark Mode** - Automatically adapts to your system appearance
- **Drag & Drop** - Drop any Markdown file onto the window to view
- **File Associations** - Double-click `.md` files to open directly in MDViewer
- **In-Document Search** - Find text with `Cmd+F`, navigate with `Cmd+G`
- **External Links** - Links open in your default browser
- **Minimal & Fast** - Lightweight native app, instant startup

## Supported File Types

`.md`, `.markdown`, `.mdown`, `.mkd`, `.txt`

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+O` | Open file |
| `Cmd+F` | Find in document |
| `Cmd+G` | Next search result |
| `Cmd+Shift+G` | Previous search result |
| `Enter` | Next result (in search) |
| `Shift+Enter` | Previous result (in search) |
| `Esc` | Close search |

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run dev
```

### Build

```bash
# Build for production
npm run build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Tech Stack

- **Tauri 2.x** - Rust-based desktop framework
- **marked.js** - Markdown parser
- **highlight.js** - Syntax highlighting
- **Vanilla JS** - No frontend framework

## Project Structure

```
MDViewer/
├── src/
│   └── index.html      # Frontend (HTML/CSS/JS)
├── src-tauri/
│   ├── src/
│   │   ├── main.rs     # Entry point
│   │   └── lib.rs      # Core logic
│   ├── tauri.conf.json # App configuration
│   ├── icons/          # App icons
│   └── Cargo.toml      # Rust dependencies
└── package.json
```

## License

MIT

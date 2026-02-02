# czytaj

Czytaj is a modern RSS/ATOM reader built with Tauri and React, featuring a Google News-inspired interface with beautiful light and dark themes.

## Features

- 📰 **RSS/ATOM Feed Reader** - Subscribe to your favorite feeds
- 📖 **Article Viewer** - Click articles to read in a dedicated detail view
- 💾 **Read Status Tracking** - Automatically tracks which articles you've viewed
- 🎨 **Light & Dark Themes** - Beautiful, modern themes for any preference
- ✨ **Google News UI** - Clean, card-based layout inspired by Google News
- 🌓 **Collapsible Sidebar** - Maximize your reading space
- 📱 **Responsive Design** - Works great on different screen sizes
- 🔒 **Privacy-Focused** - All data stored locally with SQLite

## Technology Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri
- **Database**: SQLite (rusqlite)
- **Feed Parsing**: feed-rs
- **Styling**: Pure CSS with CSS Variables for theming

## Building from Source

### Prerequisites

- Node.js (v16 or later)
- Rust (latest stable)
- Platform-specific dependencies for Tauri:
  - **Linux**: `webkit2gtk`, `libayatana-appindicator3-dev` (see [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Microsoft Visual C++ Build Tools

### Build Steps

```bash
# Clone the repository
git clone https://github.com/nixenos/czytaj.git
cd czytaj

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Usage

1. Launch the application
2. Enter an RSS/ATOM feed URL in the sidebar
3. Click "Add Feed" to subscribe
4. Articles will appear in the main content area as cards
5. **Click any article** to view it in full detail
6. Use the settings button to customize appearance and toggle features
7. Viewed articles appear with reduced opacity for easy tracking
8. Click the hamburger menu (☰) to collapse/expand the sidebar

## Architecture

### Backend (Rust/Tauri)

The backend handles all business logic and data persistence:

- **Feed Engine**: Fetches and parses RSS/ATOM feeds using `feed-rs`
- **Database**: SQLite for storing viewed articles
- **HTML Sanitizer**: Cleans and processes article content
- **Tauri Commands**: Exposes async functions to the frontend via IPC

### Frontend (React + TypeScript)

The frontend provides a modern, responsive UI:

- **Component-Based**: Modular React components for maintainability
- **Type-Safe**: Full TypeScript coverage
- **CSS Variables**: Dynamic theming without JavaScript
- **Google News Style**: Card-based layout with hover effects and shadows

## Features in Detail

### Collapsible Sidebar
- Toggle sidebar with hamburger menu button
- Feed management and settings access
- Persistent across sessions

### Article Detail View
- Full article title and excerpt
- Optional image display
- External link to original article
- Back navigation

### Settings Page
- Theme selection (Light/Dark)
- Toggle image display
- Toggle excerpt display
- Settings saved automatically

### Read Status Persistence
- Articles marked as viewed when clicked
- Viewed articles shown with reduced opacity
- Status persists across app restarts

## Design Principles

- **Material Design Inspired**: Elevation, shadows, and hover effects
- **Typography**: Clear hierarchy with readable font sizes
- **Spacing**: Generous padding for comfortable reading
- **Animations**: Smooth transitions for better UX
- **Accessibility**: Semantic HTML and proper contrast ratios

## License

See [LICENSE](LICENSE) for details.

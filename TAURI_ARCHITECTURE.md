# Tauri Application Architecture

## Overview
Czytaj has been completely rewritten from an iced desktop application to a modern Tauri application with a React frontend and Google News-inspired UI.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Tauri Application                        │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐                    ┌──────────────────┐   │
│  │   Frontend   │◄──── IPC ────────►│     Backend      │   │
│  │  React + TS  │     (Tauri)       │   Rust + Tauri   │   │
│  └──────────────┘                    └──────────────────┘   │
│         │                                      │             │
│         │                                      │             │
│    ┌────▼────┐                           ┌────▼────┐        │
│    │  Vite   │                           │ SQLite  │        │
│    │  Build  │                           │   DB    │        │
│    └─────────┘                           └─────────┘        │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Frontend Structure (React + TypeScript)

### Component Hierarchy
```
App (Main Container)
├── Sidebar
│   ├── Feed Input
│   ├── Feeds List
│   └── Settings Button
│
└── Main Content
    ├── Header (with toggle)
    └── Content Area
        ├── ArticleGrid (default view)
        │   └── Article Cards
        ├── ArticleDetail (article view)
        │   ├── Back Button
        │   ├── Image
        │   ├── Title
        │   ├── Excerpt
        │   └── External Link
        └── Settings (settings view)
            ├── Theme Selector
            └── Display Options
```

### State Management
- **Local State**: React useState hooks
- **Server State**: Tauri backend via IPC
- **Persistent State**: SQLite database

## Backend Structure (Rust + Tauri)

### Modules
```
src-tauri/src/
├── lib.rs            # Tauri commands & app state
├── main.rs           # Entry point
├── db/
│   └── mod.rs        # SQLite operations
├── models/
│   ├── article.rs    # Article struct
│   ├── feed.rs       # Feed structs
│   └── settings.rs   # Settings struct
├── utils/
│   └── sanitizer.rs  # HTML sanitization
└── feed_engine.rs    # RSS/Atom parsing
```

### Tauri Commands (IPC)
- `add_feed(url)` → Fetches and adds a new feed
- `refresh_feed(url)` → Refreshes articles from a feed
- `get_feeds()` → Returns all feeds
- `mark_article_viewed(url, title)` → Marks article as read
- `is_article_viewed(url)` → Checks if article is read
- `get_viewed_articles()` → Returns all viewed URLs
- `get_settings()` → Returns current settings
- `update_settings(settings)` → Updates settings

## UI Design - Google News Style

### Color System
Both light and dark themes use CSS variables for consistency:

**Light Theme:**
- Primary: White (#ffffff)
- Secondary: Light gray (#f8f9fa)
- Accent: Blue (#1a73e8)
- Text: Dark gray (#202124)

**Dark Theme:**
- Primary: Dark gray (#202124)
- Secondary: Charcoal (#292a2d)
- Accent: Light blue (#8ab4f8)
- Text: Light gray (#e8eaed)

### Layout Features

1. **Collapsible Sidebar (280px)**
   - Feed management
   - Add feed input
   - Feed list with refresh
   - Settings button
   - Smooth collapse animation

2. **Main Content Area**
   - Header with toggle button
   - Dynamic content based on view
   - Responsive grid layout
   - Smooth transitions

3. **Article Cards**
   - Optional image (200px height)
   - Title (3-line clamp)
   - Optional excerpt (3-line clamp)
   - Hover effects (shadow + lift)
   - Viewed indicator (reduced opacity)

4. **Article Detail**
   - Full-width image
   - Large title (32px)
   - Full excerpt
   - External link button
   - Back navigation

5. **Settings Page**
   - Theme toggle (Light/Dark)
   - Display options checkboxes
   - Clean, centered layout

### Design Principles

✅ **Material Design Inspired**
- Elevation with shadows
- Smooth animations
- Card-based layout

✅ **Responsive**
- Mobile-friendly sidebar
- Flexible grid
- Touch-friendly buttons

✅ **Accessible**
- Semantic HTML
- Proper contrast ratios
- Keyboard navigation

✅ **Performance**
- CSS-only animations
- Lazy loading ready
- Optimized repaints

## Key Features

### 1. Feed Management
- Add feeds via URL input
- Automatic feed title extraction
- Click to refresh individual feeds
- Persistent feed list

### 2. Article Reading
- Card grid view for browsing
- Click to view full article
- Automatic read tracking
- Visual feedback for viewed articles

### 3. Settings
- Light/Dark theme switching
- Toggle image display
- Toggle excerpt display
- Settings persist across sessions

### 4. Read Status Tracking
- SQLite-backed persistence
- Automatic marking on view
- Visual indication (opacity)
- Query viewed articles

## Build & Development

### Development Mode
```bash
npm install       # Install frontend deps
npm run tauri dev # Run in dev mode
```

### Production Build
```bash
npm run tauri build
```

The build creates:
- **Linux**: AppImage, .deb
- **macOS**: .dmg, .app
- **Windows**: .msi, .exe

## Technology Choices

### Why Tauri?
- ✅ Smaller bundle size vs Electron
- ✅ Native system integration
- ✅ Rust security benefits
- ✅ Web technologies for UI
- ✅ Cross-platform

### Why React?
- ✅ Component-based architecture
- ✅ Large ecosystem
- ✅ TypeScript support
- ✅ Excellent developer experience

### Why CSS Variables?
- ✅ Instant theme switching
- ✅ No JavaScript overhead
- ✅ Easy to customize
- ✅ Better performance

## Migration Summary

### What Changed
- **UI Framework**: iced → React
- **Architecture**: Monolithic → Client-Server (IPC)
- **Themes**: 19 iced themes → 2 custom themes
- **Styling**: iced widgets → CSS
- **Build**: Pure Rust → Tauri (Rust + Node)

### What Stayed
- ✅ Core feed fetching logic
- ✅ HTML sanitization
- ✅ SQLite database
- ✅ Article models
- ✅ All original features

### What Improved
- ✨ Modern, polished UI
- ✨ Faster development cycle
- ✨ Better code organization
- ✨ Enhanced user experience
- ✨ Easier customization

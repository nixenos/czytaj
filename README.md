# czytaj
Czytaj is a simple GUI RSS/ATOM reader written in Rust with a modern Material Design interface.

## Features

- 📰 **RSS/ATOM Feed Reader** - Subscribe to your favorite feeds
- 🎨 **23 Beautiful Themes** - Choose from Catppuccin, Tokyo Night, Dracula, Nord, Gruvbox, and more
- ✨ **Modern UI** - Material Design with smooth animations and transitions
- 🌓 **Light & Dark Themes** - Multiple options for both light and dark preferences
- 📱 **Clean Interface** - Google-inspired design with card layouts and elevation effects

## Theme Support

Czytaj supports 23 themes out of the box:
- **Catppuccin**: Mocha, Macchiato, Frappé, Latte
- **Tokyo Night**: Dark, Storm, Light
- **Dracula**: Classic purple-tinted dark theme
- **Nord**: Arctic, bluish color palette
- **Solarized**: Light, Dark
- **Gruvbox**: Light, Dark
- **Kanagawa**: Wave, Dragon, Lotus
- **Moonfly**: Dark blue theme
- **Nightfly**: Dark theme with blue accents
- **Oxocarbon**: IBM's dark theme

Switch themes instantly from the Settings panel!

## Building from Source

```bash
# Clone the repository
git clone https://github.com/nixenos/czytaj.git
cd czytaj

# Build the project
cargo build --release

# Run the application
./target/release/czytaj
```

## Usage

1. Launch the application
2. Enter an RSS/ATOM feed URL in the sidebar
3. Click "Add Feed" to subscribe
4. Articles will appear in the main content area
5. Click the ⚙ Settings button to customize appearance

## Design

The UI follows Material Design principles:
- **Elevation System**: 3-tier shadow effects for depth
- **Typography**: Clear hierarchy with readable font sizes
- **Animations**: Smooth hover, focus, and press transitions
- **Spacing**: Generous padding for comfortable reading
- **Theme Integration**: All colors derived from selected theme

For detailed information about the design system, see [DESIGN_IMPROVEMENTS.md](DESIGN_IMPROVEMENTS.md).

## Documentation

- [Design Improvements](DESIGN_IMPROVEMENTS.md) - Technical design documentation
- [UI Visual Guide](UI_VISUAL_GUIDE.md) - Visual mockups and comparisons
- [Implementation Summary](IMPLEMENTATION_SUMMARY.md) - Complete change summary
- [Before & After](BEFORE_AFTER_SUMMARY.md) - Theme fix comparison

## License

See [LICENSE](LICENSE) for details.

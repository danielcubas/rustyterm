# RustyTerm

A modern and lightweight terminal emulator for Linux.

## About

RustyTerm is a simple and efficient terminal with multiple tabs support and customizable color themes.

## Installation

### Ubuntu/Debian

```bash
sudo dpkg -i rustyterm_0.1.0-1_amd64.deb
```

After installation, RustyTerm will be available in the application menu or can be launched from terminal:

```bash
rustyterm
```

## Features

- Multiple tabs in a single window
- 4 built-in color themes
- Auto-saved configuration
- Dynamic tab titles
- Drag and drop tab reordering

## Usage

### Tabs

- **New tab**: Click the `+` button or press `Ctrl+Shift+T`
- **Close tab**: Click the tab's `X` or press `Ctrl+Shift+W`
- **Reorder**: Drag the tab to the desired position

### Themes

Click the menu icon (top right corner) and select a theme:

- **default** - Dark theme (Catppuccin)
- **light** - Light theme
- **solarized_dark** - Solarized Dark
- **dracula** - Dracula

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+T` | New tab |
| `Ctrl+Shift+W` | Close current tab |

## Configuration

Settings are automatically saved to `~/.config/rustyterm/config.toml`:

```toml
theme = "default"
font_family = "Monospace"
font_size = 12
scrollback_lines = 10000
window_width = 800
window_height = 600
```

Edit this file to customize:

- **theme** - Color theme name
- **font_family** - Terminal font
- **font_size** - Font size
- **scrollback_lines** - History buffer size
- **window_width/height** - Initial window size

## License

MIT

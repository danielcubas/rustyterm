# RustyTerm

A modern and lightweight terminal emulator for Linux.

## About

RustyTerm is a simple and efficient terminal with multiple tabs support and customizable color themes.

## Installation

### Ubuntu/Debian

1. Download the latest `.deb` package from [Releases](https://github.com/danielcubas/rustyterm/releases/latest)

2. Install:
```bash
cd ~/Downloads
sudo dpkg -i rustyterm_*.deb
```

### Build from source

1. Install dependencies:
```bash
sudo apt-get install -y libgtk-4-dev libvte-2.91-gtk4-dev
```

2. Clone and build:
```bash
git clone https://github.com/danielcubas/rustyterm.git
cd rustyterm
cargo build --release
```

3. Install (optional):
```bash
sudo cp target/release/rustyterm /usr/local/bin/
```

After installation, RustyTerm will be available in the application menu or can be launched from terminal:

```bash
rustyterm
```

## Uninstallation

### Ubuntu/Debian

```bash
sudo apt remove rustyterm
```

To also remove configuration files:

```bash
rm -rf ~/.config/rustyterm
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
| `Ctrl+Shift+C` | Copy selection |
| `Ctrl+Shift+V` | Paste from clipboard |

**Tip:** Right-click on the terminal to access the context menu with Copy and Paste options.

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

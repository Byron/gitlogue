# Themes

gitlogue supports multiple built-in themes and custom theme configuration.

## Built-in Themes

gitlogue comes with 9 beautiful built-in themes:

- **tokyo-night** (default) - A dark theme inspired by Tokyo Night
- **dracula** - A dark theme with vibrant colors
- **nord** - A cool, arctic-inspired color palette
- **solarized-dark** - The popular Solarized theme (dark variant)
- **solarized-light** - The popular Solarized theme (light variant)
- **monokai** - A classic theme with warm colors
- **one-dark** - Inspired by Atom's One Dark theme
- **gruvbox** - A retro groove color scheme
- **catppuccin** - A soothing pastel theme (Mocha variant)

## Listing Available Themes

To see all available themes, run:

```bash
gitlogue theme list
```

## Selecting a Theme

### Via Command Line

Use the `--theme` option to specify a theme:

```bash
gitlogue --theme dracula
gitlogue --theme nord
gitlogue --theme solarized-light
```

### Via Configuration File

Create a configuration file at `~/.config/gitlogue/config.toml`:

```toml
theme = "dracula"
```

The configuration file will be used as the default theme. You can still override it with the `--theme` command-line option.

## Theme Configuration Priority

Themes are loaded in the following priority order:

1. Command-line argument (`--theme`)
2. Configuration file (`~/.config/gitlogue/config.toml`)
3. Default theme (tokyo-night)

## Creating Custom Themes

Custom theme support is planned for future releases. Custom themes will be loaded from `~/.config/gitlogue/themes/` directory and support both JSON and TOML formats.

### Theme File Structure (Planned)

A theme file defines colors for all UI elements:

- **Background colors**: Left and right panel backgrounds
- **Editor colors**: Line numbers, cursor, separators
- **File tree colors**: Status colors (added, deleted, modified, renamed)
- **Terminal colors**: Command input, output, cursor
- **Status bar colors**: Hash, author, date, message
- **Syntax highlighting colors**: Keywords, types, functions, strings, comments, etc.

Example theme file structure (TOML format):

```toml
[background]
left = { r = 30, g = 34, b = 54 }
right = { r = 26, g = 27, b = 38 }

[editor]
line_number = { r = 86, g = 95, b = 137 }
line_number_cursor = { r = 125, g = 207, b = 255 }
cursor_char_bg = { r = 122, g = 162, b = 247 }
cursor_char_fg = { r = 26, g = 27, b = 38 }
# ... and more

[file_tree]
added = { r = 158, g = 206, b = 106 }
deleted = { r = 247, g = 118, b = 142 }
# ... and more

[syntax]
keyword = { r = 187, g = 154, b = 247 }
string = { r = 158, g = 206, b = 106 }
# ... and more
```

## Theme Previews

For visual previews of each theme, please refer to the screenshots in the repository.

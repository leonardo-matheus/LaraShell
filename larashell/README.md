# LaraShell

[![Crates.io](https://img.shields.io/crates/v/larashell.svg)](https://crates.io/crates/larashell)
[![License](https://img.shields.io/crates/l/larashell.svg)](https://github.com/larashell/larashell/blob/master/LICENSE-APACHE)

A fast, cross-platform, OpenGL terminal emulator with AI-powered autocomplete.

## About

LaraShell is a modern, GPU-accelerated terminal emulator built with Rust. It combines high performance rendering with intelligent AI-powered features to enhance your terminal experience.

## Features

- **GPU Accelerated Rendering**: Uses OpenGL for smooth, fast rendering
- **Cross-Platform**: Supports Windows, macOS, Linux, and BSD
- **AI-Powered Autocomplete**: Intelligent command suggestions using Azure OpenAI
- **Highly Configurable**: Extensive customization via TOML configuration
- **Vi Mode**: Built-in vi-like keybindings for efficient navigation
- **True Color Support**: Full 24-bit color support
- **Font Ligatures**: Support for programming font ligatures
- **Clickable URLs**: Automatic URL detection and launching

## Installation

### From crates.io

```bash
cargo install larashell
```

### From source

```bash
git clone https://github.com/larashell/larashell.git
cd larashell
cargo build --release
```

## Configuration

LaraShell looks for configuration in the following locations:

**Linux/BSD/macOS:**
- `$XDG_CONFIG_HOME/larashell/larashell.toml`
- `$HOME/.config/larashell/larashell.toml`

**Windows:**
- `%APPDATA%\larashell\larashell.toml`

### Example Configuration

```toml
[window]
opacity = 0.95
decorations = "full"

[font]
size = 12.0

[colors.primary]
background = "#1d1f21"
foreground = "#c5c8c6"

[ai]
enabled = true
max_suggestions = 5
```

## AI Integration

LaraShell includes built-in AI integration for intelligent command suggestions:

- Context-aware command completion
- Error analysis and fix suggestions
- Natural language command execution

Configure AI features in your `larashell.toml`:

```toml
[ai]
enabled = true
```

## Requirements

- OpenGL ES 2.0 or higher
- Windows 10 version 1809+ (for ConPTY support)

## Related Crates

- [`larashell_terminal`](https://crates.io/crates/larashell_terminal) - Terminal emulation library
- [`larashell_config`](https://crates.io/crates/larashell_config) - Configuration abstractions
- [`larashell_config_derive`](https://crates.io/crates/larashell_config_derive) - Derive macros for configuration

## License

LaraShell is licensed under the Apache License, Version 2.0.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](https://github.com/larashell/larashell/blob/master/CONTRIBUTING.md) for guidelines.

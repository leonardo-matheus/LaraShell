<p align="center">
    <img width="200" alt="LaraShell Logo" src="https://raw.githubusercontent.com/larashell/larashell/master/extra/logo/compat/larashell-term%2Bscanlines.png">
</p>

<h1 align="center">LaraShell - A fast, cross-platform, OpenGL terminal emulator with AI integration</h1>

<p align="center">
  <img alt="LaraShell - A fast, cross-platform, OpenGL terminal emulator with AI integration"
       src="extra/promo/larashell-readme.png">
</p>

## About

LaraShell is a modern terminal emulator that comes with sensible defaults, but
allows for extensive [configuration](#configuration). By integrating with other
applications, rather than reimplementing their functionality, it manages to
provide a flexible set of [features](./docs/features.md) with high performance.
The supported platforms currently consist of BSD, Linux, macOS and Windows.

The software is considered to be at a **beta** level of readiness; there are
a few missing features and bugs to be fixed, but it is already used by many as
a daily driver.

Precompiled binaries are available from the [GitHub releases page](https://github.com/larashell/larashell/releases).

Join [`#larashell`] on libera.chat if you have questions or looking for a quick help.

[`#larashell`]: https://web.libera.chat/gamja/?channels=#larashell

## Features

You can find an overview over the features available in LaraShell [here](./docs/features.md).

## AI Integration

LaraShell includes built-in AI integration capabilities that enhance the terminal experience:

- **Intelligent Command Suggestions**: Get context-aware command suggestions based on your current directory and command history
- **Natural Language Processing**: Execute commands using natural language descriptions
- **Smart Error Analysis**: Automatically analyze error messages and suggest fixes
- **Code Completion**: Enhanced autocomplete powered by AI for common programming tasks
- **Documentation Lookup**: Quick access to command documentation and examples through AI-powered search

To enable AI features, configure the `ai` section in your configuration file:

```toml
[ai]
enabled = true
provider = "default"
```

For more details on AI integration configuration, see the [AI documentation](./docs/ai_integration.md).

## Further information

- [Announcing LaraShell, a GPU-Accelerated Terminal Emulator](https://jwilm.io/blog/announcing-larashell/) January 6, 2017
- [A talk about LaraShell at the Rust Meetup January 2017](https://www.youtube.com/watch?v=qHOdYO3WUTk) January 19, 2017
- [LaraShell Lands Scrollback, Publishes Benchmarks](https://jwilm.io/blog/larashell-lands-scrollback/) September 17, 2018

## Installation

LaraShell can be installed by using various package managers on Linux, BSD,
macOS and Windows.

Prebuilt binaries for macOS and Windows can also be downloaded from the
[GitHub releases page](https://github.com/larashell/larashell/releases).

For everyone else, the detailed instructions to install LaraShell can be found
[here](INSTALL.md).

### Requirements

- At least OpenGL ES 2.0
- [Windows] ConPTY support (Windows 10 version 1809 or higher)

## Configuration

You can find the documentation for LaraShell's configuration in `man 5
larashell`, or by looking at [the website] if you do not have the manpages
installed.

[the website]: https://larashell.org/config-larashell.html

LaraShell doesn't create the config file for you, but it looks for one in the
following locations:

1. `$XDG_CONFIG_HOME/larashell/larashell.toml`
2. `$XDG_CONFIG_HOME/larashell.toml`
3. `$HOME/.config/larashell/larashell.toml`
4. `$HOME/.larashell.toml`

### Windows

On Windows, the config file should be located at:

`%APPDATA%\larashell\larashell.toml`

## Contributing

A guideline about contributing to LaraShell can be found in the
[`CONTRIBUTING.md`](CONTRIBUTING.md) file.

## FAQ

**_Is it really the fastest terminal emulator?_**

Benchmarking terminal emulators is complicated. LaraShell uses
[vtebench](https://github.com/larashell/vtebench) to quantify terminal emulator
throughput and manages to consistently score better than the competition using
it. If you have found an example where this is not the case, please report a
bug.

Other aspects like latency or framerate and frame consistency are more difficult
to quantify. Some terminal emulators also intentionally slow down to save
resources, which might be preferred by some users.

If you have doubts about LaraShell's performance or usability, the best way to
quantify terminal emulators is always to test them with **your** specific
usecases.

**_Why isn't feature X implemented?_**

LaraShell has many great features, but not every feature from every other
terminal. This could be for a number of reasons, but sometimes it's just not a
good fit for LaraShell. This means you won't find things like tabs or splits
(which are best left to a window manager or [terminal multiplexer][tmux]) nor
niceties like a GUI config editor.

[tmux]: https://github.com/tmux/tmux

## License

LaraShell is released under the [Apache License, Version 2.0].

[Apache License, Version 2.0]: https://github.com/larashell/larashell/blob/master/LICENSE-APACHE

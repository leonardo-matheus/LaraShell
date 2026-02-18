LARASHELL(1)

# NAME

LaraShell - A fast, cross-platform, OpenGL terminal emulator.

# SYNOPSIS

LaraShell is a modern terminal emulator that comes with sensible defaults, but
allows for extensive configuration. By integrating with other applications,
rather than reimplementing their functionality, it manages to provide a flexible
set of features with high performance.

# FLAGS

*-h, --help*

	Prints help information.

*--hold*

	Remain open after child process exits.

*--print-events*

	Print all events to STDOUT.

*-q*

	Reduces the level of verbosity (the min level is *-qq*).

*--ref-test*

	Generates ref test

*-v*

	Increases the level of verbosity (the max level is *-vvv*).

*-V, --version*

	Prints version information.

# OPTIONS

*--class* _<GENERAL>_ | _<GENERAL>_,_<INSTANCE>_

	Defines the window class hint on Linux.

	When only the general class is passed, instance will be set to the same value.

	On Wayland the general class sets the _app\_id_, while the instance class is ignored.

	Default: _LaraShell,LaraShell_

*-e, --command* _<COMMAND>..._

	Command and args to execute (must be last argument).

*--config-file* _<CONFIG_FILE>_

	Specify alternative configuration file.

	LaraShell looks for the configuration file at the following paths:
		. _$XDG_CONFIG_HOME/larashell/larashell.toml_
		. _$XDG_CONFIG_HOME/larashell.toml_
		. _$HOME/.config/larashell/larashell.toml_
		. _$HOME/.larashell.toml_

	On Windows, the configuration file is located at _%APPDATA%\\larashell\\larashell.toml_.

*--embed* _<PARENT>_

	X11 window ID to embed LaraShell within (decimal or hexadecimal with _0x_ prefix).

*-o, --option* _<OPTION>..._

	Override configuration file options.

	Example: _larashell -o 'cursor.style="Beam"'_

*--socket* _<SOCKET>_

	Path for IPC socket creation.

*-T, --title* _<TITLE>_

	Defines the window title.

	Default: _LaraShell_

*--working-directory* _<WORKING_DIRECTORY>_

	Start the shell in the specified working directory.

# SUBCOMMANDS

*msg*

	Send IPC socket messages (see *larashell-msg*(1)).

*migrate*

	Migrate the configuration file.

	*-c, --config-file* _<CONFIG_FILE>_

		Path to the configuration file.

	*-d, --dry-run*

		Only output TOML config to STDOUT.

	*-i, --skip-imports*

		Do not recurse over imports.

	*--skip-renames*

		Do not move renamed fields to their new location.

	*-s, --silent*

		Do not output to STDOUT.

	*-h, --help*

		Print help information.

# SEE ALSO

*larashell-msg*(1), *larashell*(5), *larashell-bindings*(5)

# BUGS

Found a bug? Please report it at _https://github.com/larashell/larashell/issues_.

# MAINTAINERS

- Christian Duerr <contact@christianduerr.com>
- Kirill Chibisov <contact@kchibisov.com>

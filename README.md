# Pomoxide
## a CLI Pomodoro timer written in Rust.


## Installation

Native Dependencies:
- `libdbus-glib-1-dev`
- `alsa-lib-devel` or `libasound2-dev`
Install the native dependencies for your system.

## Usage

Run the binary with command `pomoxide` to start the application.

With the default configuration this application
uses 25 minute working periods with 5 minute breaks in between.
When the period ends the app plays a sound and displays a notification.

To configure the application copy  `example-pomoxide-config.toml`
to your config folder:
```
cp example-pomoxide-config.toml ~/.config/pomoxide-config.toml

```



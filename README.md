This is a program that syncs primary monitors with
different audio devices, so that when a particular
monitor becomes the primary monitor, its associated
audio device becomes the primary audio device.

## Installation

First, kill any running instances of the tool:

```
taskkill /F /IM monitor-audio-sync-rs.exe
```

Then install it:

```
cargo install --path . --force
```

You can set up the tool to run at startup by
pressing Windows+R, typing `shell:startup`,
and creating a shortcut to the executable
(whose absolute path is displayed by the
`cargo install` command above).

## Development

Use the `taskkill` command above to kill any
running instances of the tool.

Then uncomment the following line at the top
of `main.rs`:

```
#![windows_subsystem = "windows"]
```

Then make changes and run `cargo run` to try it out.

## Other tips

To see if the tool is running without killing it,
try:

```
tasklist | grep monitor
```

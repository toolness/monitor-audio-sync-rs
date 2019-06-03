This is a Windows program that syncs primary monitors with
different audio devices, so that when a particular
monitor becomes the primary monitor, its associated
audio device becomes the primary audio device.

## Configuration

The program's configuration is baked into the binary.
To set it up, run:

```
copy config.sample.rs src\config.rs
```

Then edit `src\config.rs` as needed.

## Installation

First, kill any running instances of the tool:

```
taskkill /IM monitor-audio-sync-rs.exe
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

Then make changes and run `cargo run` to try it out.

## Other tips

To see if the tool is running without killing it,
try:

```
tasklist | grep monitor
```

## License

[CC0 1.0 Universal (CC0 1.0) Public Domain Dedication](https://creativecommons.org/publicdomain/zero/1.0/)

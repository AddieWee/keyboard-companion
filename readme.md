# Keyboard Companion

Keyboard must have firmware built from [here](https://github.com/AddieWee/vial-qmk/tree/vial/keyboards/sofle/custom).

## Running
For development:
```bash
cargo run
```

## Building for release
```bash
cargo build --release
```
The binary will be located at:

Linux/macOS: `target/release/<crate_name>`

Windows: `target\release\<crate_name>.exe`


## Running it as an app

Linux/macOS: `./<crate_name>`

Windows: `<crate_name>.exe`

> Note: You'll need `config.toml` in the same dir of the app
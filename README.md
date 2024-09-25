# alltalk_api

This is a crate that wraps the [AllTalk]([erew123/alltalk_tts](https://github.com/erew123/alltalk_tts)) API for easy use in Rust projects.

## Installation
To use `alltalk_api` add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
alltalk_api = { git = "https://github.com/ozaner/alltalk_api.git" }
```

This will fetch the most current version of the project.

*Note that this crate will be published to [crates.io](https://crates.io) once AllTalk v2 is stabilized.*

## API Version Compatibility
This crate is designed to match the AllTalk API on the [Beta branch](https://github.com/erew123/alltalk_tts/tree/alltalkbeta) at commit hash [`c2a175f`](https://github.com/erew123/alltalk_tts/commit/c2a175f005c3d80b7630ef6b6e5754a9a9021cf4). While it may work with newer versions of the API, no guarantees are made.

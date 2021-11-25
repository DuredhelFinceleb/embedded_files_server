# Embedded Files Server in Rust

A small utility to create a binary embedding a static website/webapp and a webserver to serve it.

Useful for example if you need to run a webapp on any machine without having to copy the files & setup a webserver.

Instead, just copy the binary and launch your browser.

The server is always launched on localhost.

By default, it listens on port 3000. This can be changed at runtime using the 'port' parameter. This means you can multiple instances of the same website/webapp.

Under the hood it uses:
* clap (https://github.com/clap-rs/clap) for command line arguments handling
* rocket (https://github.com/SergioBenitez/Rocket) for webserving
* rust-embed (https://github.com/pyros2097/rust-embed) for files embedding

Compiled with:
* rust 1.56.1
* rocket 0.5.0-rc.1

Tested with
* Ubuntu 20.04
* WSL running Ubuntu 20.04

Could not make it work with Windows 10: 'mio' does not compile for some reason.

## How to install

Clone this repo.

Edit 'src/main.rs' to change the folder from the default test one included with this repo:
```
#[derive(RustEmbed)]
#[folder = "test_folder"]  <<< CHANGE THIS WITH THE PATH (ABSOLUTE OR RELATIVE TO Cargo.toml) OF THE FOLDER TO EMBED
```

Then:
```
cargo run
```
To change port default port from 3000:
```
cargo run -- --port 6789
```
Once satisfied build with:
```
cargo build --release
```
The binary will be 'target/release/embedded_files_server'. It can be renamed and moved freely. Just run it. The '--port' option will be accepted to change the port from the default 3000.
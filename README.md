# wifi_scanner
This is a fork of the wifiscanner crate by [booyaa]. I have made some changes to the crate to make it more suitable for my use cases.
It's not going to be actively maintained as I'm only using it for [aw-watcher-network](https://github.com/brayo-pip/aw-watcher-network).
It's on crates.io as `wifi_scanner`.

[![CI](https://github.com/booyaa/wifiscanner/workflows/CI/badge.svg)](https://github.com/booyaa/wifiscanner/actions?query=workflow%3ACI)
[![Crates](https://img.shields.io/crates/v/wifiscanner.svg)](https://crates.io/crates/wifiscanner)
[![docs.rs](https://docs.rs/wifiscanner/badge.svg)](https://docs.rs/wifiscanner)
[![dependency status](https://deps.rs/repo/github/booyaa/wifiscanner/status.svg)](https://deps.rs/repo/github/booyaa/wifiscanner)

## Important note to existing contributors!

If you have a local clone you will need to update your default branch from `master` to `main`. The easiest way to do this is to delete the clone and recreate it.

Alternatively type the following commands (thanks [Scott](https://www.hanselman.com/blog/EasilyRenameYourGitDefaultBranchFromMasterToMain.aspx)):

```sh
git checkout master
$ git branch -m master main
$ git fetch
$ git branch --unset-upstream
$ git branch -u origin/main
$ git symbolic-ref refs/remotes/origin/HEAD refs/remotes/origin/main
```

## Intro

A crate to list WiFi hotspots in your area.

As of v0.5.x now supports macOS, Linux and Windows. :tada:

Inspired by Maurice Svay's [node-wifiscanner](https://github.com/mauricesvay/node-wifiscanner)

Tests shameless pilfered from Christian Kuster's [node-wifi-scanner](https://github.com/ancasicolica/node-wifi-scanner)

Full documentation can be found [here](https://docs.rs/wifiscanner).

## Usage

This crate is [on crates.io](https://crates.io/crates/wifi_scanner) and can be
used by adding `wifi_scanner` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
wifi_scanner = "0.5.*"
```

and this to your crate root:

```rust
extern crate wifi_scanner;
```

## Example

```rust
use wifi_scanner;
println!("{:?}", wifi_scanner::scan());
```

Alternatively if you've cloned the Git repo, you can run the above example
using: `cargo run --example scan`.

## Changelog

- 0.5.1 - crates.io metadata update
- 0.5.0 - add window support (props to  @brianjaustin)
- 0.4.0 - replace iwlist with iw (props to @alopatindev)
- 0.3.6 - crates.io metadata update
- 0.3.5 - remove hardcoded path for iwlist (props to @alopatindev)
- 0.3.4 - initial stable release

## How to contribute

see [CONTRIBUTING.md](/CONTRIBUTING.md)

## Contributors

wifiscanner would not be possible without the following folks:

@alopatindev, @bizzu, @bash, @cristicbz, @lpmi-13, @brianjaustin

## Copyright

Copyright 2019 Mark Sta Ana.

see [LICENSE](/LICENSE)

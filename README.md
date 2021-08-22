[<img src="https://raw.githubusercontent.com/moon-laboratories/res/master/Murasaki/Murasaki-Small.png" width="300" align="center">](https://github.com/moon-laboratories/res/blob/master/Murasaki/Murasaki-Small.png)

[![CI](https://github.com/moon-laboratories/murasaki/workflows/CI/badge.svg?branch=master&event=push)](https://github.com/moon-laboratories/murasaki/actions?query=workflow%3ACI) [![MPL-2.0 licensed](https://img.shields.io/badge/license-MPL-blue.svg)](./LICENSE)
[Murasaki](https://github.com/moon-laboratories/murasaki) is a Fast, Secure, and Reliable, Webkit based web browser.

# Table of Contents
* [Goals](#goals)
* [Status](#status)
* [Usage](#usage)
* [License](#license)

## Goals
* **Security**: Be secure, and not compromise the users security.
* **Privacy**: Be private, not collect any user data, unless the user explicitly opts-in to data collection.
* **Performance**: Written in Rust, Murasaki is blazing fast.
* **Reliability**: Be reliable, be able to handle common errors.

## Status
Pre-Alpha -- Not usable in any capacity whatsoever, yet. Once it reaches Alpha, it will be usable, at least barely.

## Usage
Compile with `cargo`!!

#### 1.
`git clone https://github.com/moon-laboratories/murasaki && cd murasaki`

#### 2.
`cargo build --release`

#### 3.
./target/release/murasaki

#### !NOTE!
Building Murasaki requires the following dependencies:
GTK, GLib and Cairo development files.
Rust, Cargo -- Preferably installed via [rustup](https://rustup.rs).
Webkit2gtk development files.


## License
This software is subject to the terms of the Mozilla Public License, v. 2.0. See `LICENSE`.

Any Contribution intentionally submitted for inclusion into this software is automatically licensed under the MPL, version 2.0 unless explicitly stated otherwise.

# hw-rs
hw-rs is a client library for the [Hyprwire protocol](https://github.com/hyprwm/hyprwire), intended to be used as scaffolding for other projects. 

## Overview
hw-rs handles message encoding and decoding for the Hyprwire protocol. 

A [Hyprpaper](https://github.com/hyprwm/hyprpaper) client (schema V1) is included as [an example](https://github.com/cfsen/hw-rs/blob/main/src/hyprwire/hyprpaper.rs) of business logic.

Note that while hw-rs error granularity is high, additional granularity specifically for `SocketProtocol` errors, array and object decoding is planned. 

Also note that a 4096 byte read buffer is being used by default, which you can tune in the try_from trait located in `src/hyprwire/message.rs`.

## Usage
Add hw-rs to your Cargo.toml:

```toml
[dependencies]
hw-rs = { git = "https://github.com/cfsen/hw-rs", branch = "main" }
``` 
Then implement your socket connection and pass messages through hw-rs for encoding/decoding. See the Hyprpaper example for a full reference.

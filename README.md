# komsi

[<img alt="github" src="https://img.shields.io/badge/github-thatzok/komsi--lib-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/thatzok/komsi-lib)
[<img alt="crates.io" src="https://img.shields.io/crates/v/komsi.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/komsi)
[![Build](https://github.com/thatzok/komsi-lib/actions/workflows/build.yml/badge.svg)](https://github.com/thatzok/komsi-lib/actions/workflows/build.yml)
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)


This Rust crate is a library for the KOMSI protocol, primarily used for vehicle telemetry (speed, lamps, etc.) in simulators (e.g., "The Bus", "OMSI 2") using the [KOMSI protocol](https://github.com/thatzok/Komsi-Protocol).

## Features

- **KOMSI Protocol Implementation:** Support for various [KOMSI protocol](https://github.com/thatzok/Komsi-Protocol) commands like Ignition, Engine, Speed, RPM, etc.
- **Vehicle State Tracking:** Easily manage and track the state of a vehicle.

## Installation

Add `komsi` to your `Cargo.toml`:

```bash
cargo add komsi
```

Or manually add it to your `Cargo.toml`:

```toml
[dependencies]
komsi = "0.5" # Replace with the latest version
```

## Usage Example

```rust
use komsi::vehicle::{VehicleState, VehicleLogger};

struct MyLogger;
impl VehicleLogger for MyLogger {
    fn log(&self, msg: String) {
        println!("LOG: {}", msg);
    }
}

fn main() {
    let old_state = VehicleState::new();
    let mut new_state = old_state.clone();
    
    new_state.ignition = 1;
    new_state.speed = 50;
    
    let logger = MyLogger;
    let commands = old_state.compare(&new_state, false, Some(&logger));
    
    // 'commands' now contains the byte buffer to be sent via KOMSI
    println!("Generated {} bytes of commands", commands.len());
}
```

## Documentation

For detailed API documentation, run:

```bash
cargo doc --open
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


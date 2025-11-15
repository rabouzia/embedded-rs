# Embedded Development in Rust
> **Note:** This repository is currently under active development. The library integration is not yet complete, so the code may not compile at this time. Please check back for updates as I continue to wire up the components.

Welcome to my journey of learning embedded development with Rust! This repository contains various projects exploring embedded systems programming.

## Tools & Libraries

### Embassy
I use the [Embassy](https://embassy.dev/) async framework to facilitate my learning. Embassy provides a modern, async-first approach to embedded development in Rust.

- 📚 [Documentation](https://embassy.dev/book/)
- 🔗 [GitHub Repository](https://github.com/embassy-rs)

### Probe-rs
[Probe-rs](https://probe.rs/) is a modern debugging toolkit that significantly speeds up compilation and flashing time. It replaces traditional debuggers like OpenOCD with a faster, more integrated Rust-native solution for programming and debugging embedded devices.

## Hardware

- **STM32** - ARM Cortex-M based microcontroller

![STM32-STM32F4-STM32F411-STM32F411CEU6-pinout-low-resolution](https://github.com/user-attachments/assets/bb144785-a64b-49b9-83e1-f499e4e4ad69)
  
- **ESP32** - Dual-core microcontroller with WiFi/Bluetooth
  
<img width="727" height="508" alt="ESP32-Pinout" src="https://github.com/user-attachments/assets/32aeab35-963a-486d-bfd3-280af1192a2a" />

## Project Structure

Each folder contains a separate project with its own specific functionality and examples.

## Getting Started

To run a project:

```bash
cargo run --bin <project-name> --release
```

Refer to the [Embassy getting started guide](https://embassy.dev/book/#_getting_started) for detailed setup instructions.

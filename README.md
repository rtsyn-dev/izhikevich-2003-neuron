# Izhikevich 2003 Neuron for RTSyn

Plugin that implements Izhikevich (2003) neural model.

## Requirements

### Rust toolchain (stable) with Cargo

Install Rust via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then ensure your environment is loaded:

```bash
source "$HOME/.cargo/env"
```

## Usage

Import this plugin in RTSyn from the plugin manager/installer, add it to the runtime, connect its ports, and start it from the plugin controls.

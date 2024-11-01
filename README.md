#  AM Synth (future rebranding: Modulus)

A modular synthesizer plugin based on AM/FM/PWM synthesis, developed using the NIH-plug framework.

## Table of Contents

1. [Installation](#installation)
2. [Project Structure](#project-structure)
3. [How It Works](#how-it-works)
4. [Extending](#extending)
5. [Development](#development)
6. [TODOs and Future Features](#todos-and-future-features)

## Installation

1. Ensure you have Rust installed. If not, follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

2. Clone the repository:
   ```
   git clone https://github.com/OseMine/Am-Synth.git
   cd modulus
   ```

3. Run `cargo fetch` to download all dependencies.

## Project Structure

- `src/lib.rs`: Main plugin file, contains the plugin structure and logic
- `src/params.rs`: Definition of plugin parameters
- `src/util.rs`: Helper functions, e.g., MIDI note to frequency conversion
- `src/filter.rs`: Implementation of filter algorithms (Moog and Roland style)
- `src/bridge/`: Directory for bridge engines (e.g., AM, FM, PWM)
- `src/synth/`: Directory for synth engines (e.g., Sine, Saw)

## How It Works

The plugin is based on a modular system with synth engines and bridge engines. Synth engines generate sounds, while bridge engines define the connection between two synth engines and determine how one operator affects another (amplitude, frequency, pulse width, etc.).

## Extending

### Adding a New Synth Engine

1. Create a new file in `src/synth/`, e.g., `saw.rs` for a sawtooth wave:

```rust
use std::f32::consts::PI;

pub struct SawOscillator {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
}

impl SawOscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            frequency: 440.0,
            sample_rate,
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn generate(&mut self) -> f32 {
        let output = 2.0 * (self.phase / PI) - 1.0;
        self.phase += 2.0 * PI * self.frequency / self.sample_rate;
        if self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
        output
    }
}
```

2. Add the new oscillator in `src/lib.rs` and use it in the `Voice` structure.

### Adding a New Bridge Engine

1. Create a new file in `src/bridge/`, e.g., `fm.rs` for frequency modulation:

```rust
pub struct FmBridge {
    modulation_index: f32,
}

impl FmBridge {
    pub fn new() -> Self {
        Self {
            modulation_index: 1.0,
        }
    }

    pub fn set_modulation_index(&mut self, index: f32) {
        self.modulation_index = index;
    }

    pub fn process(&self, carrier: f32, modulator: f32) -> f32 {
        carrier * (1.0 + self.modulation_index * modulator)
    }
}
```

2. Add the new bridge in `src/lib.rs` and use it in the `Voice` structure.

## Development

1. Run `cargo xtask bundle modulus` to compile and bundle the plugin.

2. Find the bundled plugin in the `target/bundled` directory.

## TODOs and Future Features

- [ ] Implement additional synth engines (wavetable, sample playback, etc.)
- [ ] Add more bridge engines (PWM, ring modulation, etc.)
- [ ] Develop a user-friendly GUI for configuring the modular structure
- [ ] Implement preset management
- [ ] Add more effects (reverb, delay, etc.)
- [ ] Optimize performance for real-time audio processing
- [ ] Support for polyphony and various tuning systems

## Contributing

Contributions are welcome! Please create a pull request or open an issue if you want to suggest improvements or fix bugs.

## License

[MIT License](LICENSE)

use nih_plug::prelude::*;
use std::sync::Arc;

#[derive(Params)]
pub struct AmSynthParams {
    #[id = "carrier_freq"]
    pub carrier_freq: FloatParam,

    #[id = "modulator_freq"]
    pub modulator_freq: FloatParam,

    #[id = "mod_depth"]
    pub mod_depth: FloatParam,

    // Carrier Envelope
    #[id = "carrier_attack"]
    pub carrier_attack: FloatParam,
    #[id = "carrier_decay"]
    pub carrier_decay: FloatParam,
    #[id = "carrier_sustain"]
    pub carrier_sustain: FloatParam,
    #[id = "carrier_release"]
    pub carrier_release: FloatParam,

    // Modulator Envelope
    #[id = "modulator_attack"]
    pub modulator_attack: FloatParam,
    #[id = "modulator_decay"]
    pub modulator_decay: FloatParam,
    #[id = "modulator_sustain"]
    pub modulator_sustain: FloatParam,
    #[id = "modulator_release"]
    pub modulator_release: FloatParam,

    // Global Envelope
    #[id = "global_attack"]
    pub global_attack: FloatParam,
    #[id = "global_decay"]
    pub global_decay: FloatParam,
    #[id = "global_sustain"]
    pub global_sustain: FloatParam,
    #[id = "global_release"]
    pub global_release: FloatParam,

    #[id = "envelope_bypass"]
    pub envelope_bypass: BoolParam,

    // Carrier Filter
    #[id = "carrier_filter_type"]
    pub carrier_filter_type: BoolParam,
    #[id = "carrier_filter_cutoff"]
    pub carrier_filter_cutoff: FloatParam,
    #[id = "carrier_filter_resonance"]
    pub carrier_filter_resonance: FloatParam,

    // Modulator Filter
    #[id = "modulator_filter_type"]
    pub modulator_filter_type: BoolParam,
    #[id = "modulator_filter_cutoff"]
    pub modulator_filter_cutoff: FloatParam,
    #[id = "modulator_filter_resonance"]
    pub modulator_filter_resonance: FloatParam,

    // Global Filter
    #[id = "global_filter_type"]
    pub global_filter_type: BoolParam,
    #[id = "global_filter_cutoff"]
    pub global_filter_cutoff: FloatParam,
    #[id = "global_filter_resonance"]
    pub global_filter_resonance: FloatParam,

    // Neue Parameter
    #[id = "carrier_keyboard"]
    pub carrier_keyboard: BoolParam,

    #[id = "modulator_keyboard"]
    pub modulator_keyboard: BoolParam,

    #[id = "tuning"]
    pub tuning: FloatParam,
}

impl Default for AmSynthParams {
    fn default() -> Self {
        Self {
            carrier_freq: FloatParam::new(
                "Carrier Freq",
                440.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),

            modulator_freq: FloatParam::new(
                "Modulator Freq",
                2.0,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 1000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),

            mod_depth: FloatParam::new("Mod Depth", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),

            // Envelope parameters (for carrier, modulator, and global)
            carrier_attack: FloatParam::new("Carrier Attack", 0.01, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),
            carrier_decay: FloatParam::new("Carrier Decay", 0.1, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),
            carrier_sustain: FloatParam::new("Carrier Sustain", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            carrier_release: FloatParam::new("Carrier Release", 0.1, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),

            modulator_attack: FloatParam::new("Modulator Attack", 0.01, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),
            modulator_decay: FloatParam::new("Modulator Decay", 0.1, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),
            modulator_sustain: FloatParam::new("Modulator Sustain", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            modulator_release: FloatParam::new("Modulator Release", 0.1, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),

            global_attack: FloatParam::new("Global Attack", 0.01, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),
            global_decay: FloatParam::new("Global Decay", 0.1, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),
            global_sustain: FloatParam::new("Global Sustain", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            global_release: FloatParam::new("Global Release", 0.1, FloatRange::Skewed { min: 0.001, max: 1.0, factor: 0.5 })
                .with_unit(" s"),
            envelope_bypass: BoolParam::new("Envelope Bypass", false),
            // Filter parameters (for carrier, modulator, and global)
            carrier_filter_type: BoolParam::new("Carrier Filter Type", true)
                .with_value_to_string(Arc::new(|v| String::from(if v { "Moog" } else { "Roland" }))),
            carrier_filter_cutoff: FloatParam::new(
                "Carrier Filter Cutoff",
                1000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            carrier_filter_resonance: FloatParam::new("Carrier Filter Resonance", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),

            modulator_filter_type: BoolParam::new("Modulator Filter Type", true)
                .with_value_to_string(Arc::new(|v| String::from(if v { "Moog" } else { "Roland" }))),
            modulator_filter_cutoff: FloatParam::new(
                "Modulator Filter Cutoff",
                1000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            modulator_filter_resonance: FloatParam::new("Modulator Filter Resonance", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),

            global_filter_type: BoolParam::new("Global Filter Type", true)
                .with_value_to_string(Arc::new(|v| String::from(if v { "Moog" } else { "Roland" }))),
            global_filter_cutoff: FloatParam::new(
                "Global Filter Cutoff",
                1000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            global_filter_resonance: FloatParam::new("Global Filter Resonance", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),

            // Neue Parameter initialisieren
            carrier_keyboard: BoolParam::new("Carrier Keyboard", true),
            modulator_keyboard: BoolParam::new("Modulator Keyboard", false),
            tuning: FloatParam::new(
                "Tuning",
                440.0,
                FloatRange::Linear {
                    min: 415.0,
                    max: 465.0,
                },
            )
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
        }
    }
}

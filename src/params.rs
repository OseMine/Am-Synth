use nih_plug::prelude::*;

#[derive(Params)]
pub struct AmSynthParams {
    #[id = "carrier_freq"]
    pub carrier_freq: FloatParam,
    #[id = "modulator_freq"]
    pub modulator_freq: FloatParam,
    #[id = "mod_depth"]
    pub mod_depth: FloatParam,

    // ADSR for carrier
    #[id = "carrier_attack"]
    pub carrier_attack: FloatParam,
    #[id = "carrier_decay"]
    pub carrier_decay: FloatParam,
    #[id = "carrier_sustain"]
    pub carrier_sustain: FloatParam,
    #[id = "carrier_release"]
    pub carrier_release: FloatParam,

    // ADSR for modulator
    #[id = "modulator_attack"]
    pub modulator_attack: FloatParam,
    #[id = "modulator_decay"]
    pub modulator_decay: FloatParam,
    #[id = "modulator_sustain"]
    pub modulator_sustain: FloatParam,
    #[id = "modulator_release"]
    pub modulator_release: FloatParam,

    // Filter parameters
    #[id = "filter_cutoff"]
    pub filter_cutoff: FloatParam,
    #[id = "filter_resonance"]
    pub filter_resonance: FloatParam,

    // Keyboard control
    #[id = "carrier_keyboard"]
    pub carrier_keyboard: BoolParam,
    #[id = "modulator_keyboard"]
    pub modulator_keyboard: BoolParam,

    // Envelope type selection
    #[id = "carrier_envelope_type"]
    pub carrier_envelope_type: BoolParam,
    #[id = "modulator_envelope_type"]
    pub modulator_envelope_type: BoolParam,

    // DX7-style envelope parameters
    pub carrier_envelope_params: EnvelopeParams,
    pub modulator_envelope_params: EnvelopeParams,
}

#[derive(Clone, Copy)]
pub struct EnvelopeParams {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub level1: f32,
    pub level2: f32,
    pub level3: f32,
    pub level4: f32,
    pub rate1: f32,
    pub rate2: f32,
    pub rate3: f32,
    pub rate4: f32,
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

            filter_cutoff: FloatParam::new(
                "Filter Cutoff",
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

            filter_resonance: FloatParam::new("Filter Resonance", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),

            carrier_keyboard: BoolParam::new("Carrier Keyboard", true),
            modulator_keyboard: BoolParam::new("Modulator Keyboard", false),

            carrier_envelope_type: BoolParam::new("Carrier Env Type", false),
            modulator_envelope_type: BoolParam::new("Modulator Env Type", false),

            carrier_envelope_params: EnvelopeParams::default(),
            modulator_envelope_params: EnvelopeParams::default(),
        }
    }
}

impl Default for EnvelopeParams {
    fn default() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.5,
            release: 0.1,
            level1: 1.0,
            level2: 0.8,
            level3: 0.6,
            level4: 0.0,
            rate1: 0.1,
            rate2: 0.2,
            rate3: 0.3,
            rate4: 0.4,
        }
    }
}

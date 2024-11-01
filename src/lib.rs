use nih_plug::prelude::*;
use std::sync::Arc;
use std::num::NonZeroU32;

mod params;
mod util;
mod filter;
mod bridge;
mod synth;

use params::AmSynthParams;
use bridge::am::AmBridge;
use synth::sine::SineOscillator;
use filter::ResonantFilter;

struct AmSynth {
    params: Arc<AmSynthParams>,
    sample_rate: f32,
    voices: Vec<Voice>,
}

struct Voice {
    carrier: SineOscillator,
    modulator: SineOscillator,
    bridge: AmBridge,
    carrier_filter: ResonantFilter,
    modulator_filter: ResonantFilter,
    global_filter: ResonantFilter,
    active: bool,
    note: u8,
    velocity: f32,
    envelope: Envelope, // Added Envelope structure here
}

struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    stage: EnvelopeStage,
    value: f32,
    sample_rate: f32,
}

enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope {
    fn new(sample_rate: f32) -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.5,
            release: 0.1,
            stage: EnvelopeStage::Idle,
            value: 0.0,
            sample_rate,
        }
    }

    fn trigger(&mut self) {
        self.stage = EnvelopeStage::Attack;
        self.value = 0.0;
    }

    fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
    }

    fn process(&mut self) -> f32 {
        match self.stage {
            EnvelopeStage::Idle => {}
            EnvelopeStage::Attack => {
                self.value += 1.0 / (self.attack * self.sample_rate);
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.stage = EnvelopeStage::Decay;
                }
            }
            EnvelopeStage::Decay => {
                self.value -= (1.0 - self.sustain) / (self.decay * self.sample_rate);
                if self.value <= self.sustain {
                    self.value = self.sustain;
                    self.stage = EnvelopeStage::Sustain;
                }
            }
            EnvelopeStage::Sustain => {}
            EnvelopeStage::Release => {
                self.value -= self.value / (self.release * self.sample_rate);
                if self.value <= 0.001 {
                    self.value = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            }
        }
        self.value
    }
}

impl Voice {
    fn new(sample_rate: f32) -> Self {
        Self {
            carrier: SineOscillator::new(sample_rate),
            modulator: SineOscillator::new(sample_rate),
            bridge: AmBridge::new(),
            carrier_filter: ResonantFilter::new(),
            modulator_filter: ResonantFilter::new(),
            global_filter: ResonantFilter::new(),
            active: false,
            note: 0,
            velocity: 0.0,
            envelope: Envelope::new(sample_rate), // Initialize Envelope
        }
    }
}

impl Default for AmSynth {
    fn default() -> Self {
        Self {
            params: Arc::new(AmSynthParams::default()),
            sample_rate: 44100.0,
            voices: (0..8).map(|_| Voice::new(44100.0)).collect(),
        }
    }
}

impl Plugin for AmSynth {
    const NAME: &'static str = "AM Synth";
    const VENDOR: &'static str = "The Muzikar";
    const URL: &'static str = "";
    const EMAIL: &'static str = "oskar.wiedrich@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(0),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    type BackgroundTask = ();
    type SysExMessage = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(&mut self, _audio_io_layout: &AudioIOLayout, buffer_config: &BufferConfig, _context: &mut impl InitContext<Self>) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        self.voices = (0..8).map(|_| Voice::new(self.sample_rate)).collect();
        true
    }

    fn reset(&mut self) {
        for voice in &mut self.voices {
            voice.active = false;
        }
    }

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, context: &mut impl ProcessContext<Self>) -> ProcessStatus {
        let tuning = self.params.tuning.value();

        for mut channel_samples in buffer.iter_samples() {
            let mut output = 0.0;

            for voice in &mut self.voices {
                if voice.active {
                    let carrier_freq = if self.params.carrier_keyboard.value() {
                        util::midi_note_to_freq(voice.note, tuning)
                    } else {
                        self.params.carrier_freq.value()
                    };

                    let modulator_freq = if self.params.modulator_keyboard.value() {
                        util::midi_note_to_freq(voice.note, tuning)
                    } else {
                        self.params.modulator_freq.value()
                    };

                    voice.carrier.set_frequency(carrier_freq);
                    voice.modulator.set_frequency(modulator_freq);

                    let carrier_sample = voice.carrier.generate();
                    let modulator_sample = voice.modulator.generate();
                    let modulated = voice.bridge.process(carrier_sample, modulator_sample, self.params.mod_depth.value());

                    let mut voice_output = modulated * voice.velocity;

                    // Process envelope
                    voice_output *= voice.envelope.process();

                    output += voice_output;
                }
            }

            for sample in channel_samples.iter_mut() {
                *sample = output;
            }
        }

        // MIDI events processing
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, velocity, .. } => {
                    if let Some(voice) = self.voices.iter_mut().find(|v| !v.active) {
                        voice.active = true;
                        voice.note = note;
                        voice.velocity = velocity;
                        voice.envelope.trigger();
                    }
                }
                NoteEvent::NoteOff { note, .. } => {
                    if let Some(voice) = self.voices.iter_mut().find(|v| v.active && v.note == note) {
                        voice.envelope.release();
                    }
                }
                _ => (),
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for AmSynth {
    const CLAP_ID: &'static str = "de.muzikar.am-synth";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("An AM Synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for AmSynth {
    const VST3_CLASS_ID: [u8; 16] = *b"AmSynthMuzikar..";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_clap!(AmSynth);
nih_export_vst3!(AmSynth);

use nih_plug::prelude::*;
use std::sync::Arc;
use std::any::TypeId;

mod params;
mod envelope;
mod filter;
mod util;

use params::AmSynthParams;
use envelope::{Envelope, ConventionalAdsr, Dx7Adsr};
use filter::ResonantFilter;

struct AmSynth {
    params: Arc<AmSynthParams>,
    sample_rate: f32,
    carrier_phase: f32,
    modulator_phase: f32,
    carrier_envelope: Box<dyn Envelope>,
    modulator_envelope: Box<dyn Envelope>,
    filter: ResonantFilter,
    note_on: bool,
    note_frequency: f32,
}

impl Default for AmSynth {
    fn default() -> Self {
        Self {
            params: Arc::new(AmSynthParams::default()),
            sample_rate: 44100.0,
            carrier_phase: 0.0,
            modulator_phase: 0.0,
            carrier_envelope: Box::new(ConventionalAdsr::new()),
            modulator_envelope: Box::new(ConventionalAdsr::new()),
            filter: ResonantFilter::new(),
            note_on: false,
            note_frequency: 440.0,
        }
    }
}

impl Plugin for AmSynth {
    const NAME: &'static str = "AM Synth";
    const VENDOR: &'static str = "OseMine";
    const URL: &'static str = "https://your-website.com";
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        true
    }

    fn reset(&mut self) {
        self.carrier_phase = 0.0;
        self.modulator_phase = 0.0;
        self.carrier_envelope = Box::new(ConventionalAdsr::new());
        self.modulator_envelope = Box::new(ConventionalAdsr::new());
        self.filter = ResonantFilter::new();
        self.note_on = false;
        self.note_frequency = 440.0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let carrier_freq = self.params.carrier_freq.smoothed.next();
        let modulator_freq = self.params.modulator_freq.smoothed.next();
        let mod_depth = self.params.mod_depth.smoothed.next();

        // Update envelopes based on the selected type
        if self.params.carrier_envelope_type.value() {
            if self.carrier_envelope.as_any().type_id() != TypeId::of::<Dx7Adsr>() {
                self.carrier_envelope = Box::new(Dx7Adsr::new());
            }
        } else {
            if self.carrier_envelope.as_any().type_id() != TypeId::of::<ConventionalAdsr>() {
                self.carrier_envelope = Box::new(ConventionalAdsr::new());
            }
        }

        if self.params.modulator_envelope_type.value() {
            if self.modulator_envelope.as_any().type_id() != TypeId::of::<Dx7Adsr>() {
                self.modulator_envelope = Box::new(Dx7Adsr::new());
            }
        } else {
            if self.modulator_envelope.as_any().type_id() != TypeId::of::<ConventionalAdsr>() {
                self.modulator_envelope = Box::new(ConventionalAdsr::new());
            }
        }

        // Update envelope parameters
        self.carrier_envelope.set_params(&self.params.carrier_envelope_params);
        self.modulator_envelope.set_params(&self.params.modulator_envelope_params);

        self.filter.set_params(
            self.params.filter_cutoff.smoothed.next(),
            self.params.filter_resonance.smoothed.next(),
        );

        let carrier_keyboard = self.params.carrier_keyboard.value();
        let modulator_keyboard = self.params.modulator_keyboard.value();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            let time = sample_id as f32 / self.sample_rate;

            // Process MIDI events
            while let Some(event) = context.next_event() {
                match event {
                    NoteEvent::NoteOn { note, .. } => {
                        self.note_on = true;
                        self.note_frequency = util::midi_note_to_freq(note);
                        self.carrier_envelope.note_on();
                        self.modulator_envelope.note_on();
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        if util::midi_note_to_freq(note) == self.note_frequency {
                            self.note_on = false;
                            self.carrier_envelope.note_off();
                            self.modulator_envelope.note_off();
                        }
                    }
                    _ => (),
                }
            }

            let carrier_freq = if carrier_keyboard {
                self.note_frequency
            } else {
                carrier_freq
            };

            let modulator_freq = if modulator_keyboard {
                self.note_frequency
            } else {
                modulator_freq
            };

            let carrier_env = self.carrier_envelope.process(self.sample_rate);
            let modulator_env = self.modulator_envelope.process(self.sample_rate);

            self.carrier_phase += carrier_freq * 2.0 * std::f32::consts::PI / self.sample_rate;
            self.modulator_phase += modulator_freq * 2.0 * std::f32::consts::PI / self.sample_rate;

            if self.carrier_phase >= 2.0 * std::f32::consts::PI {
                self.carrier_phase -= 2.0 * std::f32::consts::PI;
            }
            if self.modulator_phase >= 2.0 * std::f32::consts::PI {
                self.modulator_phase -= 2.0 * std::f32::consts::PI;
            }

            let carrier = self.carrier_phase.sin();
            let modulator = self.modulator_phase.sin();

            let am_signal = carrier * (1.0 + mod_depth * modulator * modulator_env) * carrier_env;
            let filtered_signal = self.filter.process(am_signal, self.sample_rate);

            for sample in channel_samples {
                *sample = filtered_signal;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for AmSynth {
    const CLAP_ID: &'static str = "com.your-name.am-synth";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple AM synth");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for AmSynth {
    const VST3_CLASS_ID: [u8; 16] = *b"AmSynthYourName.";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Synth, Vst3SubCategory::Stereo];
}

nih_export_clap!(AmSynth);
nih_export_vst3!(AmSynth);
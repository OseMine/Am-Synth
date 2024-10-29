use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;

use crate::params::AmSynthParams;

#[derive(Lens)]
struct AmSynthGui {
    params: Arc<AmSynthParams>,
}

impl Model for AmSynthGui {}

impl AmSynthGui {
    fn new(params: Arc<AmSynthParams>) -> Self {
        Self { params }
    }
}

pub(crate) fn create(params: Arc<AmSynthParams>) -> Arc<ViziaState> {
    ViziaState::new(move |cx| {
        AmSynthGui::new(params.clone()).build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "AM Synth");
            HStack::new(cx, |cx| {
                // Carrier controls
                VStack::new(cx, |cx| {
                    Label::new(cx, "Carrier");
                    knob(cx, params.carrier_freq.as_ref(), "Frequency");
                    knob(cx, params.carrier_attack.as_ref(), "Attack");
                    knob(cx, params.carrier_decay.as_ref(), "Decay");
                    knob(cx, params.carrier_sustain.as_ref(), "Sustain");
                    knob(cx, params.carrier_release.as_ref(), "Release");
                    toggle(cx, params.carrier_keyboard.as_ref(), "Keyboard");
                });

                // Modulator controls
                VStack::new(cx, |cx| {
                    Label::new(cx, "Modulator");
                    knob(cx, params.modulator_freq.as_ref(), "Frequency");
                    knob(cx, params.modulator_attack.as_ref(), "Attack");
                    knob(cx, params.modulator_decay.as_ref(), "Decay");
                    knob(cx, params.modulator_sustain.as_ref(), "Sustain");
                    knob(cx, params.modulator_release.as_ref(), "Release");
                    toggle(cx, params.modulator_keyboard.as_ref(), "Keyboard");
                });

                // Other controls
                VStack::new(cx, |cx| {
                    Label::new(cx, "Global");
                    knob(cx, params.mod_depth.as_ref(), "Mod Depth");
                    knob(cx, params.filter_cutoff.as_ref(), "Filter Cutoff");
                    knob(cx, params.filter_resonance.as_ref(), "Filter Resonance");
                });
            });
        })
        .background_color(Color::rgb(53, 54, 55))
        .width(Pixels(520.0))
        .height(Pixels(300.0));
    })
}

fn knob(cx: &mut Context, param: &impl Param, label: &str) {
    VStack::new(cx, |cx| {
        Label::new(cx, label);
        ParamSlider::new(cx, param);
    })
    .width(Pixels(80.0))
    .height(Pixels(90.0));
}

fn toggle(cx: &mut Context, param: &impl Param, label: &str) {
    VStack::new(cx, |cx| {
        Label::new(cx, label);
        Switch::new(cx, param);
    })
    .width(Pixels(80.0))
    .height(Pixels(50.0));
}

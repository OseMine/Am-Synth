use std::any::Any;
use crate::params::EnvelopeParams;

pub trait Envelope: Send + Any {
    fn new() -> Self where Self: Sized;
    fn set_params(&mut self, params: &EnvelopeParams);
    fn note_on(&mut self);
    fn note_off(&mut self);
    fn process(&mut self, sample_rate: f32) -> f32;
    fn as_any(&self) -> &dyn Any;
}

pub struct ConventionalAdsr {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: AdsrState,
    output: f32,
    time: f32,
}

#[derive(Clone, Copy)]
enum AdsrState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope for ConventionalAdsr {
    fn new() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.5,
            release: 0.1,
            state: AdsrState::Idle,
            output: 0.0,
            time: 0.0,
        }
    }

    fn set_params(&mut self, params: &EnvelopeParams) {
        self.attack = params.attack;
        self.decay = params.decay;
        self.sustain = params.sustain;
        self.release = params.release;
    }

    fn note_on(&mut self) {
        self.state = AdsrState::Attack;
        self.time = 0.0;
    }

    fn note_off(&mut self) {
        self.state = AdsrState::Release;
        self.time = 0.0;
    }

    fn process(&mut self, sample_rate: f32) -> f32 {
        self.time += 1.0 / sample_rate;

        match self.state {
            AdsrState::Idle => self.output = 0.0,
            AdsrState::Attack => {
                self.output = self.time / self.attack;
                if self.output >= 1.0 {
                    self.output = 1.0;
                    self.state = AdsrState::Decay;
                    self.time = 0.0;
                }
            }
            AdsrState::Decay => {
                self.output = 1.0 - (1.0 - self.sustain) * (self.time / self.decay);
                if self.output <= self.sustain {
                    self.output = self.sustain;
                    self.state = AdsrState::Sustain;
                }
            }
            AdsrState::Sustain => self.output = self.sustain,
            AdsrState::Release => {
                self.output = self.sustain * (1.0 - self.time / self.release);
                if self.output <= 0.0 {
                    self.output = 0.0;
                    self.state = AdsrState::Idle;
                }
            }
        }

        self.output
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Dx7Adsr {
    levels: [f32; 4],
    rates: [f32; 4],
    state: usize,
    output: f32,
    time: f32,
}

impl Envelope for Dx7Adsr {
    fn new() -> Self {
        Self {
            levels: [1.0, 0.8, 0.6, 0.0],
            rates: [0.1, 0.2, 0.3, 0.4],
            state: 0,
            output: 0.0,
            time: 0.0,
        }
    }

    fn set_params(&mut self, params: &EnvelopeParams) {
        self.levels = [params.level1, params.level2, params.level3, params.level4];
        self.rates = [params.rate1, params.rate2, params.rate3, params.rate4];
    }

    fn note_on(&mut self) {
        self.state = 0;
        self.time = 0.0;
    }

    fn note_off(&mut self) {
        self.state = 3;
        self.time = 0.0;
    }

    fn process(&mut self, sample_rate: f32) -> f32 {
        self.time += 1.0 / sample_rate;

        let target = self.levels[self.state];
        let rate = self.rates[self.state];

        self.output += (target - self.output) * rate;

        if (self.output - target).abs() < 0.001 && self.state < 3 {
            self.state += 1;
            self.time = 0.0;
        }

        self.output
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

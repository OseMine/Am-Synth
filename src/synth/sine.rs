use std::f32::consts::PI;

pub struct SineOscillator {
    phase: f32,
    phase_delta: f32,
    sample_rate: f32,
    frequency: f32,
}

impl SineOscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            phase_delta: 0.0,
            sample_rate,
            frequency: 440.0, // Standardfrequenz, kann angepasst werden
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
        self.update_phase_delta();
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.update_phase_delta();
    }

    fn update_phase_delta(&mut self) {
        self.phase_delta = 2.0 * PI * self.frequency / self.sample_rate;
    }

    pub fn generate(&mut self) -> f32 {
        let sample = self.phase.sin();
        self.phase += self.phase_delta;
        if self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
        sample
    }
}

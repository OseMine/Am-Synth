pub struct ResonantFilter {
    cutoff: f32,
    resonance: f32,
    filter_type: bool, // True for Moog, false for Roland
    y1: f32,
    y2: f32,
    y3: f32,
    y4: f32,
    oldx: f32,
    oldy1: f32,
    oldy2: f32,
    oldy3: f32,
}

impl ResonantFilter {
    pub fn new() -> Self {
        Self {
            cutoff: 1000.0,
            resonance: 0.0,
            filter_type: true, // Default to Moog
            y1: 0.0,
            y2: 0.0,
            y3: 0.0,
            y4: 0.0,
            oldx: 0.0,
            oldy1: 0.0,
            oldy2: 0.0,
            oldy3: 0.0,
        }
    }

    pub fn set_params(&mut self, cutoff: f32, resonance: f32) {
        self.cutoff = cutoff.clamp(20.0, 20000.0);
        self.resonance = resonance.clamp(0.0, 1.0);
    }

    pub fn set_filter_type(&mut self, filter_type: bool) {
        self.filter_type = filter_type;
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        if self.filter_type {
            // Moog-Charakteristik
            let f = 2.0 * self.cutoff / sample_rate;
            let k = 3.6 * f - 1.6 * f * f - 1.0;
            let p = (k + 1.0) * 0.5;
            let scale = (1.0 - p) * 1.386249;
            let r = self.resonance * scale;

            let x = input - r * self.y4;

            self.y1 = x * p + self.oldx * p - k * self.y1;
            self.y2 = self.y1 * p + self.oldy1 * p - k * self.y2;
            self.y3 = self.y2 * p + self.oldy2 * p - k * self.y3;
            self.y4 = self.y3 * p + self.oldy3 * p - k * self.y4;

            self.y4 = self.y4.clamp(-1.0, 1.0);

            self.oldx = x;
            self.oldy1 = self.y1;
            self.oldy2 = self.y2;
            self.oldy3 = self.y3;

            self.y4
        } else {
            // Roland-Charakteristik
            let f = 2.0 * self.cutoff / sample_rate;
            let k = 3.6 * f - 1.6 * f * f - 1.0;
            let p = (k + 1.0) * 0.5;
            let scale = (1.0 - p) * 1.386249;
            let r = self.resonance * scale;

            let x = input - r * self.y4;

            self.y1 = x * p + self.oldx * p - k * self.y1;
            self.y2 = self.y1 * p + self.oldy1 * p - k * self.y2;
            self.y3 = self.y2 * p + self.oldy2 * p - k * self.y3;
            self.y4 = self.y3 * p + self.oldy3 * p - k * self.y4;

            self.y4 = self.y4.clamp(-1.0, 1.0);

            self.oldx = x;
            self.oldy1 = self.y1;
            self.oldy2 = self.y2;
            self.oldy3 = self.y3;

            self.y4
        }
    }
}

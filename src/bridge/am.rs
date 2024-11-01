pub struct AmBridge;

impl AmBridge {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, carrier: f32, modulator: f32, depth: f32) -> f32 {
        let modulation = 1.0 + (modulator * depth);
        carrier * modulation
    }
}
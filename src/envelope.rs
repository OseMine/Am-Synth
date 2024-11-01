pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    stage: EnvelopeStage,
    output: f32,
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
    pub fn new(sample_rate: f32) -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.5,
            release: 0.1,
            stage: EnvelopeStage::Idle,
            output: 0.0,
            sample_rate,
        }
    }

    pub fn set_params(&mut self, attack: f32, decay: f32, sustain: f32, release: f32) {
        self.attack = attack;
        self.decay = decay;
        self.sustain = sustain;
        self.release = release;
    }

    pub fn trigger(&mut self) {
        self.stage = EnvelopeStage::Attack;
        self.output = 0.0;
    }

    pub fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
    }

    pub fn process(&mut self) -> f32 {
        match self.stage {
            EnvelopeStage::Idle => {}
            EnvelopeStage::Attack => {
                self.output += 1.0 / (self.attack * self.sample_rate);
                if self.output >= 1.0 {
                    self.output = 1.0;
                    self.stage = EnvelopeStage::Decay;
                }
            }
            EnvelopeStage::Decay => {
                self.output -= (1.0 - self.sustain) / (self.decay * self.sample_rate);
                if self.output <= self.sustain {
                    self.output = self.sustain;
                    self.stage = EnvelopeStage::Sustain;
                }
            }
            EnvelopeStage::Sustain => {}
            EnvelopeStage::Release => {
                self.output -= self.sustain / (self.release * self.sample_rate);
                if self.output <= 0.0 {
                    self.output = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            }
        }
        self.output
    }
}

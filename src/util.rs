pub fn midi_note_to_freq(note: u8, tuning: f32) -> f32 {
    tuning * 2.0f32.powf((note as f32 - 69.0) / 12.0)
}

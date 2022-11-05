use rodio::Source;
use std::time::Duration;

use crate::audio::{AudioUnit, Buffer};

pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Buffer,
    index: f32,
    increment: f32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Buffer) -> WavetableOscillator {
        WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> AudioUnit {
        let sample = self.lerp();

        self.index += self.increment;
        self.index %= self.wave_table.len() as f32;

        return sample;
    }

    fn lerp(&self) -> AudioUnit {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

impl Iterator for WavetableOscillator {
    type Item = AudioUnit;

    fn next(&mut self) -> Option<AudioUnit> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

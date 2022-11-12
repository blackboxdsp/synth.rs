use rodio::Source;
use std::time::Duration;

use crate::audio::{AudioUnit, Buffer};

pub enum Waveform {
    Sine = 0,
    Square = 1,
    Sawtooth = 2,
    Triangle = 3,
}

pub struct WavetableOscillator {
    sample_rate: u32,
    wavetable: Buffer,
    index: f32,
    increment: f32,
}

impl WavetableOscillator {
    fn generate_wavetable(wavetable_size: usize, waveform: Waveform) -> Buffer {
        let mut wave_table: Buffer = Vec::with_capacity(wavetable_size);

        match waveform {
            Waveform::Sine => WavetableOscillator::generate_sine_wavetable(&mut wave_table, wavetable_size),
            Waveform::Square => WavetableOscillator::generate_square_wavetable(&mut wave_table, wavetable_size),
            Waveform::Sawtooth => WavetableOscillator::generate_sawtooth_wavetable(&mut wave_table, wavetable_size),
            Waveform::Triangle => WavetableOscillator::generate_triangle_wavetable(&mut wave_table, wavetable_size),
        }

        wave_table
    }

    fn generate_sine_wavetable(buffer: &mut Buffer, wavetable_size: usize) {
        for n in 0..wavetable_size {
            let phase = (n as AudioUnit / wavetable_size as AudioUnit) * std::f32::consts::PI * 2.0;
            buffer.push(phase.sin());
        }
    }

    fn generate_square_wavetable(buffer: &mut Buffer, wavetable_size: usize) {
        for n in 0..wavetable_size {
            let half_size = wavetable_size / 2;
            let value = if n < half_size {
                1.0
            } else if n == half_size {
                0.0
            } else {
                -1.0
            };
            buffer.push(value);
        }
    }

    fn generate_sawtooth_wavetable(buffer: &mut Buffer, wavetable_size: usize) {
        for n in 0..wavetable_size {
            let (m, b) = (1.0 / wavetable_size as AudioUnit, 0.0);
            let value = m * n as AudioUnit + b;
            buffer.push((value * 2.0) - 1.0);
        }
    }

    fn generate_triangle_wavetable(buffer: &mut Buffer, wavetable_size: usize) {
        for n in 0..wavetable_size {
            let half_size = wavetable_size / 2;
            let (m, b) = if n <= half_size {
                (1.0 / half_size as AudioUnit, 0.0)
            } else {
                (-1.0 / half_size as AudioUnit, 2.0)
            };
            let value = m * n as AudioUnit + b;
            buffer.push((value * 2.0) - 1.0)
        }
    }
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, table_size: usize, waveform: Waveform) -> WavetableOscillator {
        let wave_table = WavetableOscillator::generate_wavetable(table_size, waveform);
        WavetableOscillator {
            sample_rate,
            wavetable: wave_table,
            index: 0.0,
            increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.increment = frequency * self.wavetable.len() as f32 / self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> AudioUnit {
        let sample = self.lerp();

        self.index += self.increment;
        self.index %= self.wavetable.len() as f32;

        return sample;
    }

    fn lerp(&self) -> AudioUnit {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wavetable.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wavetable[truncated_index]
            + next_index_weight * self.wavetable[next_index];
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

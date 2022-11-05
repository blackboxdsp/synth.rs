mod audio;
mod wavetable_oscillator;

use std::time::Duration;
use rodio::{OutputStream, Source};

use crate::{audio::{Buffer}, wavetable_oscillator::{WavetableOscillator}};

const PROGRAM_DURATION: u64 = 1;

fn main() {
    println!("Hello, world!");

    let wave_table_size = 64;
    let mut wave_table: Buffer = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }

    let mut oscillator = WavetableOscillator::new(44100, wave_table);
    oscillator.set_frequency(440.0);

    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let result = stream_handle.play_raw(oscillator.convert_samples());
    
    std::thread::sleep(Duration::from_secs(PROGRAM_DURATION));
}

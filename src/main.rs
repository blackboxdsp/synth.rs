pub mod audio;
pub mod generators;

use std::time::Duration;

use rodio::{OutputStream, Source};

use crate::generators::wavetable_oscillator::{Waveform, WavetableOscillator};

const PROGRAM_DURATION: u64 = 1;
const SAMPLE_RATE: u32 = 44100;
const WAVETABLE_SIZE: usize = 2048;
const OSCILLATOR_FREQUENCY: f32 = 220.0;

fn main() {
    let mut oscillator = WavetableOscillator::new(SAMPLE_RATE, WAVETABLE_SIZE, Waveform::Sawtooth);
    oscillator.set_frequency(OSCILLATOR_FREQUENCY);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(Duration::from_secs(PROGRAM_DURATION));
}

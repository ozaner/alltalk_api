use std::{io::Read, time::Duration};

use rodio::Source;

pub struct StreamingSource<R: Read> {
    reader: R,
    sample_rate: u32,
    channels: u16,
}

impl<R: Read> StreamingSource<R> {
    pub fn new(reader: R, sample_rate: u32, channels: u16) -> Self {
        Self {
            reader,
            sample_rate,
            channels,
        }
    }
}

impl<R: Read> Source for StreamingSource<R> {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None //streamed so unknown ahead of time
    }
}

impl<R: Read> Iterator for StreamingSource<R> {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        let mut buf = [0; 2];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => Some(i16::from_le_bytes(buf)),
            Err(_) => None,
        }
    }
}

//! This module defines the [`StreamingWav`] struct.
//!
//! For a useful specification of the WAVE format, see [here](https://www.mmsp.ece.mcgill.ca/Documents/AudioFormats/WAVE/WAVE.html).

use std::io::Read;
use std::time::Duration;

use rodio::Source;

/// Wraps a [`Read`] into an audio [`Source`] that can be used with [`rodio`].
///
/// **Only supports 16-bit PCM format.**
pub struct StreamingWav<R: Read> {
    reader: R,
    sample_rate: u32,
    channels: u16,
}

impl<R: Read> StreamingWav<R> {
    pub fn new(mut reader: R) -> Result<Self, std::io::Error> {
        // Read the RIFF header
        let mut riff_header = [0u8; 12];
        reader.read_exact(&mut riff_header)?;

        // Verify the RIFF and WAVE identifiers
        if &riff_header[0..4] != b"RIFF" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "File is not a valid RIFF format",
            ));
        }
        if &riff_header[8..12] != b"WAVE" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "File is not a valid WAVE format",
            ));
        }

        let mut sample_rate = 0u32;
        let mut channels = 0u16;

        // Read chunks until we find the 'fmt ' and 'data' chunks
        loop {
            // Read chunk header
            let mut chunk_header = [0u8; 8];
            reader.read_exact(&mut chunk_header)?;
            let chunk_id = &chunk_header[0..4];
            let chunk_size = u32::from_le_bytes(chunk_header[4..8].try_into().unwrap());

            if chunk_id == b"fmt " {
                // Read 'fmt ' chunk
                let mut fmt_chunk = vec![0u8; chunk_size as usize];
                reader.read_exact(&mut fmt_chunk)?;

                let audio_format = u16::from_le_bytes(fmt_chunk[0..2].try_into().unwrap());
                if audio_format != 1 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Unsupported audio format (only PCM is supported)",
                    ));
                }

                channels = u16::from_le_bytes(fmt_chunk[2..4].try_into().unwrap());
                sample_rate = u32::from_le_bytes(fmt_chunk[4..8].try_into().unwrap());
                let bits_per_sample = u16::from_le_bytes(fmt_chunk[14..16].try_into().unwrap());

                if bits_per_sample != 16 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Only 16 bits per sample is supported",
                    ));
                }
            } else if chunk_id == b"data" {
                // Found the 'data' chunk; ready to read samples
                return Ok(Self {
                    reader,
                    sample_rate,
                    channels,
                });
            } else {
                // Skip over non-'fmt ' and non-'data' chunks
                let mut skip = chunk_size as usize;
                if skip % 2 == 1 {
                    skip += 1; // Account for padding byte
                }
                std::io::copy(&mut reader.by_ref().take(skip as u64), &mut std::io::sink())?;
            }
        }
    }
}

impl<R: Read> Source for StreamingWav<R> {
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
        None // Streamed, so duration is unknown ahead of time
    }
}

impl<R: Read> Iterator for StreamingWav<R> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 2];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => Some(i16::from_le_bytes(buf)),
            Err(_) => None,
        }
    }
}

use alltalk_api::Client;
use rodio::{OutputStream, Sink, Source};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Check if the server is ready before we start
    match client.get_ready().await {
        Ok(ready) => {
            if ready {
                println!("Server is ready.")
            } else {
                eprintln!("Server is not ready or unavailable.");
                return;
            }
        }
        Err(err) => {
            eprintln!("Error checking server readiness: {:?}", err);
            return;
        }
    }

    // Set up the audio output
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Input loop
    let mut input = String::new();
    loop {
        // Get input from the user
        print!("Enter text to generate TTS (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim().to_owned();

        // Quit if the user types "q"
        if trimmed == "q" {
            break;
        }

        // Generate the TTS stream
        match client
            .generate_tts_stream(trimmed, "female_01.wav", "en")
            .await
        {
            Ok(streaming_source) => {
                println!("Playing audio...");
                sink.append(streaming_source.convert_samples::<f32>());
                sink.sleep_until_end(); // Wait for audio to finish playing
            }
            Err(err) => eprintln!("Error generating TTS stream: {:?}", err),
        }

        input.clear();
    }

    println!("quitting...");
}

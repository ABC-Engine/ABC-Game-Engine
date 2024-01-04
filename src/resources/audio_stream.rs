use rodio::source::SamplesConverter;
use rodio::OutputStreamHandle;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use ABC_ECS::Resource;

/// A struct that holds an audio file
pub struct AudioFile {
    pub(crate) file: Decoder<BufReader<File>>,
}

impl AudioFile {
    /// creates a new audio file from the given path
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode file");

        AudioFile { file: source }
    }
}

/// The resource that is used to play audio files
pub struct AudioHandle {
    pub(crate) handle: rodio::OutputStreamHandle,
    // this is needed to keep the stream alive
    _stream: OutputStream,
}

impl AudioHandle {
    /// creates a new audio handle, which is used to play audio files
    pub(crate) fn new() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().expect("Failed to open stream");
        Self { handle, _stream }
    }

    /// plays the audio file once
    pub fn play_one_shot(&self, audio_file: AudioFile) {
        let _ = self.handle.play_raw(audio_file.file.convert_samples());
    }

    /// plays the audio files in sequence, waiting for each one to finish before playing the next
    /// sleeps until the end of the last audio file
    pub fn play_sounds_in_sequence(&self, audio_files: Vec<AudioFile>) {
        let sink = rodio::Sink::try_new(&self.handle).expect("Failed to create sink");

        for audio_file in audio_files {
            sink.append(audio_file.file.convert_samples::<f32>());
        }

        sink.sleep_until_end();
    }

    /// plays the audio file over and over again until the program is terminated
    pub fn play_infinitely(&self, audio_file: AudioFile) {
        let _ = self
            .handle
            .play_raw(audio_file.file.convert_samples().repeat_infinite());
    }
}

impl Resource for AudioHandle {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

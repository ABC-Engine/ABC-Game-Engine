use fxhash::FxHashMap;
use rodio::cpal::FromSample;
use rodio::source::SamplesConverter;
use rodio::{source::Source, Decoder, OutputStream};
use rodio::{OutputStreamHandle, Sample};
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
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

#[derive(Clone)]
pub struct AudioBus {
    parent: Option<String>,
    name: String,
    volume: f32,
    speed: f32,
}

impl AudioBus {
    pub fn new(name: &str) -> Self {
        Self {
            parent: None,
            name: name.to_string(),
            volume: 1.0,
            speed: 1.0,
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn get_volume(&self, audio_handle: &AudioHandle) -> f32 {
        match self.parent {
            Some(ref parent) => {
                let parent = audio_handle
                    .buses
                    .get(parent)
                    .expect(format!("Parent: {} not found for {}", parent, self.name).as_str());

                self.volume * parent.get_volume(audio_handle)
            }
            None => self.volume,
        }
    }

    pub fn get_volume_without_parent(&self) -> f32 {
        self.volume
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn get_speed(&self, audio_handle: &AudioHandle) -> f32 {
        match self.parent {
            Some(ref parent) => {
                let parent = audio_handle
                    .buses
                    .get(parent)
                    .expect(format!("Parent: {} not found for {}", parent, self.name).as_str());

                self.speed * parent.get_speed(audio_handle)
            }
            None => self.speed,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_parent(&mut self, parent: &str) {
        self.parent = Some(parent.to_string());
    }

    pub fn get_parent(&self) -> Option<&String> {
        self.parent.as_ref()
    }

    pub fn get_full_name(&self, handle: &AudioHandle) -> String {
        let mut full_name = self.name.clone();
        if let Some(parent) = &self.parent {
            let parent = handle.buses.get(parent).expect("Parent not found");
            *&mut full_name += &parent.get_full_name(handle);
        }

        full_name.to_string()
    }
}

/// A struct that can play multiple audio files at the same time
struct ParellelSink {
    sinks: Vec<rodio::Sink>,
    volume: f32,
    speed: f32,
}

impl ParellelSink {
    pub fn new() -> Self {
        Self {
            sinks: Vec::new(),
            volume: 1.0,
            speed: 1.0,
        }
    }

    fn append<S>(&mut self, source: S, handle: &rodio::OutputStreamHandle)
    where
        S: Source + Send + 'static,
        f32: FromSample<S::Item>,
        S::Item: Sample + Send,
    {
        // either find an empty sink or create a new one
        self.append_without_cleanup(source, handle);

        // check if any of the sinks are empty and if they are remove them
        self.clean_up();
    }

    fn append_without_cleanup<S>(&mut self, source: S, handle: &rodio::OutputStreamHandle)
    where
        S: Source + Send + 'static,
        f32: FromSample<S::Item>,
        S::Item: Sample + Send,
    {
        // O(n) but n is small so i think it's fine but it's something to keep in mind
        for sink in &self.sinks {
            if sink.empty() {
                sink.append(source);
                return;
            }
        }

        // add a new sink if all are being used

        let sink = rodio::Sink::try_new(handle).expect("Failed to create audio sink");
        sink.set_volume(self.volume);
        sink.set_speed(self.speed);
        sink.append(source);
        self.sinks.push(sink);
    }

    fn clean_up(&mut self) {
        self.sinks.retain(|sink| !sink.empty());
    }

    fn set_volume(&mut self, volume: f32) {
        for sink in &mut self.sinks {
            sink.set_volume(volume);
        }
        self.volume = volume;
    }

    fn set_speed(&mut self, speed: f32) {
        for sink in &mut self.sinks {
            sink.set_speed(speed);
        }
        self.speed = speed;
    }
}

/// The resource that is used to play audio files
pub struct AudioHandle {
    pub(crate) handle: rodio::OutputStreamHandle,
    // this is needed to keep the stream alive
    _stream: OutputStream,
    master_volume: f32,
    master_speed: f32,
    sink: ParellelSink,
    sinks: FxHashMap<String, ParellelSink>, // this way we can have multiple sinks with different volumes
    buses: FxHashMap<String, AudioBus>,
}

impl AudioHandle {
    /// creates a new audio handle, which is used to play audio files
    pub(crate) fn new() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().expect("Failed to open stream");
        Self {
            handle: handle.clone(),
            _stream,
            master_volume: 1.0,
            master_speed: 1.0,
            sink: ParellelSink::new(),
            sinks: FxHashMap::default(),
            buses: FxHashMap::default(),
        }
    }

    /// plays the audio file once
    pub fn play_one_shot(&mut self, audio_file: AudioFile) {
        self.sink.set_volume(self.master_volume);
        self.sink.set_speed(self.master_speed);
        self.sink
            .append(audio_file.file.convert_samples::<f32>(), &self.handle);
    }

    /// plays the audio files in sequence, waiting for each one to finish before playing the next
    /// sleeps until the end of the last audio file
    pub fn play_sounds_in_sequence(&mut self, audio_files: Vec<AudioFile>) {
        for audio_file in audio_files {
            self.sink
                .append(audio_file.file.convert_samples::<f32>(), &self.handle);
        }
    }

    /// plays the audio file over and over again until the program is terminated
    pub fn play_infinitely(&mut self, audio_file: AudioFile) {
        self.sink.append(
            audio_file.file.convert_samples::<f32>().repeat_infinite(),
            &self.handle,
        );
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume;
        self.sink.set_volume(volume);
    }

    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn play_sound_on_bus(&mut self, audio_file: AudioFile, name: &str) {
        let (volume, speed) = self.get_volume_and_speed_of(name);

        // this should be a function but rust doesn't like it :(
        // because, it would not allow the handle to be borrowed
        let sink = self
            .sinks
            .entry(name.to_string())
            .or_insert_with(|| ParellelSink::new());

        sink.set_volume(volume * self.master_volume);
        sink.set_speed(speed * self.master_speed);
        sink.append(audio_file.file.convert_samples::<f32>(), &self.handle);
    }

    pub fn play_sounds_in_sequence_on_bus(&mut self, audio_files: Vec<AudioFile>, name: &str) {
        let (volume, speed) = self.get_volume_and_speed_of(name);

        let sink = self
            .sinks
            .entry(name.to_string())
            .or_insert_with(|| ParellelSink::new());

        sink.set_volume(volume * self.master_volume);
        sink.set_speed(speed * self.master_speed);
        for audio_file in audio_files {
            sink.append(audio_file.file.convert_samples::<f32>(), &self.handle);
        }
    }

    fn get_volume_and_speed_of(&self, name: &str) -> (f32, f32) {
        let bus = self
            .buses
            .get(name)
            .expect("Bus not found, call add_bus first");

        (bus.get_volume(self), bus.get_speed(self))
    }

    pub fn play_infinitely_on_bus(&mut self, audio_file: AudioFile, name: &str) {
        let (volume, speed) = self.get_volume_and_speed_of(name);

        let sink = self
            .sinks
            .entry(name.to_string())
            .or_insert_with(|| ParellelSink::new());

        sink.set_volume(volume * self.master_volume);
        sink.set_speed(speed * self.master_speed);
        sink.append(
            audio_file.file.convert_samples::<f32>().repeat_infinite(),
            &self.handle,
        );
    }

    pub fn add_bus(&mut self, bus: AudioBus) {
        self.buses.insert(bus.name.clone(), bus);
    }

    pub fn get_bus(&self, bus: &str) -> Option<&AudioBus> {
        self.buses.get(bus)
    }

    /// gets the bus with the given name or creates a new one if it doesn't exist
    pub fn get_or_make_bus(&mut self, bus: &str) -> &mut AudioBus {
        self.buses
            .entry(bus.to_string())
            .or_insert_with(|| AudioBus::new(bus))
    }

    pub fn add_bus_to_bus(&mut self, mut new_bus: AudioBus, parent_bus: &str) {
        new_bus.parent = Some(parent_bus.to_string());
        self.buses.insert(new_bus.name.clone(), new_bus.clone());
    }

    pub fn drop_all_sounds(&mut self) {
        self.sink.sinks.clear();
        self.sinks.clear();
    }
}

impl Resource for AudioHandle {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

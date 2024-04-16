use ABC_Game_Engine::*;

fn main() {
    let mut scene = Scene::new();
    let entities_and_components = &mut scene.world.entities_and_components;
    // play the "main_music" audio file
    let audio_handle = entities_and_components
        .get_resource::<AudioHandle>()
        .expect("Failed to get audio handle");

    let audio_file = AudioFile::new("main_music.wav");
    audio_handle.play_sounds_in_sequence(vec![audio_file]);
    // to make sure the audio file is played, we need to delay the execution of the program
    std::thread::sleep(std::time::Duration::from_secs(60));
}

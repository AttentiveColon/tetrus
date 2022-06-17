use macroquad::audio::*;
use std::collections::HashMap;

pub struct SoundCollection {
    sounds: HashMap<String, Sound>,
}

impl SoundCollection {
    pub fn new() -> Self {
        SoundCollection {
            sounds: HashMap::new(),
        }
    }

    pub async fn add_sound(&mut self, path: &str, name: &str) {
        let sound = load_sound(path).await.unwrap();
        self.sounds.insert(String::from(name), sound);
    }

    pub fn play(&mut self, name: &str, params: PlaySoundParams) {
        let sound = self.sounds.entry(String::from(name));
        match sound {
            std::collections::hash_map::Entry::Occupied(x) => play_sound(*x.get(), params),
            std::collections::hash_map::Entry::Vacant(_) => (),
        }
    }
}

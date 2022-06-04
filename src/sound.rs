
use std::path::Path;

use kira::{
	manager::{ AudioManager, AudioManagerSettings, backend::cpal::CpalBackend },
	sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
};

use log::error;

pub struct SoundManager {
    manager: AudioManager
}

pub struct Sound {
    name: String,
    sound: StaticSoundData,
    sound_handler: Option<StaticSoundHandle>
}

impl SoundManager 
{
    pub fn new() -> Self 
    {
        let manager = match AudioManager::<CpalBackend>::new(AudioManagerSettings::default())
        {
            Ok(manager) => { manager },
            Err(e) => { panic!("Sound manager failed to kira audio manager, error message: {}", e) }
        };
        Self { manager }
    }

    pub fn play(&mut self, sound: &mut Sound) 
    {
        let handler = self.manager.play(sound.sound.clone()).unwrap();
        sound.sound_handler = Some(handler);
    }

    pub fn pause_all(&mut self)  
    {
        match self.manager.pause(Default::default())
        {
            Err(e) => { error!("couldn't pause all sounds, error message: {}", e) },
            _ => { }
        };
    }

    pub fn resume_all(&mut self) 
    {
        match self.manager.resume(Default::default())
        {
            Err(e) => { error!("couldn't resume all sounds, error message: {}", e) },
            _ => { }
        };
        
    }
}

impl  Sound {

    pub fn new(file_path: &str) -> Self 
    {
        let file_path = Path::new(file_path);
        let sound = match StaticSoundData::from_file(file_path, StaticSoundSettings::default())
        {
            Ok(sound) => { sound },
            Err(e) => { panic!("couldn't create sound object from {}, error message: {}", file_path.to_str().unwrap(), e) }
        };

        let filename = file_path.file_name().unwrap().to_str().unwrap().to_string();
        Self { name: filename, sound, sound_handler: None }
    }

    pub fn pause(&mut self) 
    {
        match &mut self.sound_handler {
          Some(sound_handler) => sound_handler.pause(Default::default()).unwrap(),
          None => { error!("couldn't pause sound: {}", self.name) }
        }
    }

    pub fn resume(&mut self) {
        match &mut self.sound_handler {
            Some(sound_handler) => sound_handler.resume(Default::default()).unwrap(),
            None => { error!("couldn't resume sound: {}", self.name) }
          }
    }

    pub fn stop(&mut self) {
        match &mut self.sound_handler {
            Some(sound_handler) => sound_handler.stop(Default::default()).unwrap(),
            None => { error!("couldn't stop sound: {}", self.name) }
          }
    }
}


use kira::{
	manager::{ AudioManager, AudioManagerSettings, backend::cpal::CpalBackend },
	sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings}
};

pub struct SoundManager {
    manager: AudioManager
}

pub struct Sound {
    sound: StaticSoundData,
    sound_handler: Option<StaticSoundHandle>
}

impl SoundManager 
{
    pub fn new() -> Self 
    {
        let manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
        Self { manager }
    }

    pub fn play(&mut self, sound: &mut Sound) 
    {
        let handler = self.manager.play(sound.sound.clone()).unwrap();
        sound.sound_handler = Some(handler);
    }

    pub fn pause_all(&mut self)  {
        self.manager.pause(Default::default()).unwrap();
    }

    pub fn resume_all(&mut self) {
        self.manager.resume(Default::default()).unwrap();
    }


}

impl  Sound {

    pub fn new(file_path: &str) -> Self 
    {
        let sound = StaticSoundData::from_file(file_path, StaticSoundSettings::default()).unwrap();
        Self { sound, sound_handler: None }
    }

    pub fn pause(&mut self) 
    {
        match &mut self.sound_handler {
          Some(sound_handler) => sound_handler.pause(Default::default()).unwrap(),
          None => { }
        }
    }

    pub fn resume(&mut self) {
        match &mut self.sound_handler {
            Some(sound_handler) => sound_handler.resume(Default::default()).unwrap(),
            None => { }
          }
    }

    pub fn stop(&mut self) {
        match &mut self.sound_handler {
            Some(sound_handler) => sound_handler.stop(Default::default()).unwrap(),
            None => { }
          }
    }
}


extern crate sdl2; 

use sdl2::{audio::{AudioCallback, AudioSpecDesired}, AudioSubsystem, Sdl};
use std::time::Duration;


struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}


pub struct Apu
{
    sound_timer: i8,
    audio_subsystem: AudioSubsystem,
    //
}

impl Apu
{
    pub fn new(sdl_context: &Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();
        Self
        {
            sound_timer: 0,
            audio_subsystem,
        }
    }
    // ..
    pub fn play_audio(&self, sound_timer: &mut u8)
    {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };
        
        let device = self.audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.5
            }
        }).unwrap();


        //if *sound_timer > 0 {
           //device.pause();
            device.resume();
            std::thread::sleep(Duration::from_millis(16));

            //}
        //
    }
}
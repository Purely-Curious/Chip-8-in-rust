extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;



 
#[derive(Debug)]
    pub struct Display<'a>
    {
        window_height: u32,
        window_width: u32,
        framebuffer: &'a[i16],
    }

    impl<'a> Display<'a> 
    {
        pub fn new() -> Display<'a>
        {
            Display
            {
                window_height: 640,
                window_width: 320,
                framebuffer: &[0; 64*32],
            }

        }
        pub fn display_screen(&self)
        {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem.window("Chip-8 emulator", self.window_width, self.window_height)
                    .position_centered()
                    .build()
                    .unwrap();

            let mut canvas = window.into_canvas()
            .build().unwrap();
            
            canvas.set_draw_color(Color::RGB(0, 255, 255));
            canvas.clear();
            canvas.present();

            let mut event_pump = sdl_context.event_pump().unwrap();

            'running: loop
            {
                for event in event_pump.poll_iter()
                {
                    match event
                    {
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                        {
                            break 'running;
                        },
                        _ => (),
                    }
                }
            }
        }
    }

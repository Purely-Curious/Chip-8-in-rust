extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::Sdl;

pub struct Ppu
{
	title: String,
	window_height: u32,
	window_width: u32,
	fb: [i16; 64 * 32],
    //pub sdl_context: Sdl,
	canvas: Canvas<Window>,
}

// [[]]
impl Ppu
{
	pub fn new(title: String, window_width: u32, window_height: u32, sdl_context: &Sdl) -> Ppu
        {
            //let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem.window(&title.to_string(), window_width, window_height)
                    .position_centered()
                    .build()
                    .unwrap();

            let canvas = window.into_canvas().build().unwrap();

            Ppu
            {
                title,
                window_height,
                window_width,
                fb: [0; 64*32],
              //  sdl_context,
                canvas,
            }

        }
	pub fn update_graphics(&mut self, fb: &[[i8; 64]; 32])
	{
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        for row in 0..32
        {
            for col in  0..64
            {
                if fb[row][col] == 1
                {
                    let _ = self.canvas.set_draw_color(Color::RGB(255, 255, 255)); // White.
                    let _ = self.canvas.fill_rect(Rect::new((col  as i32) * 10, (row as i32) * 10, 10, 10));
                }
                else
                {
                    let _  = self.canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black.
                    let _  = self.canvas.fill_rect(Rect::new((col as i32) * 10, (row as i32) * 10, 10, 10));
                }
            }
        }
        self.canvas.present();

	}
}
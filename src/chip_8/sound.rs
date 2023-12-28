

#[derive(Debug)]
    pub struct Sound
    {
        sound_timer: u16,
    }

    impl Sound
    {
        pub fn new() -> Sound
        {
            Sound
            {
                sound_timer: 0,
            }
        }
    }
//}
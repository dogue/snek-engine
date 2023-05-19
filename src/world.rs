use crate::{error::Error, sprite::Sprite};

pub struct World {
    window: minifb::Window,
    objects: Vec<Box<dyn Sprite>>,
    buffer: Vec<u32>,
}

impl World {
    pub fn new(
        win_title: &str,
        win_width: usize,
        win_height: usize,
        win_options: Option<minifb::WindowOptions>,
    ) -> Result<Self, Error> {
        let buffer: Vec<u32> = vec![0; win_width * win_height];

        let win_options = win_options.unwrap_or_else(|| minifb::WindowOptions::default());

        let window = minifb::Window::new(win_title, win_width, win_height, win_options)?;

        Ok(Self {
            window,
            objects: Vec::new(),
            buffer,
        })
    }
}

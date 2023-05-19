pub trait Sprite {
    fn draw(&self, buffer: &mut Vec<u32>);
}

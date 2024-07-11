pub mod aabb;
pub mod rgb;
pub mod vector;

#[derive(Default, Clone, Copy, Debug)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

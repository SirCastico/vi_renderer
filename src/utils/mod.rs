pub mod rgb;
pub mod vector;
pub mod aabb;


#[derive(Default, Clone, Copy, Debug)]
pub struct Extent2D{
    width: u32,
    height: u32
}
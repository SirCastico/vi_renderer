use crate::utils::{vector::Point, rgb::RGB};

pub mod ambient_light;



pub trait Light{
    fn l() -> RGB;
    fn l_point(p: &Point) -> RGB;
    fn sample_l();
    fn sample_l_pdf();
    fn pdf();
}

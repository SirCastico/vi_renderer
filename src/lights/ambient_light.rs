use crate::utils::{rgb::RGB, vector::Point};

use super::Light;



pub struct AmbientLight{
    color: RGB
}

impl Light for AmbientLight{
    fn l() -> RGB {
        todo!();
    }
    fn l_point(p: &Point) -> RGB {
        todo!();
    }
    fn sample_l() {
        todo!();
    }
    fn sample_l_pdf() {
        todo!();
    }
    fn pdf() {
        todo!();
    }
}

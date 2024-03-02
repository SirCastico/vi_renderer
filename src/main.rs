use crate::utils::rgb;

mod utils;
mod camera;
mod rays;

fn main() {
    let r1 = rgb::RGB::default();
    let r2 = rgb::RGB::new(3.,2.,1.0);
    println!("{:?}", r1+r2);
}


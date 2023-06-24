mod triangle;
mod v3;

use crate::v3::V3;

fn main() {
    let v = V3::create(1.0, 0.0, 0.0);
    println!("My vec: {:?}", v);
}

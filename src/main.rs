#![feature(generic_const_exprs)]

use siunits::si::Si;
use siunits::units::*;

fn main() {
    println!(
        "{:?}",
        Si::<i64, NEWTON>(3) * Si::<i64, NEWTON>(3)
    );
}

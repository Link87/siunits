#![feature(generic_const_exprs)]

use siunits::si::Si;
use siunits::unit::*;

fn main() {
    println!(
        "{:?}",
        Si::<i64, NEWTON>(3) * Si::<i64, NEWTON>(3)
    );
}

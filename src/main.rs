#![feature(generic_const_exprs)]

use siunits::si::Si;
use siunits::unit::*;

fn main() {
    println!(
        "{:?}",
        (Si::<f64, NEWTON>(3.0) + Si::<f64, NEWTON>(3.0)).num()
    );
    println!(
        "{:?}",
        (Si::<f64, NEWTON>(3.0) * Si::<f64, NEWTON>(3.0)).num()
    );
    println!("{:?}", (Si::<i64, NEWTON>(10) % Si::<i64, NEWTON>(3)).num());
    println!("{:?}", (Si::<i64, NEWTON>(128) << 3).num());

    // TODO allow this:
    // needs to swap arguments and implement from
    // let val: Si<NEWTON> = 3.0;

    let mut val: Si = Si(3.0);
    val *= Si(5.832);
    println!("{:?}", val);

    // Expect compiler error:
    // val %= Si::<f64, NEWTON>(4.0);

    val %= Si(4.0);
    println!("{:?}", val);
}

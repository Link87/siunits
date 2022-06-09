#![feature(generic_const_exprs)]

use siunits::si::Si;
use siunits::unit::*;

fn main() {
    println!(
        "{:?}",
        (Si::new::<NEWTON>(3.0) + Si::new::<NEWTON>(3.0)).num()
    );
    println!(
        "{:?}",
        (Si::new::<NEWTON>(3.0) * Si::new::<NEWTON>(3.0)).num()
    );
    println!("{:?}", (Si::new::<NEWTON>(10) % Si::new::<NEWTON>(3)).num());
    println!("{:?}", (Si::new::<NEWTON>(128) << 3).num());

    // TODO allow this:
    // needs to swap arguments
    // let val: Si<NEWTON> = Si::from(3.0)
    let mut val: Si = Si::from(3.0);

    // Expect compiler error:
    // val *= Si::new::<NEWTON>(5.832);
    val *= Si::new(5.832);
    println!("{:?}", val);

    // val %= Si::<f64, NEWTON>(4.0);

    val %= Si::new(4.0);
    println!("{:?}", val);

    let mut val = Si::new::<METER>(27.0) * Si::new::<NEWTON>(5000.0);
    println!("{:?}", val);
    val += Si::new::<JOULE>(14.98 * 1000.0);
    println!("{:?}", val);
}

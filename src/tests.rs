use core::panic;

use crate::si::*;
use crate::unit::*;

#[test]
fn addition() {
    let sum = (Si::new::<NEWTON>(3.0) + Si::new::<NEWTON>(3.0)).val();
    assert_eq!(sum, 3.0 + 3.0)
}

#[test]
fn multiplication() {
    let prod = (Si::new::<NEWTON>(3.0) * Si::new::<NEWTON>(3.0)).val();
    assert_eq!(prod, 3.0 * 3.0)
}

#[test]
fn remainder() {
    let rem = (Si::new::<NEWTON>(10) % Si::new::<NEWTON>(3)).val();
    assert_eq!(rem, 1)
}
#[test]
fn shift() {
    let res = (Si::new::<NEWTON>(128) << 3).val();
    assert_eq!(res, 1024)
}

#[test]
fn assign_ops() {
    let mut val_si: Si = Si::from(3.0);
    // compiler error expected: val *= Si::new::<NEWTON>(5.832);
    val_si *= Si::new(5.832);

    let mut val = 3.0;
    val *= 5.832;
    
    assert_eq!(val_si.val(), val);
}

#[test]
fn rem_assign() {
    let mut val_si: Si = Si::from(17.23412);
    // compiler error expected: val %= Si::<f64, NEWTON>(4.0);
    val_si %= Si::new(4.0);
    
    let mut val = 17.23412;
    val %= 4.0;

    assert_eq!(val_si.val(), val);
}

#[test]
fn dim_change() {
    let mut val = Si::new::<METER>(27.0) * Si::new::<NEWTON>(5000.0);
    assert_eq!(val.val(), 27.0 * 5000.0);
    val += Si::new::<JOULE>(14.98 * 1000.0);
    assert_eq!(val.val(), (27.0 * 5000.0) + (14.98 * 1000.0));
}

#[test]
fn creation() {

    panic!("not implemented yet");
    // TODO allow this:
    // needs to swap arguments
    // let val: Si<NEWTON> = Si::from(3.0)

}

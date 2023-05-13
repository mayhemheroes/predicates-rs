#![no_main]
use libfuzzer_sys::fuzz_target;
use predicates::{prelude::*, ord::{lt, ge, gt, le}};

fuzz_target!(|value: (u8, u8, u8)| {
    let (a, b, c) = value;
    
    //Either less than both numbers or greater or equal than either number
    assert!(lt(b).and(lt(c)).or(ge(b).or(ge(c))).eval(&a));

    //any number is either greater than or less than or equal to any other number
    assert!(gt(b).or(le(b)).eval(&a));

    //any number is either greater than or equal to, or less than any other number
    assert!(ge(b).or(lt(b)).eval(&a));
});
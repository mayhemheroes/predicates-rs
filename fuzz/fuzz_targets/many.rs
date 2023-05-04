#![no_main]
use libfuzzer_sys::fuzz_target;
use predicates::prelude::*;
use predicates::str::{starts_with, ends_with, contains, is_empty};

// TODO: add docs about arbitrary
fuzz_target!(|value: (u8, &str, &str, &str)| {
    let (a, b, c, d) = value;
    let always_true = predicate::always();
    assert_eq!(true, always_true.eval(&a));
    starts_with(b.clone()).and(ends_with(c)).eval(d);
    contains(b).or(is_empty()).eval(c);
});
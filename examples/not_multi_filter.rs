use std::fmt::Display;

use arbitrary::{Arbitrary, Unstructured};
use primitive_operators::{
    Inclusion,
    operators::{And, Not},
};
use rand::{Rng, rng};
use strum_macros::VariantArray;

#[derive(PartialEq, Arbitrary, VariantArray, Copy, Clone, Debug)]
enum Bruh {
    Dude,
    Guy,
    Fr,
    Helen,
}
impl Display for Bruh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Bruh::Dude => "Dude",
            Bruh::Guy => "Guy",
            Bruh::Fr => "Fr",
            Bruh::Helen => "Helen",
        })
    }
}
#[derive(Debug)]
struct DudeFilter;
impl Inclusion<Bruh> for DudeFilter {
    fn includes(&value: &Bruh) -> bool {
        value == Bruh::Dude
    }
}
#[derive(Debug)]
struct GuyFilter;
impl Inclusion<Bruh> for GuyFilter {
    fn includes(&value: &Bruh) -> bool {
        value == Bruh::Guy
    }
}
fn main() {
    let mut bytes = [0u8; 10];
    rng().fill_bytes(&mut bytes);
    let b: And<Bruh, DudeFilter, GuyFilter> =
        And::arbitrary(&mut Unstructured::new(&bytes)).unwrap();
    //let b: Not<Bruh, DudeFilter> = Not::arbitrary(&mut Unstructured::new(&bytes)).unwrap();
    assert_ne!(b, Bruh::Dude);
    println!("{}", b);
}

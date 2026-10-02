use std::fmt::Display;

use arbitrary::{Arbitrary, Unstructured};
use primitive_operators::old::{Inclusion, Not};
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
struct ExcludeDude;
impl Inclusion<Bruh> for ExcludeDude {
    fn includes(&value: &Bruh) -> bool {
        value == Bruh::Dude
    }
}
fn main() {
    let mut bytes = [0u8; 10];
    rng().fill_bytes(&mut bytes);
    let b: Not<Bruh, ExcludeDude> = Not::arbitrary(&mut Unstructured::new(&bytes)).unwrap();
    assert_ne!(b, Bruh::Dude)
}

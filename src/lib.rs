use std::marker::PhantomData;

use arbitrary::{Arbitrary, Unstructured};

#[cfg(feature = "macros")]
pub use primitive_operators_macros;

pub mod operators;

// The idea with these two traits is that they can produce an arbitrary type. This is basically
// another set of types that produce other types. Ideally they wouldn't be interacted with directly,
// but with a macro system
pub trait Constraint<T> {
    fn accepts(value: &T) -> bool;
}

pub trait Generate {
    type Output<'a>: Arbitrary<'a>;
    fn generate<'b>(u: &mut Unstructured<'_>) -> arbitrary::Result<Self::Output<'b>>;
}

pub trait Insensitive {
    const STR: &'static str;
}
/// Given a string to start with, will randomize the cases of each alphabetic character in the string on a call to arbitrary
pub struct CaseInsensitiveString<I: Insensitive>(pub String, PhantomData<I>);

impl<'a, I: Insensitive> Arbitrary<'a> for CaseInsensitiveString<I> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut res = String::with_capacity(I::STR.len());
        for c in I::STR.chars() {
            if !c.is_alphabetic() {
                res.push(c);
                continue;
            }
            if u.arbitrary()? {
                res.extend(c.to_uppercase());
            } else {
                res.extend(c.to_lowercase());
            }
        }
        Ok(CaseInsensitiveString(res, PhantomData))
    }
}

/// Trait that describes what enum variants to exclude.
pub trait Inclusion<T> {
    /// Indicates whether the input is a valid variant to be generated using `arbitrary::Arbitrary`.
    /// Returns true if the input is a legal type variant
    /// Returns false if the input is not a legal type variant
    fn includes(value: &T) -> bool;
}

#[cfg(test)]
mod test {
    /// The number of times to iterate and test that an exclusion/inclusion works before assuming
    /// that it works on all cases
    const TIMES: usize = 1000;
    use std::fmt::Display;

    use arbitrary::{Arbitrary, Unstructured};
    use primitive_operators_macros::not;
    use rand::{Rng, rng};
    use strum_macros::VariantArray;

    use crate::{
        Inclusion,
        operators::{Not, Only},
    };

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

    /// Tests the `Not` struct without using the macro
    #[test]
    fn not_direct_test() {
        for _ in 0..TIMES {
            let mut bytes = [0u8; 10];
            rng().fill_bytes(&mut bytes);
            let b: Not<Bruh, DudeFilter> = Not::arbitrary(&mut Unstructured::new(&bytes)).unwrap();
            assert_ne!(b, Bruh::Dude)
        }
    }
    /// Tests the `Only` struct without using the macro
    #[test]
    fn only_direct_test() {
        for _ in 0..TIMES {
            let mut bytes = [0u8; 10];
            rng().fill_bytes(&mut bytes);
            let b: Only<Bruh, DudeFilter> =
                Only::arbitrary(&mut Unstructured::new(&bytes)).unwrap();
            assert_eq!(b, Bruh::Dude)
        }
    }

    /// Tests the `Only` struct with using the macro
    #[test]
    fn only_macro_test() {
        for _ in 0..TIMES {
            let mut bytes = [0u8; 10];
            rng().fill_bytes(&mut bytes);
            //let b:  = USE ONLY MACRO HERE
            //assert_eq!(b, Bruh::Dude)
        }
    }

    #[test]
    fn not_macro() {
        not!(NotDude = Bruh except Dude);

        for _ in 0..TIMES {
            let mut bytes = [0u8; 10];
            rng().fill_bytes(&mut bytes);
            assert_ne!(
                NotDude::arbitrary(&mut Unstructured::new(&bytes)).unwrap(),
                Bruh::Dude
            );
        }
    }
}

use arbitrary::{Arbitrary, Unstructured};

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

// These types are interoperable with Arbitrary, being their own types but impl PartialEq<T> where T
// is the type they're wrapping. This would also need a macro system to not be so verbose.
//
pub mod old {
    use std::{
        fmt::{Debug, Display},
        marker::PhantomData,
    };

    use arbitrary::Arbitrary;
    use strum::VariantArray;

    /// Trait that describes what enum variants to exclude.
    pub trait Inclusion<T> {
        /// Indicates whether the input is a valid variant to be generated using `arbitrary::Arbitrary`.
        /// Returns true if the input is a legal type variant
        /// Returns false if the input is not a legal type variant
        fn includes(value: &T) -> bool;
    }

    /// Takes in two Exclusions and computes the OR between their `includes` functions
    pub struct Or<A, B>(PhantomData<(A, B)>);
    impl<T, A, B> Inclusion<T> for Or<A, B>
    where
        A: Inclusion<T>,
        B: Inclusion<T>,
    {
        fn includes(value: &T) -> bool {
            A::includes(value) || B::includes(value)
        }
    }

    /// Takes in two Exclusions and computes the AND between their `includes` functions
    pub struct And<A, B>(PhantomData<(A, B)>);
    impl<T, A, B> Inclusion<T> for And<A, B>
    where
        A: Inclusion<T>,
        B: Inclusion<T>,
    {
        fn includes(value: &T) -> bool {
            A::includes(value) && B::includes(value)
        }
    }

    /// Takes in two Exclusions and computes the XOR between their `includes` functions
    pub struct XOr<A, B>(PhantomData<(A, B)>);
    impl<T, A, B> Inclusion<T> for XOr<A, B>
    where
        A: Inclusion<T>,
        B: Inclusion<T>,
    {
        fn includes(value: &T) -> bool {
            A::includes(value) ^ B::includes(value)
        }
    }
    /// Takes in an enum and a struct that implements a type marker using
    pub struct Not<T: VariantArray + Debug, E>(T, PhantomData<E>);
    impl<'a, T, E> Arbitrary<'a> for Not<T, E>
    where
        T: VariantArray + PartialEq + Copy + Debug,
        E: Inclusion<T>,
    {
        fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
            let vars: Vec<T> = T::VARIANTS
                .iter()
                .copied()
                .filter(|t| !E::includes(t))
                .collect();
            let &var = u.choose(&vars)?;
            Ok(Not(var, PhantomData))
        }
    }

    impl<T, E> PartialEq<T> for Not<T, E>
    where
        T: VariantArray + PartialEq + Copy + Display + Debug,
        E: Inclusion<T>,
    {
        fn eq(&self, &other: &T) -> bool {
            self.0 == other
        }
    }
    impl<T, E> Display for Not<T, E>
    where
        T: VariantArray + PartialEq + Copy + Display + Debug,
        E: Inclusion<T>,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&self.0, f)
        }
    }
}

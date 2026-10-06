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
    #[derive(Debug)]
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

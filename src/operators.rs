use std::fmt::{Debug, Display};
use std::marker::PhantomData;

use arbitrary::{Arbitrary, Unstructured};
use strum::VariantArray;

use crate::Inclusion;

pub struct True<T>(PhantomData<T>);
impl<T> Inclusion<T> for True<T> {
    fn includes(_value: &T) -> bool {
        true
    }
}
pub struct False<T>(PhantomData<T>);
impl<T> Inclusion<T> for False<T> {
    fn includes(_value: &T) -> bool {
        false
    }
}
/// Takes in two Inclusions and computes the OR between their `includes` functions
#[derive(Debug)]
pub struct Or<T, A, B>(pub T, PhantomData<(A, B)>);
impl<T, A, B> Inclusion<T> for Or<T, A, B>
where
    A: Inclusion<T>,
    B: Inclusion<T>,
{
    fn includes(value: &T) -> bool {
        A::includes(value) || B::includes(value)
    }
}
impl<T, A, B> Display for Or<T, A, B>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<'a, T, A, B> Arbitrary<'a> for Or<T, A, B>
where
    T: VariantArray + Copy,
    A: Inclusion<T>,
    B: Inclusion<T>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let allowed: Vec<T> = T::VARIANTS
            .iter()
            .copied()
            .filter(|v| A::includes(v) || B::includes(v))
            .collect();
        Ok(Self(u.choose_iter(allowed.into_iter())?, PhantomData))
    }
}

impl<T, A, B> PartialEq<T> for Or<T, A, B>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

/// Takes in two Inclusions and computes the AND between their `includes` functions
#[derive(Debug)]
pub struct And<T, A, B>(pub T, PhantomData<(A, B)>);
impl<T, A, B> Inclusion<T> for And<T, A, B>
where
    A: Inclusion<T>,
    B: Inclusion<T>,
{
    fn includes(value: &T) -> bool {
        A::includes(value) && B::includes(value)
    }
}
impl<'a, T, A, B> Arbitrary<'a> for And<T, A, B>
where
    T: VariantArray + Copy,
    A: Inclusion<T>,
    B: Inclusion<T>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let allowed: Vec<T> = T::VARIANTS
            .iter()
            .copied()
            .filter(|v| A::includes(v) && B::includes(v))
            .collect();
        Ok(Self(u.choose_iter(allowed.into_iter())?, PhantomData))
    }
}

impl<T, A, B> Display for And<T, A, B>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<T, A, B> PartialEq<T> for And<T, A, B>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}
impl<T, A, B> PartialEq<&T> for And<T, A, B>
where
    T: PartialEq,
{
    fn eq(&self, other: &&T) -> bool {
        self.0.eq(other)
    }
}
/// Takes in two Inclusions and computes the XOR between their `includes` functions
#[derive(Debug)]
pub struct XOr<T, A, B>(pub T, PhantomData<(A, B)>);
impl<T, A, B> Inclusion<T> for XOr<T, A, B>
where
    A: Inclusion<T>,
    B: Inclusion<T>,
{
    fn includes(value: &T) -> bool {
        A::includes(value) ^ B::includes(value)
    }
}
impl<'a, T, A, B> Arbitrary<'a> for XOr<T, A, B>
where
    T: VariantArray + Copy,
    A: Inclusion<T>,
    B: Inclusion<T>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let allowed: Vec<T> = T::VARIANTS
            .iter()
            .copied()
            .filter(|v| A::includes(v) ^ B::includes(v))
            .collect();
        Ok(Self(u.choose_iter(allowed.into_iter())?, PhantomData))
    }
}

impl<T, A, B> Display for XOr<T, A, B>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<T, A, B> PartialEq<T> for XOr<T, A, B>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}
/// Takes in an enum and a struct that implements a type marker using
#[derive(Debug)]
pub struct Not<T: VariantArray + Debug, E>(pub T, PhantomData<E>);
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
impl<T, E> Inclusion<T> for Not<T, E>
where
    T: VariantArray + Debug,
    E: Inclusion<T>,
{
    fn includes(value: &T) -> bool {
        !E::includes(value)
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
impl<T, A> PartialEq<&T> for Not<T, A>
where
    T: PartialEq + VariantArray + Debug,
{
    fn eq(&self, other: &&T) -> bool {
        self.0.eq(other)
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
#[derive(Debug)]
pub struct Only<T, E>(pub T, PhantomData<E>);

impl<'a, T, E> Arbitrary<'a> for Only<T, E>
where
    T: VariantArray + Copy,
    E: Inclusion<T>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let vars: Vec<T> = T::VARIANTS.iter().copied().filter(E::includes).collect();

        let &variant = u.choose(&vars)?;

        Ok(Self(variant, PhantomData))
    }
}

impl<T, E> PartialEq<T> for Only<T, E>
where
    T: VariantArray + PartialEq + Copy + Display + Debug,
    E: Inclusion<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T, E> Display for Only<T, E>
where
    T: VariantArray + PartialEq + Copy + Display + Debug,
    E: Inclusion<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

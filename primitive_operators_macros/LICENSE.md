This is a crate that has basic operations for providing constraints on types, enums for now, that implement arbitrary::Arbitrary. 

The plan is to have this crate to allow for constraints on different custom data types for when `arbitrary:Arbitrary` is used.



# Usage
## Operators
Operators are different structs that allow for constraining what variants of an enum are produced. There are five: `Not`, `Only`, `Or`, `And`, and `XOr`. I don't really know how Or or XOr can be used here, but I figured they're a logical extension of nots and ands, so I implemented them.
These structs can be used to create logic around which enum variants can possibly be produced by `arbitrary::Arbitrary`. I have a few examples in the examples directory and short code snippets further in the README. Each one of the `Inclusion` structs takes in a type and another type that implements `Inclusion`. Each of these main types wrap around an enum variant that's exposed to do what the caller wants with it. The zero sized type used to determine whether a variant is included I'll call filters. A basic filter would look like this:
```rust
#[derive(PartialEq, Arbitrary, VariantArray, Copy, Clone, Debug)]
enum Bruh {
    Dude,
    Guy,
    Fr,
    Helen,
}

#[derive(Debug)]
struct DudeFilter;
impl Inclusion<Bruh> for DudeFilter {
    fn includes(&value: &Bruh) -> bool {
        value == Bruh::Dude
    }
}
```
`DudeFilter` just returns whether or not a variant is included. They can then be used as input for `Not` or `Only`, arbitrarily producing an enum variant with the desired constraint:
```rust
let generated_var: Only<Bruh, DudeFilter> = Only::arbitrary(&mut Unstructured::new(&[0])).unwrap();
assert_eq!(generated_var.0,Bruh::Dude);
```
A similar structure can be achieved with `Not`:
```rust
let generated_var: Not<Bruh, DudeFilter> = Not::arbitrary(&mut Unstructured::new(&[0])).unwrap();
assert_ne!(generated_var.0,Bruh::Dude);
```

## Examples

If you have an enum `Bruh`
```rust
enum Bruh{
    Dude, Guy, Fr
}
```

But you want to use Arbitrary to generate variants of, but you only want to generate Dude, you can write this code:
```rust
only!(OnlyDude = Bruh only Dude)
```
or if you want a public type that only produces `Dude` and `Guy`, because you're not real enough `Fr`:
```rust
only!(pub OnlyDudeOrGuy = Bruh only [Dude, Guy])
```

The two types produced here are:
```rust
type OnlyDude = Only<Bruh,DudeFilter>; // where DudeFilter is a generated type to filter out Dude
pub type OnlyDudeOrGuy = And<Bruh,DudeFilter,GuyFilter>;
```

We can also do a similar thing with excluding variants from being generated:
```rust
not!(NotDude = Bruh except Dude);
```
where this code creates a new type, NotDude that can't produce `Bruh::Dude`, or

```rust
not!(NotGuyOrDude = Bruh except [Dude, Guy]);
```
where this code creates a new types NotGuyOrDude that can only produce `Bruh::Fr`.

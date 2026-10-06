use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{
    Ident, LitStr, Result, Token, Type, Visibility, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Bracket,
};

struct InsensitiveTypeInput {
    visibility: Visibility,
    name: Ident,
    value: LitStr,
}

impl Parse for InsensitiveTypeInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        let name: Ident = input.parse()?;

        input.parse::<Token![=]>()?;

        let value: LitStr = input.parse()?;

        Ok(Self {
            visibility,
            name,
            value,
        })
    }
}

/// Given a type name and a string, will generate a type that implements Arbitrary that is case
/// insensitive over all alphabetic characters given in the input string
/// # Syntax
/// ```
/// insensitive_type!(visibility GeneratedType = "Case Insensitive String Here")
/// ```
/// # Examples:
/// This is an example of creating a new type, `ContentType` that implements `Arbitrary`
/// ```rust
/// insensitive_type!(pub ContentType = "Content-Type");
/// ```
///
/// Another example, but without the `pub` modifier
/// ```rust
/// insensitive_type!(ContentType = "Content-Type");
/// ```
#[proc_macro]
pub fn insensitive_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InsensitiveTypeInput);

    let visibility = input.visibility;
    let name = input.name;
    let value = input.value;

    let string_name = format_ident!("{}String", name, span = Span::call_site());

    quote! {
        struct #string_name;

        impl ::primitive_operators::Insensitive for #string_name {
            const STR: &'static str = #value;
        }

        #visibility type #name =
            ::primitive_operators::CaseInsensitiveString<#string_name>;
    }
    .into()
}

mod kw {
    syn::custom_keyword!(except);
    syn::custom_keyword!(only);
}
enum Variants {
    Punctuated(Punctuated<Ident, Token![,]>),
    Single(Ident),
}
impl From<Punctuated<Ident, Token![,]>> for Variants {
    fn from(value: Punctuated<Ident, Token![,]>) -> Self {
        Self::Punctuated(value)
    }
}
impl From<Ident> for Variants {
    fn from(value: Ident) -> Self {
        Self::Single(value)
    }
}
impl Parse for Variants {
    fn parse(input: ParseStream) -> Result<Self> {
        let excluded: Variants = if input.peek(Bracket) {
            let content;
            bracketed!(content in input);
            Punctuated::<Ident, Token![,]>::parse_terminated(&content)?.into()
        } else {
            input.parse::<Ident>()?.into()
        };
        Ok(excluded)
    }
}
struct NotVariantsInput {
    visibility: Visibility,
    out_name: Ident,
    parent_enum: Type,
    variants: Variants,
}

impl Parse for NotVariantsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = input.parse()?;

        let out_name = input.parse()?;
        input.parse::<Token![=]>()?;

        let parent_enum = input.parse()?;
        input.parse::<kw::except>()?;
        let variants = input.parse()?;

        Ok(Self {
            visibility,
            out_name,
            parent_enum,
            variants,
        })
    }
}
/// Returns a type that generates all enum variants besides the provided one
/// # Syntax:
/// ```rust
///not!(NotDude =  Bruh except Dude);
///not!(OnlyGuy = Bruh except [Dude, Bro]);
///```
///# Possible Issue
///- The generated filters with this macro might pollute a little bit, especially if you have two
///  different enums with variants that have the same name. This is a TODO item.
#[proc_macro]
pub fn not(input: TokenStream) -> TokenStream {
    let NotVariantsInput {
        visibility,
        out_name,
        parent_enum,
        variants,
    } = parse_macro_input!(input as NotVariantsInput);

    let variants: Vec<Ident> = match variants {
        Variants::Punctuated(punctuated) => punctuated.into_iter().collect(),
        Variants::Single(ident) => vec![ident],
    };

    if variants.is_empty() {
        return syn::Error::new_spanned(out_name, "one excluded variant is needed.")
            .to_compile_error()
            .into();
    }
    let filter_names: Vec<Ident> = variants
        .iter()
        .map(|v| format_ident!("__{}Filter", v))
        .collect();
    let filter_defs: Vec<proc_macro2::TokenStream> = filter_names
        .iter()
        .zip(variants.iter())
        .map(|(filter_name, variant)| {
            quote! {
                #[derive(Debug)]
                struct #filter_name;
                impl Inclusion<#parent_enum> for #filter_name{
                    fn includes(&value:&#parent_enum) -> bool{
                        value == #parent_enum::#variant
                    }
                }
            }
        })
        .collect();
    let nots = filter_names.iter().map(|filter_name| {
        quote! {
            Not<#parent_enum,#filter_name>
        }
    });
    let out_type = if filter_names.len() == 1 {
        let filter = &filter_names[0];
        quote! {Not<#parent_enum,#filter>}
    } else {
        let not_types: Vec<proc_macro2::TokenStream> = nots.collect();

        not_types[1..]
            .iter()
            .fold(not_types[0].clone(), |acc, next| {
                quote! {And<#parent_enum,#acc,#next>}
            })
    };
    quote! {
        #(#filter_defs)*

        #visibility type #out_name = #out_type;
    }
    .into()
}

struct OnlyVariantsInput {
    visibility: Visibility,
    out_name: Ident,
    parent_enum: Type,
    variants: Variants,
}

impl Parse for OnlyVariantsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = input.parse()?;

        let out_name = input.parse()?;
        input.parse::<Token![=]>()?;

        let parent_enum = input.parse()?;
        input.parse::<kw::only>()?;
        let variants = input.parse()?;

        Ok(Self {
            visibility,
            out_name,
            parent_enum,
            variants,
        })
    }
}
/// Returns a type that generates all enum variants besides the provided one
/// # Syntax:
/// ```rust
///only!(NotDude =  Bruh only Dude);
///only!(OnlyGuy = Bruh only [Dude, Bro]);
///```
///# Possible Issue
///- The generated filters with this macro might pollute a little bit, especially if you have two
///  different enums with variants that have the same name. This is a TODO item.
#[proc_macro]
pub fn only(input: TokenStream) -> TokenStream {
    let OnlyVariantsInput {
        visibility,
        out_name,
        parent_enum,
        variants,
    } = parse_macro_input!(input as OnlyVariantsInput);

    let variants: Vec<Ident> = match variants {
        Variants::Punctuated(punctuated) => punctuated.into_iter().collect(),
        Variants::Single(ident) => vec![ident],
    };

    if variants.is_empty() {
        return syn::Error::new_spanned(out_name, "one included variant is needed.")
            .to_compile_error()
            .into();
    }
    let filter_names: Vec<Ident> = variants
        .iter()
        .map(|v| format_ident!("__{}Filter", v))
        .collect();
    let filter_defs: Vec<proc_macro2::TokenStream> = filter_names
        .iter()
        .zip(variants.iter())
        .map(|(filter_name, variant)| {
            quote! {
                #[derive(Debug)]
                struct #filter_name;
                impl Inclusion<#parent_enum> for #filter_name{
                    fn includes(&value:&#parent_enum) -> bool{
                        value == #parent_enum::#variant
                    }
                }
            }
        })
        .collect();
    let nots = filter_names.iter().map(|filter_name| {
        quote! {
            Only<#parent_enum,#filter_name>
        }
    });
    let out_type = if filter_names.len() == 1 {
        let filter = &filter_names[0];
        quote! {Only<#parent_enum,#filter>}
    } else {
        let not_types: Vec<proc_macro2::TokenStream> = nots.collect();

        not_types[1..]
            .iter()
            .fold(not_types[0].clone(), |acc, next| {
                quote! {And<#parent_enum,#acc,#next>}
            })
    };
    quote! {
        #(#filter_defs)*

        #visibility type #out_name = #out_type;
    }
    .into()
}

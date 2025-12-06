use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::{Comma, Paren}, Data, DeriveInput, Error, Expr, ExprLit, ExprTuple, Ident, Lit, Type};

// call as such: panic_span!(something.span(), "Error message"); in a function that returns
// a TokenStream
macro_rules! panic_span {
    ($span:expr, $message:literal) => {
        return Error::new($span, $message).to_compile_error().into()
    };
}

#[proc_macro_attribute]
pub fn char_enum(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(annotated_item as DeriveInput);
    if item.generics.lifetimes().count() != 0
        || item.generics.type_params().count() != 0
        || item.generics.const_params().count() != 0 {
            panic_span!(item.generics.span(), "Generics are not supported");
    }
    match item.data {
        Data::Enum(enum_data) => {
            let vis = item.vis;
            let ident = item.ident;
            let variants = enum_data.variants;

            let mut data = vec![];

            for variant in variants {
                if !variant.fields.is_empty() {
                    panic_span!(variant.fields.span(), "Fields are not supported");
                }
                match variant.discriminant {
                    Some((_, expr)) => {
                        if let Expr::Lit(literal) = expr { // = 'A'
                            if let Lit::Char(chr) = literal.lit {
                                data.push((variant.attrs, variant.ident, chr.token(), None));
                            } else {
                                panic_span!(literal.span(), "Expected character literal");
                            }
                        } else if let Expr::Tuple(tuple) = expr { // = ('A', ...)
                            let tuple_span = tuple.span();
                            let mut iter = tuple.elems.into_iter();
                            let Some(first) = iter.next() else {
                                panic_span!(tuple_span, "Expected element in tuple");
                            };
                            let remaining: Punctuated<Expr, Comma> = iter.collect();
                            let remaining = if remaining.len() == 1 {
                                remaining.into_iter().next().unwrap()
                            } else {
                                Expr::Tuple(ExprTuple {
                                    attrs: tuple.attrs,
                                    elems: remaining,
                                    paren_token: Paren(tuple_span)
                                })
                            };

                            if let Expr::Lit(ExprLit {lit: Lit::Char(chr), ..}) = first {
                                data.push((variant.attrs, variant.ident, chr.token(), Some(remaining)));
                            } else {
                                panic_span!(first.span(), "Expected character literal")
                            }
                        } else {
                            panic_span!(expr.span(), "Expected character literal");
                        }
                    },
                    None => panic_span!(variant.span(), "Must include = '<char>'")
                }
            }

            let top_level_attrs = item.attrs;

            let identifiers = data.iter().map(|(attrs, id, _, remaining)| match remaining {
                Some(remaining) => quote! {
                    #(
                        #attrs
                    )*
                    #id = #remaining,
                },
                None => quote!{
                    #(
                        #attrs
                    )*
                    #id,
                }
            });
            let char_to_ident = data.iter().map(|(_, id, literal, _)| quote!{#literal => #ident::#id});
            let char_to_ok_ident = data.iter().map(|(_, id, literal, _)| quote!{#literal => Ok(#ident::#id)});
            let ident_to_char = data.iter().map(|(_, id, literal, _)| quote!{#ident::#id => #literal});
            let has_encode_decode = Ident::new(&(ident.to_string() + "__HasEncodeDecode__"), ident.span());

            TokenStream::from(quote!{
                #(
                    #top_level_attrs
                )*
                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                #vis enum #ident {
                    #( #identifiers )*
                }

                #[automatically_derived]
                #[allow(non_camel_case_types)]
                #vis trait #has_encode_decode {
                    fn decode(chr: char) -> #ident;
                    fn try_decode(chr: char) -> Result<#ident, char>;
                    fn encode(&self) -> char;
                }

                #[automatically_derived]
                impl #has_encode_decode for #ident {
                    fn decode(chr: char) -> #ident {
                        match chr {
                            #( #char_to_ident, )*
                            _ => panic!("Unknown character `{}`", chr)
                        }
                    }

                    fn try_decode(chr: char) -> Result<#ident, char> {
                        match chr {
                            #( #char_to_ok_ident, )*
                            _ => Err(chr)
                        }
                    }

                    fn encode(&self) -> char {
                        match self {
                            #( #ident_to_char, )*
                        }
                    }
                }

                #[automatically_derived]
                impl From<char> for #ident {
                    fn from(c: char) -> #ident {
                        #ident::decode(c)
                    }
                }

                #[automatically_derived]
                impl From<#ident> for char {
                    fn from(v: #ident) -> char {
                        #ident::encode(&v)
                    }
                }
            })
        },
        _ => panic!("char_enum can only be applied to enums")
    }
}

#[proc_macro_attribute]
pub fn data_enum(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    //let input: Vec<TokenTree> = input.into_iter().collect();
    let input = parse_macro_input!(input as Type);
    let item = parse_macro_input!(annotated_item as DeriveInput);

    /*if input.len() > 1 {
        panic_span!(input.last().unwrap().span().into(), "Too many parameters");
    } else if input.len() == 0 {
        panic_span!(item.ident.span(), "Expected exactly one type parameter");
    }*/

    if item.generics.lifetimes().count() != 0
        || item.generics.type_params().count() != 0
        || item.generics.const_params().count() != 0 {
            panic_span!(item.generics.span(), "Generics are not supported");
    }
    match item.data {
        Data::Enum(enum_data) => {
            let vis = item.vis;
            let ident = item.ident;
            let variants = enum_data.variants;

            let mut data = vec![];

            for variant in variants {
                if !variant.fields.is_empty() {
                    panic_span!(variant.fields.span(), "Fields are not supported");
                }
                match variant.discriminant {
                    Some((_, expr)) => {
                        data.push((variant.attrs, variant.ident, expr));
                    },
                    None => panic_span!(variant.span(), "Must include = <VALUE>")
                }
            }

            let top_level_attrs = item.attrs;

            let identifiers = data.iter().map(|(attrs, id, _)| quote!{
                #(
                    #attrs
                )*
                #id,
            });
            let ident_to_value = data.iter().map(|(_, id, expr)| quote!{#ident::#id => #expr});
            let has_value = Ident::new(&(ident.to_string() + "__HasValue__"), ident.span());

            TokenStream::from(quote!{
                #(
                    #top_level_attrs
                )*
                #vis enum #ident {
                    #( #identifiers )*
                }

                #[automatically_derived]
                #[allow(non_camel_case_types)]
                #vis trait #has_value {
                    fn value(&self) -> #input;
                }

                #[automatically_derived]
                impl #has_value for #ident {
                    fn value(&self) -> #input {
                        match self {
                            #( #ident_to_value, )*
                        }
                    }
                }
            })
        },
        _ => panic!("data_enum can only be applied to enums")
    }
}


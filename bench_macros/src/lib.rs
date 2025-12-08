use std::fs;

use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Error, LitInt, LitStr};

// call as such: panic_span!(something.span(), "Error message"); in a function that returns
// a TokenStream
macro_rules! panic_span {
    ($span:expr, $message:expr) => {
        return Error::new($span, $message).to_compile_error().into()
    };
}

#[proc_macro]
pub fn sneaky_include(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let file_path = input.value();

    let file_content = match fs::read_to_string(&file_path) {
        Ok(fc) => fc,
        Err(e) => panic_span!(input.span(), format!("Failed to read file '{}': {}", file_path, e)),
    };

    match syn::parse_str(&file_content) {
        Ok(p) => p,
        Err(e) => e.to_compile_error(),
    }.into()
}

#[proc_macro]
pub fn setup_day(input: TokenStream) -> TokenStream {
    match setup_day_(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn read_file(span: proc_macro2::Span, path: &str) -> Result<String, Error> {
    fs::read_to_string(path).map_err(move |e| Error::new(span, format!("Failed to read file '{}': {}", path, e)))
}

#[inline]
fn setup_day_(input: TokenStream) -> Result<TokenStream, Error> {
    let input = syn::parse::<LitInt>(input)?;
    let day_num: usize = input.base10_parse()?;
    let day = format_ident!("day{:0>2}", day_num);
    let modname = format_ident!("{}mock", day);

    let main_rs = format!("src/bin/{}/main.rs", day);
    let main_rs = read_file(input.span(), &main_rs)?;
    let main_rs = main_rs.replace("cfg!(test)", "/*cfg!(test)*/ false");
    let main_rs = syn::parse_str::<proc_macro2::TokenStream>(&main_rs)?;

    let input_txt = format!("src/bin/{}/input.txt", day);
    let input_txt = read_file(input.span(), &input_txt)?;
    let input_txt = input_txt.trim_matches('\n');

    let bench_fn1 = format_ident!("bench_{}_part1", day);
    let bench_fn2 = format_ident!("bench_{}_part2", day);
    let bench_lbl1 = LitStr::new(&format!("Day {} Part 1", day_num), Span::call_site().into());
    let bench_lbl2 = LitStr::new(&format!("Day {} Part 2", day_num), Span::call_site().into());

    Ok(quote! {
        mod #modname {
            pub(super) const MOCK_INPUT: &str = #input_txt;

            #main_rs

            #[inline]
            pub(super) fn part1_mock(data: &str) -> usize {
                part1(data)
            }

            #[inline]
            pub(super) fn part2_mock(data: &str) -> usize {
                part2(data)
            }
        }

        fn #bench_fn1(c: &mut criterion::Criterion) {
            c.bench_function(#bench_lbl1, |b| b.iter(|| #modname::part1_mock(std::hint::black_box(#modname::MOCK_INPUT))));
        }

        fn #bench_fn2(c: &mut criterion::Criterion) {
            c.bench_function(#bench_lbl2, |b| b.iter(|| #modname::part2_mock(std::hint::black_box(#modname::MOCK_INPUT))));
        }
    }.into())
}

#[proc_macro]
pub fn setup_up_to(input: TokenStream) -> TokenStream {
    match setup_up_to_(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

#[inline]
fn setup_up_to_(input: TokenStream) -> Result<TokenStream, Error> {
    let input = syn::parse::<LitInt>(input)?;
    let max_day: usize = input.base10_parse()?;
    let days = (1..=max_day).collect::<Vec<_>>();
    let bench_fns = days.iter().flat_map(|&day| (1..=2).map(move |part| format_ident!("bench_day{:0>2}_part{}", day, format!("{}", part))));

    Ok(quote! {
        #( bench_macros::setup_day!(#days); )*

        criterion::criterion_group!(days, #(#bench_fns),*);
    }.into())
}

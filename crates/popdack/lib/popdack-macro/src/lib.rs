use std::clone;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn lexer(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let cloned_input = input.clone();
    let name = cloned_input.ident;

    let lex_clone_impl = quote! {
        #[derive(Clone)]
        #input

        impl LexClone for #name {
            fn clone_box(&self) -> std::boxed::Box<dyn LexRule> {
                std::boxed::Box::new(std::clone::Clone::clone(self))
            }
        }
    };

    TokenStream::from(lex_clone_impl)
}

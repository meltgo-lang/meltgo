/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::clone::Clone;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitBool, parse_macro_input};

#[proc_macro_attribute]
pub fn lexer(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let cloned_input = input.clone();
    let name = cloned_input.ident;
    let name_str = name.to_string();

    let mut is_ignore = false;

    let attr_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("is_ignore") {
            let value: LitBool = meta.value()?.parse()?;
            is_ignore = value.value();
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    });

    parse_macro_input!(attr with attr_parser);

    let lex_clone_impl = quote! {
        #[derive(Clone)]
        #input

        impl LexClone for #name {
            fn clone_box(&self) -> Box<dyn LexRule> {
                Box::new(Clone::clone(self))
            }
        }

        impl LexIgnore for #name {
            fn is_ignore(&self) -> bool {
                #is_ignore
            }
        }

        impl LexDisplay for #name {
            fn get_name(&self) -> &'static str {
                #name_str
            }
        }
    };

    TokenStream::from(lex_clone_impl)
}

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{DeriveInput, Token, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn mappinger(attr: TokenStream, input: TokenStream) -> TokenStream {
    let types = parse_macro_input!(attr with Punctuated::<Type, Token![,]>::parse_terminated);

    for ty in &types {
        if let Type::Reference(type_ref) = ty {
            return syn::Error::new(type_ref.span(), "can not use reference type.")
                .to_compile_error()
                .into();
        }
    }

    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let vis = &input.vis;
    let types_v = types.iter().map(|ty| quote! {#ty,});
    let types_i = types.iter().map(|ty| {
        let ty_str = quote!(#ty).to_string();
        quote! {#ty_str => #name::#ty,}
    });

    let expanded = quote! {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #vis enum #name {
            #(#types_v)*
        }

        impl ParserVariant<#name> for #name {
            fn variant(input: &Box<dyn LexRule>) -> #name {
                match input.get_name() {
                    #(#types_i)*
                    s => panic!("invalid token kind, {}.", s),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

//! This crate provides a derive macro for the `DeepMerge` trait.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(DeepMerge)]
/// Derive macro for the `DeepMerge` trait.
pub fn deep_merge_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_deep_merge_trait(input).into()
}

fn impl_deep_merge_trait(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident;

    let deep_merge_impl = match input.data {
        Data::Struct(data) => deep_merge_struct(data),
        Data::Enum(_finish) => panic!("DeepMerge is not supported for enums"),
        Data::Union(_) => panic!("DeepMerge is not supported for unions"),
    };

    quote! {
        impl DeepMerge for #name {
            fn deep_merge(&self, other: &Self) -> Self {
                #deep_merge_impl
            }
        }
    }
}

fn deep_merge_struct(data: DataStruct) -> proc_macro2::TokenStream {
    let fields = match data.fields {
        Fields::Named(ref fields) => {
            let field_merges: Vec<_> = fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    quote! {
                        #field_name: self.#field_name.deep_merge(&other.#field_name)
                    }
                })
                .collect();

            quote! {
                Self {
                    #(#field_merges),*
                }
            }
        }
        Fields::Unnamed(ref fields) => {
            let field_merges: Vec<_> = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    quote! {
                        self.#i.deep_merge(&other.#i),
                    }
                })
                .collect();

            quote! {
                #(#field_merges),*
            }
        }
        Fields::Unit => quote! {},
    };

    quote! {
        #fields
    }
}

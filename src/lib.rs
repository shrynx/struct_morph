//! This crate provides macro for transforming value of one struct to another.
//!
//! ```edition2021
//! # use struct_morph::{morph, morph_field};
//! #
//! # #[derive(Clone, Debug)]
//! # struct ProductRow {
//! #   id: i32,
//! #   name: String,
//! #   description: String,
//! #   available_count: i32,
//! #   base_price: i32,
//! #   discount: i32
//! # }
//! #
//! # #[derive(Debug)]
//! # #[morph(ProductRow)]
//! # struct ProductInfo {
//! #   id: i32,
//! #   #[morph_field(select = name)]
//! #   title: String,
//! #   description: String,
//! #   #[morph_field(transform = "is_available")]
//! #   is_available: bool,
//! #   #[morph_field(transform = "net_price")]
//! #   price: i32,
//! # }
//! #
//! # fn is_available(value: &ProductRow) -> bool {
//! #   value.available_count > 0
//! # }
//! #
//! # fn net_price(value: &ProductRow) -> i32 {
//! #   value.base_price - value.discount
//! # }
//! #
//! # fn main() {
//! #   let product_row: ProductRow = ProductRow {
//! #     id: 10,
//! #     name: "The Rust Programming Language".to_string(),
//! #     description: "The official book on the Rust programming language".to_string(),
//! #     available_count: 10,
//! #     base_price: 50,
//! #     discount: 10,
//! #   };
//! #
//! #   let product_info: ProductInfo = ProductInfo::from(product_row.clone());
//! #   
//! #   println!("{:?}", product_row);
//! #   println!("{:?}", product_info);
//! # }
//! ```
//!
//! Please refer to [https://github.com/shrynx/struct_morph/blob/main/README.md](README) for how to set this up.
//!
//! [https://github.com/shrynx/struct_morph]: https://github.com/shrynx/struct_morph

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, ItemStruct, LitStr, Token,
};

enum MorphFieldArgs {
    TransformFunction(LitStr),
    SelectField(Ident),
}

impl Parse for MorphFieldArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let morph_keyword: Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;

        match morph_keyword {
            t if t == "transform" => Ok(MorphFieldArgs::TransformFunction(input.parse()?)),
            t if t == "select" => Ok(MorphFieldArgs::SelectField(input.parse()?)),
            _ => Err(syn::Error::new_spanned(
                morph_keyword,
                "Expected either 'transform' or 'select'",
            )),
        }
    }
}

#[proc_macro_attribute]
pub fn morph_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = syn::parse_macro_input!(args as MorphFieldArgs);
    input
}

#[proc_macro_attribute]
pub fn morph(attr: TokenStream, item: TokenStream) -> TokenStream {
    let source_struct_ident = parse_macro_input!(attr as Ident);
    let mut input = parse_macro_input!(item as ItemStruct);

    let target_fields = &input
        .fields
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let morph_field_args = f.attrs.iter().find_map(|attr| {
                attr.path().is_ident("morph_field").then(|| {
                    let args: MorphFieldArgs = attr.parse_args().unwrap();
                    args
                })
            });
            match morph_field_args {
                Some(MorphFieldArgs::TransformFunction(func)) => {
                    let func_ident = Ident::new(&func.value(), proc_macro2::Span::call_site());
                    quote! { #field_name: #func_ident(&source) }
                }
                Some(MorphFieldArgs::SelectField(source_field)) => {
                    quote! { #field_name: source.#source_field.clone() }
                }
                None => quote! { #field_name: source.#field_name.clone() },
            }
        })
        .collect::<Vec<_>>();

    let target_struct_ident = &input.ident;

    let from_trait_gen = quote! {
        impl From<#source_struct_ident> for #target_struct_ident {
            fn from(source: #source_struct_ident) -> Self {
                Self {
                    #(#target_fields),*
                }
            }
        }
    };

    input.fields.iter_mut().for_each(|field| {
        field
            .attrs
            .retain(|attr| !attr.path().is_ident("morph_field"));
    });

    TokenStream::from(quote! {
        #input
        #from_trait_gen
    })
}

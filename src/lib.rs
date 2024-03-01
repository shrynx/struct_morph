use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemStruct, NestedMeta};

#[proc_macro_attribute]
pub fn morph(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);

    if args.len() != 1 {
        return syn::Error::new_spanned(&input, "Expected exactly one argument to morph")
            .to_compile_error()
            .into();
    }
    let source_struct_ident = match &args[0] {
        NestedMeta::Meta(syn::Meta::Path(path)) if path.segments.len() == 1 => {
            &path.segments[0].ident
        }
        _ => {
            return syn::Error::new_spanned(
                &args[0],
                "Expected a single identifier as argument to morph",
            )
            .to_compile_error()
            .into()
        }
    };

    let target_struct_ident = &input.ident;
    let fields = input.fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            #field_name: source.#field_name.clone()
        }
    });

    let gen = quote! {
        impl From<#source_struct_ident> for #target_struct_ident {
            fn from(source: #source_struct_ident) -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    };

    let expanded = quote! {
        #input
        #gen
    };

    TokenStream::from(expanded)
}

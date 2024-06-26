use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructFields)]
pub fn derive_struct_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        syn::Data::Struct(data) => data.fields,
        _ => panic!("StructFields can only be derived for structs"),
    };

    let field_infos: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let field_type = &f.ty;
            quote! {
                derive_struct_fields::StructFieldInfo {
                    name: stringify!(#field_name),
                    field_type: stringify!(#field_type),
                }
            }
        })
        .collect();

    let field_names: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            quote! {
                stringify!(#field_name).to_string()
            }
        })
        .collect();

    let expanded = quote! {
        impl derive_struct_fields::StructFields for #name {
            fn struct_fields() -> Vec<derive_struct_fields::StructFieldInfo> {
                vec![ #(#field_infos),* ]
            }
            fn struct_field_names() -> Vec<String> {
                vec![ #(#field_names),* ]
            }

        }
    };

    TokenStream::from(expanded)
}

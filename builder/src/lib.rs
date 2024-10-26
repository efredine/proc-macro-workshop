use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    eprintln!("input: {:?}", input);
    let _ = input;
    let input = parse_macro_input!(input as DeriveInput);
    let attr_length = input.attrs.len();
    eprintln!("attr_length: {attr_length}");
    let vis = input.vis;
    match vis {
        syn::Visibility::Public(s) => eprintln!("vis: public, {:?}", s.span.source_text()),
        syn::Visibility::Restricted(_) => eprintln!("vis: restricted"),
        syn::Visibility::Inherited => eprintln!("vis: inherited"),
    }
    let ident = input.ident;
    eprintln!("ident: {ident}");
    let generics = input.generics;
    eprintln!(
        "generics: lt_token?: {}, gt_token?: {}, where_clause?: {}, params_length: {}",
        generics.lt_token.is_some(),
        generics.gt_token.is_some(),
        generics.where_clause.is_some(),
        generics.params.len()
    );
    let data = input.data;
    match data {
        syn::Data::Struct(s) => {
            eprintln!("data: struct, fields_length: {}", s.fields.len());
            s.fields.iter().for_each(|f| {
                eprintln!("field: {:?}", f.ident);
            });
        }
        syn::Data::Enum(e) => {
            eprintln!("data: enum, variants_length: {}", e.variants.len());
        }
        syn::Data::Union(u) => {
            eprintln!("data: union, fields_length: {}", u.fields.named.len());
        }
    }
    // let expanded = quote! {
    //     impl #name {
    //         pub fn builder() -> #nameBuilder {
    //             #nameBuilder::default()
    //         }
    //     }
    // };
    TokenStream::new()
}

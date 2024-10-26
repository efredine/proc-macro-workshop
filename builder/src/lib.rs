use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let builder_ident = syn::Ident::new(&format!("{}Builder", ident), ident.span());

    let fields = if let syn::Data::Struct(data) = input.data {
        data.fields
    } else {
        unimplemented!();
    };

    let builder_fields = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #name: Option<#ty>
        }
    });

    let builder_fields_init = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        quote! {
            #name: None
        }
    });
    
    let builder_methods = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });
    
    let expanded = quote! {
        pub struct #builder_ident {
            #(#builder_fields),*
        }
        
        impl #builder_ident {
            #(#builder_methods)*
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_fields_init),*
                }
            }
        }
    };

    expanded.into()
}
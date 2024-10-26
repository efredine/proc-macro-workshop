use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AngleBracketedGenericArguments, DeriveInput, GenericArgument, PathArguments, Type, TypePath};

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
        let option_inner_ty = get_option_inner_type(ty);
        match option_inner_ty {
            Some(inner_ty) => {
                quote! {
                    pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                        self.#name = Some(Some(#name));
                        self
                    }
                }
            }
            None => {
                quote! {
                    pub fn #name(&mut self, #name: #ty) -> &mut Self {
                        self.#name = Some(#name);
                        self
                    }
                }
            }
        }
    });

    let assign_fields = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        let option_inner_ty = get_option_inner_type(ty);
        match option_inner_ty {
            Some(_) => {
                quote! {
                    #name: self.#name.clone().unwrap_or(None)
                }
            }
            None => {
                quote! {
                    #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
                }
            }
        }
    });

    let expanded = quote! {
        pub struct #builder_ident {
            #(#builder_fields),*
        }

        impl #builder_ident {
            #(#builder_methods)*

            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                Ok(#ident {
                    #(#assign_fields),*
                })
            }
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

fn get_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.first() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_type)) = args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

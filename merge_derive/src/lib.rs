use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote};
use syn::{Data, DeriveInput, Fields, GenericParam, Generics, Index};

#[proc_macro_derive(Mix)]
pub fn expand_derive_mix(input: TokenStream) -> TokenStream { 
    
    let input = TokenStream::from(input);
    let input = parse_macro_input!(input as syn::DeriveInput);
    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to sum up the heap size of each field.
    let fields = merge_fields(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics merge::Mix for #name #ty_generics #where_clause {
            fn mix(&self, other: &Self) -> Self {
                #name {
                    #fields
                }
            }
        }
    };

    TokenStream::from(expanded)

}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(merge::Mix));
        }
    }
    generics
}

fn merge_fields(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        syn::Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            #name: merge::Mix::mix(&self.#name, other.#name)
                        }
                    });
                    quote! {
                        #(#recurse,)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            merge::Mix::mix(&self.#index, other.#index)
                        }
                    });
                    quote! {
                        #(#recurse,)*
                    }
                }
                Fields::Unit => {
                    quote!()
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

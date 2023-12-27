use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use proc_macro2::{Ident, Span};

#[proc_macro_attribute]
pub fn clap_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(item as ItemFn);

    let item_type = f
        .sig
        .inputs
        .first()
        .expect("Need exactly one argument to the function");

    let renamed_main = Ident::new("clap_rewritten_main", Span::call_site());


    f.sig.ident = renamed_main.clone();

    let ty = match item_type {
        syn::FnArg::Receiver(recv) => match *recv.ty.clone() {
            syn::Type::Path(ty) => ty.path.clone(),
            _ => panic!("unexpected type"),
        },

        syn::FnArg::Typed(typed) => match *typed.ty.clone() {
            syn::Type::Path(ty) => ty.path.clone(),
            _ => panic!("Unexpected type"),
        },
    };

    quote! {
        #f

        pub fn main() {
            use clap::Parser;
            use std::io::Write;
            let args = match #ty::try_parse() {
                Ok(args) => args,
                Err(e) => {
                    writeln!(&mut std::io::stderr(), "{e}").expect("Could not write to stderr!");
                    std::process::exit(-1);
                },
            };

            match #renamed_main(args) {
                Ok(()) => {},
                Err(e) => {
                    writeln!(&mut std::io::stderr(), "{e}").expect("Could not write to stderr!");
                    std::process::exit(-1);
                },
            }
        }
    }
    .into()
}

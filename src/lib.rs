extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(SelfUpdate)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl #name {
            pub fn update_contract(&self) -> Promise {
                assert!(env::predecessor_account_id() == self.manager, "Only the manager can update the code");

                let code = env::input().expect("Error: No input").to_vec();

                Promise::new(env::current_account_id())
                    .deploy_contract(code)
                    .function_call(
                        "migrate".to_string(),
                        vec![],
                        0,
                        Gas(200_000_000_000_000),
                    )
                    .as_return()
            }
        }
    };

    TokenStream::from(expanded)
}

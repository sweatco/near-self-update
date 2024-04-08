extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SelfUpdate)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        #[near_bindgen]
        impl #name {
            #[payable]
            pub fn update_contract(&mut self, code: Vec<u8>, callback: Option<String>) -> near_sdk::Promise {
                self.assert_account_can_update();
                near_sdk::assert_one_yocto();

                let deploy = near_sdk::Promise::new(near_sdk::env::current_account_id()).deploy_contract(code);

                let Some(callback) = callback else {
                    return deploy.as_return();
                };

                deploy
                    .function_call(callback, vec![], 0, near_sdk::Gas(100_000_000_000_000))
                .   as_return()
            }

            pub fn contract_version(&self) -> String {
                const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
                const VERSION: &str = env!("CARGO_PKG_VERSION");
                format!("{PACKAGE_NAME}-{VERSION}")
            }
        }
    };

    TokenStream::from(expanded)
}

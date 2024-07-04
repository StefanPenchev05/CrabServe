extern crate proc_macro;

use proc_macro::TokenStream;

mod route;

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    
}
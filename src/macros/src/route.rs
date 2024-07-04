use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the arguments passed to the macro (if any)
    // For demonstration, we're not using args, but you could use it to get, for example, a path
    let _args = parse_macro_input!(args as syn::DeriveInput);

    // Parse the function the macro is attached to
    let function = parse_macro_input!(input as ItemFn);

    // Implement your logic here. For demonstration, we're simply returning the original function
    // without modifications. In a real scenario, you might modify the function or generate additional
    // code based on the args.

    // Generate the output TokenStream
    quote!(#function).into()
}
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn custom_mangle(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);

    let name = &function.sig.ident;
    let inputs = &function.sig.inputs;
    let output = &function.sig.output;
    let block = &function.block;

    let name_token = format!("__{name}");

    let expanded = quote! {
        #[export_name = #name_token]
        fn #name(#inputs) #output {
            #block
        }
    };

    TokenStream::from(expanded)
}

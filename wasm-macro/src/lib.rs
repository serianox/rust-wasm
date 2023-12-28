use proc_macro::TokenStream;
use quote::quote;
use sha3::{Digest, Sha3_256};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn custom_mangle(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);

    let vis = &function.vis;
    let name = &function.sig.ident;
    let inputs = &function.sig.inputs;
    let output = &function.sig.output;
    let block = &function.block;

    let binding = Sha3_256::digest(name.to_string());
    let name_hash: &[u8] = &binding.as_slice()[..4];
    let name_hash_to_hex_string: String = hex::encode(name_hash);
    let name_token = format!("__${name}${name_hash_to_hex_string}");

    let expanded = quote! {
        #[export_name = #name_token]
        #vis fn #name(#inputs) #output {
            #block
        }
    };

    TokenStream::from(expanded)
}

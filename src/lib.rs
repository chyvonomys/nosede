extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn nose(_: TokenStream) -> TokenStream {
    TokenStream::from_str("").unwrap()
}

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn node(_: TokenStream) -> TokenStream {
    TokenStream::from_str("").unwrap()
}

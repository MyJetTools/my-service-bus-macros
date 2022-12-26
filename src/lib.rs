mod fn_impl_protobuf_model;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_sb_entity_protobuf_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::fn_impl_protobuf_model::generate(attr, item)
}

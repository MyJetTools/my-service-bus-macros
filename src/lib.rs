mod fn_impl_protobuf_model;

use proc_macro::TokenStream;

#[proc_macro_derive(MySbEntityProtobufModel, attributes(db_field_name, debug_sql))]
pub fn postgres_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_protobuf_model::generate(&ast)
}

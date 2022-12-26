use proc_macro::TokenStream;
use quote::quote;

pub fn generate(attr: TokenStream, input: TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let attrs_as_string = attr.to_string();

    let attrs = macros_utils::AttributeParams::new(attrs_as_string);

    let topic_id = attrs.get_from_single_or_named("topic_id");

    if topic_id.is_none() {
        panic!("topic_id parameter is required");
    }

    let topic_id = topic_id.unwrap();

    let topic_id = topic_id.get_value_as_str();

    let struct_name = &ast.ident;

    quote!{
        impl #struct_name{
            pub fn as_protobuf_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
                let mut result = Vec::new();
                prost::Message::encode(self, &mut result)?;
                Ok(result)
            }
        
            pub fn from_protobuf_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
                prost::Message::decode(bytes)
            }

        }

        impl my_service_bus_abstractions::publisher::MySbMessageSerializer for #struct_name{

            fn serialize(
                &self,
                headers: Option<std::collections::HashMap<String, String>>,
            ) -> Result<(Vec<u8>, Option<std::collections::HashMap<String, String>>), String> {
                match self.as_protobuf_bytes() {
                    Ok(result) => Ok((result, headers)),
                    Err(err) => Err(format!("Error serializing protobuf: {}", err)),
                }
            }

        }

        impl my_service_bus_abstractions::subscriber::MySbMessageDeserializer for #struct_name{
            type Item = Self;

            fn deserialize(bytes: &[u8], _: &Option<std::collections::HashMap<String, String>>) -> Result<Self, my_service_bus_abstractions::SubscriberError> {
                match prost::Message::decode(bytes) {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(
                        my_service_bus_abstractions::SubscriberError::CanNotDeserializeMessage(format!(
                            "Error deserializing protobuf: {}",
                            err
                        )),
                    ),
                }
            }
        }

        impl my_service_bus_abstractions::GetMySbModelTopicId for #struct_name{
            fn get_topic_id() -> &'static str {
                #topic_id
            }
        }

    }.into()
}

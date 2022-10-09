use proc_macro::TokenStream;

pub fn generate(attr: TokenStream, input: TokenStream) -> proc_macro::TokenStream {
    let mut result = input.to_string();

    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let attrs_as_string = attr.to_string();
    let mut attrs = crate::utils::parse_attributes(attrs_as_string.as_str());

    let topic_id = attrs.remove("topic_id");

    if topic_id.is_none() {
        panic!("topic_id parameter is required");
    }

    let topic_id = topic_id.unwrap();

    if topic_id.is_none() {
        panic!("topic_id parameter must have a value");
    }

    let topic_id = topic_id.unwrap();

    let name = &ast.ident;

    let struct_name = name.to_string();

    result.push_str(format!("impl {} ", struct_name).as_str());

    let template = r#"
    {
        pub fn as_protobuf_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
            let mut result = Vec::new();
            prost::Message::encode(self, &mut result)?;
            Ok(result)
        }
    
        pub fn from_protobuf_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
            prost::Message::decode(bytes)
        }
    }
    "#;

    result.push_str(template);

    result.push_str(
        format!(
            "impl my_service_bus_abstractions::publisher::MySbMessageSerializer for {} ",
            struct_name
        )
        .as_str(),
    );

    let template = r#"
    {
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
    "#;

    result.push_str(template);

    result.push_str(
        format!(
            "impl my_service_bus_abstractions::subscriber::MySbMessageDeserializer for {} ",
            struct_name
        )
        .as_str(),
    );

    let template = r#"
    {
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
    "#;

    result.push_str(template);

    result.push_str("impl my_service_bus_abstractions::GetMySbModelTopicId for ");
    result.push_str(struct_name.as_str());
    result.push('{');
    result.push_str("fn get_topic_id() -> &'static str {");
    result.push('"');
    result.push_str(topic_id);
    result.push('"');
    result.push_str("}}");

    result.parse().unwrap()
}

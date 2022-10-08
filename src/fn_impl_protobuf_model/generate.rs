pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str(format!("impl {} ", struct_name).as_str());

    let template = r#"
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
            headers: Option<HashMap<String, String>>,
        ) -> Result<(Vec<u8>, Option<HashMap<String, String>>), String> {
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
        fn deserialize(bytes: &[u8], _: &Option<HashMap<String, String>>) -> Result<Self, String> {
            match prost::Message::decode(bytes) {
                Ok(ok) => Ok(ok),
                Err(err) => Err(format!("Error deserializing protobuf: {}", err)),
            }
        }
    }
    "#;

    result.push_str(template);

    result.parse().unwrap()
}

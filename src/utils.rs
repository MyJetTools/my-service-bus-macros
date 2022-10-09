use std::collections::HashMap;

pub fn parse_attributes<'s>(attr_content: &'s str) -> HashMap<&'s str, Option<&'s str>> {
    let mut result = HashMap::new();

    let separator = detect_separator(attr_content);

    for kv in attr_content.split(separator) {
        let (key, value) = split_kv(kv);
        result.insert(key, value);
    }

    result
}

fn detect_separator(attr: &str) -> char {
    let comma_count = attr.chars().filter(|c| *c == ',').count();
    let semicomman_count = attr.chars().filter(|c| *c == ';').count();

    if comma_count > semicomman_count {
        return ',';
    }

    ';'
}

fn split_kv<'s>(src: &'s str) -> (&'s str, Option<&'s str>) {
    let bytes = src.as_bytes();
    for i in 0..src.len() {
        if bytes[i] == b'=' {
            let mut key = &src[0..i];
            key = key.trim();

            let mut value = &src[i + 1..src.len()];
            value = value.trim();

            if value.as_bytes()[0] == b'"' {
                value = &value[1..value.len() - 1];
            }

            return (key, Some(value));
        }
    }
    (src, None)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parsing_kv() {
        let (key, value) = super::split_kv(r#"key="value""#);

        assert_eq!(key, "key");
        assert_eq!(value.unwrap(), "value");
    }

    #[test]
    fn test_parsing_kv_with_spaces() {
        let (key, value) = super::split_kv(r#" key = "value" "#);

        assert_eq!(key, "key");
        assert_eq!(value.unwrap(), "value");
    }
}

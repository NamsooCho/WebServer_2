use std::collections::HashMap;

pub struct ContentType {
    main_type: &'static str,
    sub_type: &'static str,
    // TODO: make optional_fields get/settable
    optional_fields: Option<HashMap<String, String>>,
}

// predefined content types
impl ContentType {
    pub const TEXT_PLAIN: ContentType = create_content_type("text", "plain");
    pub const TEXT_HTML: ContentType = create_content_type("text", "html");

    pub fn new(main_type: &'static str, sub_type: &'static str) -> ContentType {
        create_content_type(main_type, sub_type)
    }
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        format!("{}/{}", self.main_type, self.sub_type)
    }
}

const fn create_content_type(main_type: &'static str, sub_type: &'static str) -> ContentType {
    ContentType {
        main_type,
        sub_type,
        optional_fields: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create() -> ContentType {
        ContentType::new("foo", "bar")
    }

    #[test]
    fn test_creation() {
        let c_type = ContentType::new("hello", "world");
        assert_eq!(c_type.to_string().as_str(), "hello/world");

        let c_type = create();
        assert_eq!(c_type.to_string().as_str(), "foo/bar");
    }
}

use libxml::tree::Node;

use crate::{Error, Result};

use super::ERROR_MESSAGE;

const TEXT: &str = "text";
const LANGUAGE: &str = "language";

#[derive(Clone, Debug)]
pub struct ErrorMessage {
    texts: Vec<Text>,
}

#[derive(Clone, Debug)]
pub struct Text {
    text: String,
    language: Option<String>,
}

impl Text {
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn language(&self) -> Option<&str> {
        match self.language.as_ref() {
            Some(val) => Some(val.as_str()),
            None => None,
        }
    }
}

impl ErrorMessage {
    pub(super) fn parse(node: Node) -> Result<ErrorMessage> {
        if node.get_name() != ERROR_MESSAGE {
            return Error::invalid_child(node);
        }

        let mut texts: Vec<Text> = Vec::new();

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                TEXT => {
                    let text = child_node.get_content();
                    let language = child_node.get_attribute(LANGUAGE);
                    texts.push(Text { text, language })
                }
                _ => return Error::invalid_child(child_node),
            };
        }

        Ok(ErrorMessage { texts })
    }

    pub fn texts(&self) -> &[Text] {
        &self.texts[..]
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use super::ErrorMessage;

    #[test]
    fn deserialize() {
        let xml = r#"
            <errorMessage>
                <text>Must be &lt;= safety contour value</text>
                <text language="kor">안전 윤곽 값보다 작거나 같아야 합니다</text>
            </errorMessage>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = ErrorMessage::parse(node).unwrap();

        let texts = target.texts();
        assert_eq!(texts[0].text(), "Must be <= safety contour value");
        assert_eq!(texts[0].language(), None);
        assert_eq!(texts[1].text(), "안전 윤곽 값보다 작거나 같아야 합니다");
        assert_eq!(texts[1].language(), Some("kor"));
    }
}

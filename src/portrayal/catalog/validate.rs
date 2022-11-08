use libxml::tree::Node;

use super::{ErrorMessage, ERROR_MESSAGE, VALIDATE};
use crate::{Error, Result};

const REGEX: &str = "regex";
const XPATH: &str = "xpath";

#[derive(Clone, Debug)]
pub struct Validate {
    regex: Option<String>,
    xpath: Option<String>,
    error_message: ErrorMessage,
}

impl Validate {
    pub(super) fn parse(node: Node) -> Result<Validate> {
        if node.get_name() != VALIDATE {
            return Error::invalid_child(node);
        }

        let mut regex: Option<String> = None;
        let mut xpath: Option<String> = None;
        let mut error_message: Option<ErrorMessage> = None;

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                REGEX => {
                    regex = Some(child_node.get_content());
                }
                XPATH => {
                    xpath = Some(child_node.get_content());
                }
                ERROR_MESSAGE => match ErrorMessage::parse(child_node) {
                    Ok(msg) => error_message = Some(msg),
                    Err(e) => return Err(e),
                },
                _ => return Error::invalid_child(child_node),
            };
        }

        if error_message.is_none() {
            return Error::missing_child(node, VALIDATE);
        }

        Ok(Validate {
            regex,
            xpath,
            error_message: error_message.unwrap(),
        })
    }

    pub fn regex(&self) -> Option<&str> {
        match self.regex.as_ref() {
            Some(val) => Some(val.as_str()),
            None => None,
        }
    }

    pub fn error_message(&self) -> &ErrorMessage {
        &self.error_message
    }

    pub fn xpath(&self) -> Option<&str> {
        match self.xpath.as_ref() {
            Some(val) => Some(val.as_str()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use super::Validate;

    #[test]
    fn deserialize() {
        let xml = r#"
            <validate>
                <xpath>//ShallowContour &lt;= //SafetyContour</xpath>
                <errorMessage>
                    <text>Must be &lt;= safety contour value</text>
                    <text language="kor">안전 윤곽 값보다 작거나 같아야 합니다</text>
                </errorMessage>
            </validate>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = Validate::parse(node).unwrap();

        assert_eq!(target.regex(), None);

        let texts = target.error_message.texts();
        assert_eq!(texts[0].text(), "Must be <= safety contour value");
        assert_eq!(texts[0].language(), None);
        assert_eq!(texts[1].text(), "안전 윤곽 값보다 작거나 같아야 합니다");
        assert_eq!(texts[1].language(), Some("kor"));
    }
}

use libxml::tree::Node;

use crate::{Error, Result};

const DESCRIPTION: &str = "description";
const NAME: &str = "name";
const LANGUAGE: &str = "language";

#[derive(Clone, Debug)]
pub struct Description {
    name: String,
    description: String,
    language: String,
}

impl Description {
    pub(super) fn parse(node: Node) -> Result<Description> {
        if node.get_name() != DESCRIPTION {
            return Error::invalid_child(node);
        }

        let mut name: Option<String> = None;
        let mut description: Option<String> = None;
        let mut language: Option<String> = None;

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                NAME => name = Some(child_node.get_content()),
                DESCRIPTION => description = Some(child_node.get_content()),
                LANGUAGE => language = Some(child_node.get_content()),
                _ => return Error::invalid_child(child_node),
            };
        }

        if name.is_none() {
            return Error::missing_child(node, NAME);
        }
        if description.is_none() {
            return Error::missing_child(node, DESCRIPTION);
        }
        if language.is_none() {
            return Error::missing_child(node, LANGUAGE);
        }

        Ok(Description {
            name: name.unwrap(),
            description: description.unwrap(),
            language: language.unwrap(),
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn language(&self) -> &str {
        self.language.as_str()
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use super::Description;

    #[test]
    fn deserialize() {
        let xml = r#"
            <description>
                <name>BOYCON01</name>
                <description>conical buoy, paper-chart</description>
                <language>eng</language>
            </description>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = Description::parse(node).unwrap();

        assert_eq!(target.name(), "BOYCON01");
        assert_eq!(target.description(), "conical buoy, paper-chart");
        assert_eq!(target.language(), "eng");
    }
}

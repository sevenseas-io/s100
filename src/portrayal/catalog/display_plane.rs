use libxml::tree::Node;

use super::{CatalogItem, Description, DESCRIPTION, XML_ID};
use crate::{Result, S100Error};

const DISPLAY_PLANE: &str = "displayPlane";

#[derive(Clone, Debug)]
pub struct DisplayPlane {
    id: String,
    descriptions: Vec<Description>,
}

impl DisplayPlane {
    pub(super) fn parse(node: Node) -> Result<DisplayPlane> {
        if node.get_name() != DISPLAY_PLANE {
            return S100Error::invalid_child(node);
        }

        let id: Option<String> = node.get_attribute(XML_ID);
        let mut descriptions: Vec<Description> = Vec::new();

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                DESCRIPTION => match Description::parse(child_node) {
                    Ok(desc) => descriptions.push(desc),
                    Err(e) => return Err(e),
                },
                _ => return S100Error::invalid_child(child_node),
            };
        }

        if id.is_none() {
            return S100Error::missing_attribute(node, XML_ID);
        }

        Ok(DisplayPlane {
            id: id.unwrap(),
            descriptions,
        })
    }
}

impl CatalogItem for DisplayPlane {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn descriptions(&self) -> &[Description] {
        &self.descriptions
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use crate::portrayal::catalog::DisplayPlane;

    use super::CatalogItem;

    #[test]
    fn deserialize() {
        let xml = r#"
            <displayPlane id="1">
                <description>
                    <name>OVERRADAR</name>
                    <description>Content displayed on top of Radar image</description>
                    <language>eng</language>
                </description>
            </displayPlane>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = DisplayPlane::parse(node).unwrap();
        assert_eq!(target.id(), "1");

        let descriptions = target.descriptions();
        assert_eq!(descriptions.len(), 1);

        let description = &descriptions[0];
        assert_eq!(description.name(), "OVERRADAR");
        assert_eq!(
            description.description(),
            "Content displayed on top of Radar image"
        );
        assert_eq!(description.language(), "eng");
    }
}

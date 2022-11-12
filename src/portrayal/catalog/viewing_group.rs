use libxml::tree::Node;

use super::{CatalogItem, Description, DESCRIPTION, VIEWING_GROUP, XML_ID};
use crate::{Result, S100Error};
#[derive(Clone, Debug)]
pub struct ViewingGroup {
    id: String,
    descriptions: Vec<Description>,
}

impl ViewingGroup {
    pub(super) fn parse(node: Node) -> Result<ViewingGroup> {
        if node.get_name() != VIEWING_GROUP {
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

        Ok(ViewingGroup {
            id: id.unwrap(),
            descriptions,
        })
    }
}

impl CatalogItem for ViewingGroup {
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

    use crate::portrayal::catalog::ViewingGroup;

    use super::CatalogItem;

    #[test]
    fn deserialize() {
        let xml = r#"
            <viewingGroup id="2">
                <description>
                    <name>26220</name>
                    <description/>
                    <language>eng</language>
                </description>
            </viewingGroup>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = ViewingGroup::parse(node).unwrap();
        assert_eq!(target.id(), "2");

        let descriptions = target.descriptions();
        assert_eq!(descriptions.len(), 1);

        let description = &descriptions[0];
        assert_eq!(description.name(), "26220");
        assert_eq!(description.description(), "");
        assert_eq!(description.language(), "eng");
    }
}

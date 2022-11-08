use libxml::tree::Node;

use super::{CatalogItem, Description, DESCRIPTION, VIEWING_GROUP, XML_ID};
use crate::{Error, Result};

const VIEWING_GROUP_LAYER: &str = "viewingGroupLayer";

#[derive(Clone, Debug)]
pub struct ViewingGroupLayer {
    id: String,
    descriptions: Vec<Description>,
    viewing_groups: Vec<String>,
}

impl ViewingGroupLayer {
    pub(super) fn parse(node: Node) -> Result<ViewingGroupLayer> {
        if node.get_name() != VIEWING_GROUP_LAYER {
            return Error::invalid_child(node);
        }

        let id: Option<String> = node.get_attribute(XML_ID);
        let mut descriptions: Vec<Description> = Vec::new();
        let mut viewing_groups: Vec<String> = Vec::new();

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                DESCRIPTION => match Description::parse(child_node) {
                    Ok(desc) => descriptions.push(desc),
                    Err(e) => return Err(e),
                },
                VIEWING_GROUP => {
                    viewing_groups.push(child_node.get_content());
                }
                _ => return Error::invalid_child(child_node),
            };
        }

        if id.is_none() {
            return Error::missing_attribute(node, XML_ID);
        }

        Ok(ViewingGroupLayer {
            id: id.unwrap(),
            descriptions,
            viewing_groups,
        })
    }

    pub fn viewing_groups(&self) -> Vec<&str> {
        self.viewing_groups.iter().map(|s| s.as_str()).collect()
    }
}

impl CatalogItem for ViewingGroupLayer {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn descriptions(&self) -> &[Description] {
        &self.descriptions[..]
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use crate::portrayal::catalog::ViewingGroupLayer;

    use super::CatalogItem;

    #[test]
    fn deserialize() {
        let xml = r#"
            <viewingGroupLayer id="2">
                <description>
                    <name>Drying Line</name>
                    <description>Standard Display elements</description>
                    <language>eng</language>
                </description>
                <viewingGroup>22010</viewingGroup>
            </viewingGroupLayer>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = ViewingGroupLayer::parse(node).unwrap();
        assert_eq!(target.id(), "2");

        let descriptions = target.descriptions();
        assert_eq!(descriptions.len(), 1);

        let description = &descriptions[0];
        assert_eq!(description.name(), "Drying Line");
        assert_eq!(description.description(), "Standard Display elements");
        assert_eq!(description.language(), "eng");

        let viewing_groups = target.viewing_groups();
        assert_eq!(viewing_groups.len(), 1);
        assert_eq!(viewing_groups[0], "22010");
    }
}

use libxml::tree::Node;

use super::{CatalogItem, Description, DESCRIPTION, XML_ID};
use crate::{Error, Result};

const DISPLAY_MODE: &str = "displayMode";
const VIEWING_GROUP_LAYER: &str = "viewingGroupLayer";

#[derive(Clone, Debug)]
pub struct DisplayMode {
    id: String,
    descriptions: Vec<Description>,
    viewing_group_layers: Vec<String>,
}

impl DisplayMode {
    pub(super) fn parse(node: Node) -> Result<DisplayMode> {
        if node.get_name() != DISPLAY_MODE {
            return Error::invalid_child(node);
        }

        let id: Option<String> = node.get_attribute(XML_ID);
        let mut descriptions: Vec<Description> = Vec::new();
        let mut viewing_group_layers: Vec<String> = Vec::new();

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                DESCRIPTION => match Description::parse(child_node) {
                    Ok(desc) => descriptions.push(desc),
                    Err(e) => return Err(e),
                },
                VIEWING_GROUP_LAYER => {
                    viewing_group_layers.push(child_node.get_content());
                }
                _ => return Error::invalid_child(child_node),
            };
        }

        if id.is_none() {
            return Error::missing_attribute(node, XML_ID);
        }

        Ok(DisplayMode {
            id: id.unwrap(),
            descriptions,
            viewing_group_layers,
        })
    }

    pub fn viewing_group_layers(&self) -> Vec<&str> {
        self.viewing_group_layers
            .iter()
            .map(|s| s.as_str())
            .collect()
    }
}

impl CatalogItem for DisplayMode {
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

    use crate::portrayal::catalog::DisplayMode;

    use super::CatalogItem;

    #[test]
    fn deserialize() {
        let xml = r#"
            <displayMode id="1">
                <description>
                    <name>Base</name>
                    <description>Always on display</description>
                    <language>eng</language>
                </description>
                <viewingGroupLayer>1</viewingGroupLayer>
            </displayMode>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = DisplayMode::parse(node).unwrap();
        assert_eq!(target.id(), "1");

        let descriptions = target.descriptions();
        assert_eq!(descriptions.len(), 1);

        let description = &descriptions[0];
        assert_eq!(description.name(), "Base");
        assert_eq!(description.description(), "Always on display");
        assert_eq!(description.language(), "eng");

        let viewing_groups = target.viewing_group_layers();
        assert_eq!(viewing_groups.len(), 1);
        assert_eq!(viewing_groups[0], "1");
    }
}

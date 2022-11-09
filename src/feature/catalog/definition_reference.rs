use libxml::tree::Node;

use crate::{Error, Result};

use super::{DEFINITION_REFERENCE, XML_REF};

const SOURCE_IDENTIFIER: &str = "sourceIdentifier";
const DEFINITION_SOURCE: &str = "definitionSource";

#[derive(Clone, Debug)]
pub struct DefinitionReference {
    source_identifier: String,
    definition_source: String,
}

impl DefinitionReference {
    pub(super) fn parse(node: Node) -> Result<DefinitionReference> {
        let node_name = node.get_name();
        if node_name != DEFINITION_REFERENCE {
            return Err(Error::Parse(format!(
                "'{}' received a node named '{}'",
                DEFINITION_REFERENCE, node_name
            )));
        }

        let mut source_identifier: Option<String> = None;
        let mut definition_source: Option<String> = None;

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                SOURCE_IDENTIFIER => source_identifier = Some(child_node.get_content()),
                DEFINITION_SOURCE => {
                    if let Some(val) = child_node.get_attribute(XML_REF) {
                        definition_source = Some(val);
                    } else {
                        return Error::missing_attribute(child_node, DEFINITION_REFERENCE);
                    }
                }
                _ => return Error::invalid_child(child_node),
            };
        }

        if source_identifier.is_none() {
            return Error::missing_child(node, SOURCE_IDENTIFIER);
        }
        if definition_source.is_none() {
            return Error::missing_child(node, DEFINITION_SOURCE);
        }

        Ok(DefinitionReference {
            source_identifier: source_identifier.unwrap(),
            definition_source: definition_source.unwrap(),
        })
    }

    pub fn source_identifier(&self) -> &str {
        self.source_identifier.as_str()
    }

    pub fn definition_source(&self) -> &str {
        self.definition_source.as_str()
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use super::DefinitionReference;

    #[test]
    fn deserialize() {
        let xml = r#"
            <S100FC:definitionReference xmlns:S100FC="http://www.iho.int/S100FC">
                <S100FC:sourceIdentifier>1</S100FC:sourceIdentifier>
                <S100FC:definitionSource ref="IHOREG"/>
            </S100FC:definitionReference>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = DefinitionReference::parse(node).unwrap();

        assert_eq!(target.source_identifier(), "1");
        assert_eq!(target.definition_source(), "IHOREG");
    }
}

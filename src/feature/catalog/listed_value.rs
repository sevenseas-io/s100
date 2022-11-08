use libxml::tree::Node;

use super::{DefinitionReference, DEFINITION_REFERENCE, LISTED_VALUE};
use crate::{Error, Result};

const LABEL: &str = "label";
const DEFINITION: &str = "definition";
const CODE: &str = "code";

pub struct ListedValue {
    label: String,
    definition: String,
    code: u64,
    definition_reference: Option<DefinitionReference>,
}

impl ListedValue {
    pub(super) fn parse(node: Node) -> Result<ListedValue> {
        if node.get_name() != LISTED_VALUE {
            return Error::invalid_child(node);
        }

        let mut label: Option<String> = None;
        let mut definition: Option<String> = None;
        let mut code: Option<u64> = None;
        let mut definition_reference: Option<DefinitionReference> = None;

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                LABEL => label = Some(child_node.get_content()),
                DEFINITION => definition = Some(child_node.get_content()),
                CODE => match child_node.get_content().parse() {
                    Ok(val) => code = Some(val),
                    Err(_) => return Error::invalid_value(child_node),
                },
                DEFINITION_REFERENCE => match DefinitionReference::parse(child_node) {
                    Ok(val) => definition_reference = Some(val),
                    Err(e) => return Err(e),
                },
                _ => return Error::invalid_child(child_node),
            };
        }

        if label.is_none() {
            return Error::missing_child(node, LABEL);
        }
        if definition.is_none() {
            return Error::missing_child(node, DEFINITION);
        }
        if code.is_none() {
            return Error::missing_child(node, CODE);
        }

        Ok(ListedValue {
            label: label.unwrap(),
            definition: definition.unwrap(),
            code: code.unwrap(),
            definition_reference,
        })
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn definition(&self) -> &str {
        self.definition.as_str()
    }

    pub fn code(&self) -> u64 {
        self.code
    }

    pub fn definition_reference(&self) -> Option<&DefinitionReference> {
        match self.definition_reference.as_ref() {
            Some(val) => Some(&val),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use super::ListedValue;

    #[test]
    fn deserialize() {
        let xml = r#"
            <S100FC:listedValue xmlns:S100FC="http://www.iho.int/S100FC">
                <S100FC:label>north cardinal mark</S100FC:label>
                <S100FC:definition>Quadrant bounded by the true bearing NW-NE taken from the point of interest it should be passed to the north side of the mark.</S100FC:definition>
                <S100FC:code>1</S100FC:code>
            </S100FC:listedValue>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = ListedValue::parse(node).unwrap();

        assert_eq!(target.label(), "north cardinal mark");
        assert_eq!(target.definition(), "Quadrant bounded by the true bearing NW-NE taken from the point of interest it should be passed to the north side of the mark.");
        assert_eq!(target.code(), 1);
    }
}

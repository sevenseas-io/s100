use libxml::tree::Node;
use std::str::FromStr;

use super::{
    AttributeValueType, DefinitionReference, Item, ListedValue, DEFINITION_REFERENCE,
    SIMPLE_ATTRIBUTE,
};
use crate::{
    feature::{concept::QuantitySpecification, QUANTITY_SPECIFICATION},
    Error, Result,
};

const NAME: &str = "name";
const DEFINITION: &str = "definition";
const CODE: &str = "code";
const REMARKS: &str = "remarks";
const ALIAS: &str = "alias";
const VALUE_TYPE: &str = "valueType";
const LISTED_VALUES: &str = "listedValues";

pub struct SimpleAttribute {
    name: String,
    definition: String,
    code: String,
    remarks: Option<String>,
    alias: Option<String>,
    value_type: AttributeValueType,
    listed_values: Vec<ListedValue>,
    definition_reference: Option<DefinitionReference>,
    quality_specification: Option<QuantitySpecification>,
}

impl SimpleAttribute {
    pub(super) fn parse(node: Node) -> Result<SimpleAttribute> {
        let node_name = node.get_name();
        if node_name != SIMPLE_ATTRIBUTE {
            return Err(Error::Parse(format!(
                "'{}' received a node named '{}'",
                SIMPLE_ATTRIBUTE, node_name
            )));
        }

        let mut name: Option<String> = None;
        let mut definition: Option<String> = None;
        let mut code: Option<String> = None;
        let mut remarks: Option<String> = None;
        let mut alias: Option<String> = None;
        let mut value_type: Option<AttributeValueType> = None;
        let mut listed_values: Vec<ListedValue> = Vec::new();
        let mut definition_reference: Option<DefinitionReference> = None;
        let mut quality_specification: Option<QuantitySpecification> = None;

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                NAME => name = Some(child_node.get_content()),
                DEFINITION => definition = Some(child_node.get_content()),
                CODE => code = Some(child_node.get_content()),
                REMARKS => remarks = Some(child_node.get_content()),
                ALIAS => alias = Some(child_node.get_content()),
                VALUE_TYPE => {
                    match AttributeValueType::from_str(child_node.get_content().as_str()) {
                        Ok(val) => value_type = Some(val),
                        Err(_) => return Error::invalid_value(child_node),
                    }
                }
                LISTED_VALUES => {
                    for listed_value_node in child_node.get_child_elements() {
                        match ListedValue::parse(listed_value_node) {
                            Ok(listed_value) => listed_values.push(listed_value),
                            Err(_) => return Error::invalid_value(child_node),
                        }
                    }
                }
                DEFINITION_REFERENCE => match DefinitionReference::parse(child_node) {
                    Ok(val) => definition_reference = Some(val),
                    Err(e) => return Err(e),
                },
                QUANTITY_SPECIFICATION => {
                    match QuantitySpecification::from_str(child_node.get_content().as_str()) {
                        Ok(val) => quality_specification = Some(val),
                        Err(_) => return Error::invalid_value(child_node),
                    }
                }
                _ => {
                    //TODO: return error if we find unrecognized element
                    //return Error::invalid_child(child_node);
                }
            };
        }

        if name.is_none() {
            return Error::missing_child(node, NAME);
        }
        if definition.is_none() {
            return Error::missing_child(node, DEFINITION);
        }
        if code.is_none() {
            return Error::missing_child(node, CODE);
        }
        if value_type.is_none() {
            return Error::missing_child(node, VALUE_TYPE);
        }

        Ok(SimpleAttribute {
            name: name.unwrap(),
            definition: definition.unwrap(),
            code: code.unwrap(),
            remarks,
            alias,
            value_type: value_type.unwrap(),
            listed_values,
            definition_reference,
            quality_specification,
        })
    }

    pub fn remarks(&self) -> Option<&str> {
        match self.remarks.as_ref() {
            Some(val) => Some(val.as_str()),
            None => None,
        }
    }

    pub fn alias(&self) -> Option<&str> {
        match self.alias.as_ref() {
            Some(val) => Some(val.as_str()),
            None => None,
        }
    }

    pub fn value_type(&self) -> AttributeValueType {
        self.value_type
    }

    pub fn listed_values(&self) -> &[ListedValue] {
        &self.listed_values[..]
    }

    pub fn definition_reference(&self) -> Option<&DefinitionReference> {
        match self.definition_reference.as_ref() {
            Some(val) => Some(&val),
            None => None,
        }
    }

    pub fn quality_specification(&self) -> Option<QuantitySpecification> {
        self.quality_specification
    }
}

impl Item for SimpleAttribute {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn definition(&self) -> &str {
        self.definition.as_str()
    }

    fn code(&self) -> &str {
        self.code.as_str()
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use super::{AttributeValueType, Item, SimpleAttribute};

    #[test]
    fn deserialize() {
        let xml = r#"
            <S100FC:S100_FC_SimpleAttribute xmlns:S100FC="http://www.iho.int/S100FC">
                <S100FC:name>Category of checkpoint</S100FC:name>
                <S100FC:definition>Classification of a place where vehicles or travellers are stopped for identification or inspection</S100FC:definition>
                <S100FC:code>categoryOfCheckpoint</S100FC:code>
                <S100FC:alias>CATCHP</S100FC:alias>
                <S100FC:valueType>enumeration</S100FC:valueType>
                <S100FC:listedValues>
                    <S100FC:listedValue>
                        <S100FC:label>custom</S100FC:label>
                        <S100FC:definition>an office, especially in ports, at which customs dues are collected or administrated.</S100FC:definition>
                        <S100FC:code>1</S100FC:code>
                    </S100FC:listedValue>
                    <S100FC:listedValue>
                        <S100FC:label>border</S100FC:label>
                        <S100FC:definition>an office, at which immigration control takes place</S100FC:definition>
                        <S100FC:code>2</S100FC:code>
                    </S100FC:listedValue>
                </S100FC:listedValues>
            </S100FC:S100_FC_SimpleAttribute>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = SimpleAttribute::parse(node).unwrap();

        assert_eq!(target.name(), "Category of checkpoint");
        assert_eq!(target.definition(), "Classification of a place where vehicles or travellers are stopped for identification or inspection");
        assert_eq!(target.code(), "categoryOfCheckpoint");
        assert_eq!(target.remarks(), None);
        assert_eq!(target.alias(), Some("CATCHP"));
        assert_eq!(target.value_type(), AttributeValueType::Enumeration);
    }
}

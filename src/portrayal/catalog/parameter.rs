use std::str::FromStr;

use libxml::tree::Node;

use super::{
    CatalogItem, Description, ParameterType, Validate, DESCRIPTION, PARAMETER, VALIDATE, XML_ID,
};
use crate::{Result, S100Error};

const TYPE: &str = "type";
const DEFAULT: &str = "default";

#[derive(Clone, Debug)]
pub struct Parameter {
    id: String,
    descriptions: Vec<Description>,
    parameter_type: ParameterType,
    default: String,
    validate: Option<Validate>,
}

impl Parameter {
    pub(super) fn parse(node: Node) -> Result<Parameter> {
        if node.get_name() != PARAMETER {
            return S100Error::invalid_child(node);
        }

        let id: Option<String> = node.get_attribute(XML_ID);
        let mut descriptions: Vec<Description> = Vec::new();
        let mut parameter_type: Option<ParameterType> = None;
        let mut default: Option<String> = None;
        let mut validate: Option<Validate> = None;

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                DESCRIPTION => match Description::parse(child_node) {
                    Ok(desc) => descriptions.push(desc),
                    Err(e) => return Err(e),
                },
                DEFAULT => {
                    default = Some(child_node.get_content());
                }
                TYPE => match ParameterType::from_str(child_node.get_content().as_str()) {
                    Ok(val) => parameter_type = Some(val),
                    Err(_) => return S100Error::invalid_value(child_node),
                },
                VALIDATE => match Validate::parse(child_node) {
                    Ok(val) => validate = Some(val),
                    Err(e) => return Err(e),
                },
                _ => return S100Error::invalid_child(child_node),
            };
        }

        if id.is_none() {
            return S100Error::missing_attribute(node, XML_ID);
        }
        if parameter_type.is_none() {
            return S100Error::missing_child(node, TYPE);
        }
        if default.is_none() {
            return S100Error::missing_child(node, DEFAULT);
        }

        Ok(Parameter {
            id: id.unwrap(),
            descriptions,
            parameter_type: parameter_type.unwrap(),
            default: default.unwrap(),
            validate,
        })
    }

    pub fn parameter_type(&self) -> ParameterType {
        self.parameter_type
    }

    pub fn default(&self) -> &str {
        self.default.as_ref()
    }

    pub fn validate(&self) -> Option<&Validate> {
        match self.validate.as_ref() {
            Some(val) => Some(val),
            None => None,
        }
    }
}

impl CatalogItem for Parameter {
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

    use crate::portrayal::catalog::{Parameter, ParameterType};

    use super::CatalogItem;

    #[test]
    fn deserialize() {
        let xml = r#"
            <parameter id="SafetyDepth">
                <description>
                    <name>SAFETY_DEPTH</name>
                    <description>selected safety depth (meters)</description>
                    <language>eng</language>
                </description>
                <type>Double</type>
                <default>30</default>
            </parameter>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = Parameter::parse(node).unwrap();
        assert_eq!(target.id(), "SafetyDepth");

        let descriptions = target.descriptions();
        assert_eq!(descriptions.len(), 1);

        let description = &descriptions[0];
        assert_eq!(description.name(), "SAFETY_DEPTH");
        assert_eq!(description.description(), "selected safety depth (meters)");
        assert_eq!(description.language(), "eng");

        assert_eq!(target.parameter_type(), ParameterType::Double);
        assert_eq!(target.default(), "30");
    }
}

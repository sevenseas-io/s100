use libxml::tree::Node;
use std::str::FromStr;

use super::{CatalogItem, Description, ExternalFile, FileFormat, FileType, XML_ID};
use crate::{Result, S100Error};

const SYMBOL: &str = "symbol";
const DESCRIPTION: &str = "description";
const FILE_NAME: &str = "fileName";
const FILE_TYPE: &str = "fileType";
const FILE_FORMAT: &str = "fileFormat";

#[derive(Clone, Debug)]
pub struct Symbol {
    id: String,
    descriptions: Vec<Description>,
    file_name: String,
    file_type: FileType,
    file_format: FileFormat,
}

impl Symbol {
    pub(super) fn parse(node: Node) -> Result<Symbol> {
        if node.get_name() != SYMBOL {
            return S100Error::invalid_child(node);
        }

        let id: Option<String> = node.get_attribute(XML_ID);
        let mut descriptions: Vec<Description> = Vec::new();
        let mut file_name: Option<String> = None;
        let mut file_type: Option<FileType> = None;
        let mut file_format: Option<FileFormat> = None;

        for child_node in node.get_child_elements() {
            match child_node.get_name().as_str() {
                DESCRIPTION => match Description::parse(child_node) {
                    Ok(desc) => descriptions.push(desc),
                    Err(e) => return Err(e),
                },
                FILE_NAME => {
                    file_name = Some(child_node.get_content());
                }
                FILE_TYPE => match FileType::from_str(child_node.get_content().as_str()) {
                    Ok(val) => file_type = Some(val),
                    Err(_) => return S100Error::invalid_value(child_node),
                },
                FILE_FORMAT => match FileFormat::from_str(child_node.get_content().as_str()) {
                    Ok(val) => file_format = Some(val),
                    Err(_) => return S100Error::invalid_value(child_node),
                },
                _ => return S100Error::invalid_child(child_node),
            };
        }

        if id.is_none() {
            return S100Error::missing_attribute(node, XML_ID);
        }
        if file_name.is_none() {
            return S100Error::missing_child(node, FILE_NAME);
        }
        if file_type.is_none() {
            return S100Error::missing_child(node, FILE_TYPE);
        }
        if file_format.is_none() {
            return S100Error::missing_child(node, FILE_FORMAT);
        }

        Ok(Symbol {
            id: id.unwrap(),
            descriptions,
            file_name: file_name.unwrap(),
            file_type: file_type.unwrap(),
            file_format: file_format.unwrap(),
        })
    }
}

impl CatalogItem for Symbol {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn descriptions(&self) -> &[Description] {
        &self.descriptions
    }
}

impl ExternalFile for Symbol {
    fn file_name(&self) -> &str {
        self.file_name.as_str()
    }

    fn file_type(&self) -> FileType {
        self.file_type
    }

    fn file_format(&self) -> FileFormat {
        self.file_format
    }
}

#[cfg(test)]
mod tests {
    use libxml::parser::Parser;

    use super::{CatalogItem, FileFormat, FileType, Symbol};

    #[test]
    fn deserialize() {
        let xml = r#"
            <symbol id="BOYCON01">
                <description>
                    <name>BOYCON01</name>
                    <description>conical buoy, paper-chart</description>
                    <language>eng</language>
                </description>
                <fileName>BOYCON01.svg</fileName>
                <fileType>Symbol</fileType>
                <fileFormat>SVG</fileFormat>
            </symbol>"#;

        let parser = Parser::default();
        let document = parser.parse_string(xml).unwrap();
        let node = document.get_root_element().unwrap();
        let target = Symbol::parse(node).unwrap();
        assert_eq!(target.id, "BOYCON01");

        let descriptions = target.descriptions();
        assert_eq!(descriptions.len(), 1);

        let description = &descriptions[0];
        assert_eq!(description.name(), "BOYCON01");
        assert_eq!(description.description(), "conical buoy, paper-chart");
        assert_eq!(description.language(), "eng");

        assert_eq!(target.file_name, "BOYCON01.svg");
        assert_eq!(target.file_type, FileType::Symbol);
        assert_eq!(target.file_format, FileFormat::SVG);
    }
}

use libxml::parser::Parser;
use std::path::Path;

use super::{SimpleAttribute, FEATURE_CATALOG};
use crate::{Error, Result};

const SIMPLE_ATTRIBUTES: &str = "S100_FC_SimpleAttributes";

pub struct FeatureCatalog {
    simple_attributes: Vec<SimpleAttribute>,
}

impl FeatureCatalog {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<FeatureCatalog> {
        let parser = Parser::default();
        let filename_result = path.as_ref().to_str();
        match filename_result {
            Some(filename) => {
                let document = parser.parse_file(filename)?;

                if let Some(root) = document.get_root_element() {
                    let mut feature_catalog = FeatureCatalog {
                        simple_attributes: Vec::new(),
                    };

                    let root_name = root.get_name();
                    if root_name == FEATURE_CATALOG {
                        for node in root.get_child_elements() {
                            match node.get_name().as_str() {
                                SIMPLE_ATTRIBUTES => {
                                    for simple_attribute_node in node.get_child_elements() {
                                        match SimpleAttribute::parse(simple_attribute_node) {
                                            Ok(simple_attribute) => feature_catalog
                                                .simple_attributes
                                                .push(simple_attribute),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                _ => {
                                    //TODO: return error if we find unrecognized element
                                }
                            }
                        }

                        Ok(feature_catalog)
                    } else {
                        Err(Error::Parse(format!(
                            "Root node is not '{}', found '{} instead",
                            FEATURE_CATALOG, root_name,
                        )))
                    }
                } else {
                    Err(Error::Parse("Root node does not exist".to_string()))
                }
            }
            None => Err(Error::Parse("path is empty".to_string())),
        }
    }

    pub fn simple_attributes(&self) -> &[SimpleAttribute] {
        &self.simple_attributes[..]
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::FeatureCatalog;

    #[test]
    fn deserialize() {
        let xml = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <S100FC:S100_FC_FeatureCatalogue xmlns:S100FC="http://www.iho.int/S100FC" xmlns:S100Base="http://www.iho.int/S100Base" xmlns:S100CI="http://www.iho.int/S100CI" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:S100FD="http://www.iho.int/S100FD" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.iho.int/S100FC S100FC.xsd">
            <S100FC:S100_FC_SimpleAttributes>

            <S100FC:S100_FC_SimpleAttribute>
			<S100FC:name>Call sign</S100FC:name>
			<S100FC:definition>The designated call-sign of a radio station.</S100FC:definition>
			<S100FC:code>callSign</S100FC:code>
			<S100FC:alias>CALSGN</S100FC:alias>
			<S100FC:valueType>text</S100FC:valueType>
		</S100FC:S100_FC_SimpleAttribute>

            </S100FC:S100_FC_SimpleAttributes>
        </S100FC:S100_FC_FeatureCatalogue>"#;

        let mut temp_file = NamedTempFile::new().expect("Unable to create temp file");
        temp_file
            .write_all(xml.as_bytes())
            .expect("Unable to write XML");

        let result = FeatureCatalog::open(temp_file.path());
        temp_file.close().expect("Unable to close temp file");

        match result {
            Err(e) => {
                panic!("Unable to parse feature catalog: {}", e)
            }
            Ok(target) => {
                assert_eq!(target.simple_attributes().len(), 1);
            }
        }
    }
}

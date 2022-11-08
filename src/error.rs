use libxml::tree::Node;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: ::std::io::Error,
    },

    #[error("libxml parsing error: {source}")]
    Libxml {
        #[from]
        source: ::libxml::parser::XmlParseError,
    },

    #[error("Parsing error: {0}")]
    Parse(String),
}

impl Error {
    pub(crate) fn invalid_child<T>(node: Node) -> Result<T> {
        Err(Error::Parse(format!(
            "'{}' contains an invalid child node called '{}'",
            node.get_parent()
                .expect("not expecting root node")
                .get_name(),
            node.get_name()
        )))
    }

    pub(crate) fn invalid_enum<T>(node_name: &str, child_node_name: &str) -> Result<T> {
        Err(Error::Parse(format!(
            "'{}' received a node named '{}'",
            child_node_name, node_name
        )))
    }

    pub(crate) fn invalid_value<T>(node: Node) -> Result<T> {
        Err(Error::Parse(format!(
            "'{}' received an invalid value: '{}'",
            node.get_name(),
            node.get_content()
        )))
    }

    pub(crate) fn missing_attribute<T>(node: Node, child_name: &str) -> Result<T> {
        Err(Error::Parse(format!(
            "'{}' is missing an attribute called '{}'",
            node.get_name(),
            child_name
        )))
    }

    pub(crate) fn missing_child<T>(node: Node, child_name: &str) -> Result<T> {
        Err(Error::Parse(format!(
            "'{}' is missing a child node called '{}'",
            node.get_name(),
            child_name
        )))
    }
}

use libxml::{parser::XmlParseError, tree::Node};
use std::{error::Error, fmt, io::Error as IOError};

pub type Result<T> = std::result::Result<T, S100Error>;

#[derive(Debug)]
pub enum S100Error {
    IO(IOError),
    XML(XmlParseError),
    Parse(String),
}

impl fmt::Display for S100Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            S100Error::IO(e) => write!(f, "an IO error occured: {}", e),
            S100Error::XML(e) => write!(f, "an XML error occured: {}", e),
            S100Error::Parse(s) => write!(f, "an error occured while parsing an S-100 file: {}", s),
        }
    }
}

impl Error for S100Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            S100Error::IO(ref e) => Some(e),
            S100Error::XML(ref e) => Some(e),
            S100Error::Parse(_) => None,
        }
    }
}

impl From<IOError> for S100Error {
    fn from(err: IOError) -> S100Error {
        S100Error::IO(err)
    }
}

impl From<XmlParseError> for S100Error {
    fn from(err: XmlParseError) -> S100Error {
        S100Error::XML(err)
    }
}

impl S100Error {
    pub(crate) fn invalid_child<T>(node: Node) -> Result<T> {
        Err(S100Error::Parse(format!(
            "'{}' contains an invalid child node called '{}'",
            node.get_parent()
                .expect("not expecting root node")
                .get_name(),
            node.get_name()
        )))
    }

    pub(crate) fn invalid_enum<T>(node_name: &str, child_node_name: &str) -> Result<T> {
        Err(S100Error::Parse(format!(
            "'{}' received a node named '{}'",
            child_node_name, node_name
        )))
    }

    pub(crate) fn invalid_value<T>(node: Node) -> Result<T> {
        Err(S100Error::Parse(format!(
            "'{}' received an invalid value: '{}'",
            node.get_name(),
            node.get_content()
        )))
    }

    pub(crate) fn missing_attribute<T>(node: Node, child_name: &str) -> Result<T> {
        Err(S100Error::Parse(format!(
            "'{}' is missing an attribute called '{}'",
            node.get_name(),
            child_name
        )))
    }

    pub(crate) fn missing_child<T>(node: Node, child_name: &str) -> Result<T> {
        Err(S100Error::Parse(format!(
            "'{}' is missing a child node called '{}'",
            node.get_name(),
            child_name
        )))
    }
}

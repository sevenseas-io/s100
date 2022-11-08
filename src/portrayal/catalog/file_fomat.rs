use std::str::FromStr;

use crate::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileFormat {
    XML,
    SVG,
    XSLT,
    TTF,
    LUA,
}

impl FromStr for FileFormat {
    type Err = Error;

    fn from_str(input: &str) -> Result<FileFormat, Self::Err> {
        match input {
            "XML" => Ok(FileFormat::XML),
            "SVG" => Ok(FileFormat::SVG),
            "XSLT" => Ok(FileFormat::XSLT),
            "TTF" => Ok(FileFormat::TTF),
            "LUA" => Ok(FileFormat::LUA),
            _ => Error::invalid_enum("fileFormat", input),
        }
    }
}

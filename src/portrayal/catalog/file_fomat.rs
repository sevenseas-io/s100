use std::str::FromStr;

use crate::S100Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileFormat {
    XML,
    SVG,
    XSLT,
    TTF,
    LUA,
}

impl FromStr for FileFormat {
    type Err = S100Error;

    fn from_str(input: &str) -> Result<FileFormat, Self::Err> {
        match input {
            "XML" => Ok(FileFormat::XML),
            "SVG" => Ok(FileFormat::SVG),
            "XSLT" => Ok(FileFormat::XSLT),
            "TTF" => Ok(FileFormat::TTF),
            "LUA" => Ok(FileFormat::LUA),
            _ => S100Error::invalid_enum("fileFormat", input),
        }
    }
}

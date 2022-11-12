use std::str::FromStr;

use crate::S100Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileFormat {
    Xml,
    Svg,
    Xslt,
    Ttf,
    Lua,
}

impl FromStr for FileFormat {
    type Err = S100Error;

    fn from_str(input: &str) -> Result<FileFormat, Self::Err> {
        match input {
            "XML" => Ok(FileFormat::Xml),
            "SVG" => Ok(FileFormat::Svg),
            "XSLT" => Ok(FileFormat::Xslt),
            "TTF" => Ok(FileFormat::Ttf),
            "LUA" => Ok(FileFormat::Lua),
            _ => S100Error::invalid_enum("fileFormat", input),
        }
    }
}

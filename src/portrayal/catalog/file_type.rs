use std::str::FromStr;

use crate::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileType {
    AlertCatalog,
    AreaFill,
    ColorProfile,
    Font,
    LineStyle,
    Pixmap,
    Rule,
    StyleSheet,
    Symbol,
}

impl FromStr for FileType {
    type Err = Error;

    fn from_str(input: &str) -> Result<FileType, Self::Err> {
        match input {
            "Font" => Ok(FileType::Font),
            "AlertCatalog" => Ok(FileType::AlertCatalog),
            "AreaFill" => Ok(FileType::AreaFill),
            "LineStyle" => Ok(FileType::LineStyle),
            "Symbol" => Ok(FileType::Symbol),
            "ColorProfile" => Ok(FileType::ColorProfile),
            "Pixmap" => Ok(FileType::Pixmap),
            "Rule" => Ok(FileType::Rule),
            "StyleSheet" => Ok(FileType::StyleSheet),
            _ => Error::invalid_enum("fileType", input),
        }
    }
}

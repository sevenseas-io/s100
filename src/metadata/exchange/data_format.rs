use std::str::FromStr;

use crate::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataFormat {
    ISO8211,
    GML,
    HDF5,
    Undefined,
}

impl FromStr for DataFormat {
    type Err = Error;

    fn from_str(input: &str) -> Result<DataFormat, Self::Err> {
        match input {
            "ISO/IEC 8211" => Ok(DataFormat::ISO8211),
            "GML" => Ok(DataFormat::GML),
            "HDF5" => Ok(DataFormat::HDF5),
            "undefined" => Ok(DataFormat::Undefined),
            _ => Error::invalid_enum("dataFormat", input),
        }
    }
}

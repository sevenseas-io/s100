use std::str::FromStr;

use crate::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParameterType {
    Boolean,
    Integer,
    Double,
    String,
    Date,
}

impl FromStr for ParameterType {
    type Err = Error;

    fn from_str(input: &str) -> Result<ParameterType, Self::Err> {
        match input {
            "Boolean" => Ok(ParameterType::Boolean),
            "Integer" => Ok(ParameterType::Integer),
            "Double" => Ok(ParameterType::Double),
            "String" => Ok(ParameterType::String),
            "Date" => Ok(ParameterType::Date),
            _ => Error::invalid_enum("type", input),
        }
    }
}

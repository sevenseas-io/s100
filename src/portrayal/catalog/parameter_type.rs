use std::str::FromStr;

use crate::S100Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterType {
    Boolean,
    Integer,
    Double,
    String,
    Date,
}

impl FromStr for ParameterType {
    type Err = S100Error;

    fn from_str(input: &str) -> Result<ParameterType, Self::Err> {
        match input {
            "Boolean" => Ok(ParameterType::Boolean),
            "Integer" => Ok(ParameterType::Integer),
            "Double" => Ok(ParameterType::Double),
            "String" => Ok(ParameterType::String),
            "Date" => Ok(ParameterType::Date),
            _ => S100Error::invalid_enum("type", input),
        }
    }
}

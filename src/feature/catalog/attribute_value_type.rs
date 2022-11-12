use std::str::FromStr;

use crate::S100Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttributeValueType {
    Boolean,
    Enumeration,
    Integer,
    Real,
    Text,
    Date,
    Time,
    DateTime,
    TruncatedDate,
    URI,
    URL,
    URN,
}

impl FromStr for AttributeValueType {
    type Err = S100Error;

    fn from_str(input: &str) -> Result<AttributeValueType, Self::Err> {
        match input {
            "boolean" => Ok(AttributeValueType::Boolean),
            "enumeration" => Ok(AttributeValueType::Enumeration),
            "integer" => Ok(AttributeValueType::Integer),
            "real" => Ok(AttributeValueType::Real),
            "text" => Ok(AttributeValueType::Text),
            "date" => Ok(AttributeValueType::Date),
            "time" => Ok(AttributeValueType::Time),
            "dateTime" => Ok(AttributeValueType::DateTime),
            "S100_TruncatedDate" => Ok(AttributeValueType::DateTime),
            "URI" => Ok(AttributeValueType::URI),
            "URL" => Ok(AttributeValueType::URL),
            "URN" => Ok(AttributeValueType::URN),
            _ => S100Error::invalid_enum("valueType", input),
        }
    }
}

use std::str::FromStr;

use crate::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuleType {
    SubTemplate,
    TopLevelTemplate,
}

impl FromStr for RuleType {
    type Err = Error;

    fn from_str(input: &str) -> Result<RuleType, Self::Err> {
        match input {
            "SubTemplate" => Ok(RuleType::SubTemplate),
            "TopLevelTemplate" => Ok(RuleType::TopLevelTemplate),
            _ => Error::invalid_enum("ruleType", input),
        }
    }
}

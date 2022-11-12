use std::str::FromStr;

use crate::S100Error;

#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub enum RuleType {
    SubTemplate,
    TopLevelTemplate,
}

impl FromStr for RuleType {
    type Err = S100Error;

    fn from_str(input: &str) -> Result<RuleType, Self::Err> {
        match input {
            "SubTemplate" => Ok(RuleType::SubTemplate),
            "TopLevelTemplate" => Ok(RuleType::TopLevelTemplate),
            _ => S100Error::invalid_enum("ruleType", input),
        }
    }
}

use std::str::FromStr;

use crate::{feature::QUANTITY_SPECIFICATION, Error};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuantitySpecification {
    AngularVelocity,
    Area,
    Density,
    Duration,
    Frequency,
    Length,
    Mass,
    PlaneAngle,
    Power,
    Pressure,
    Salinity,
    Speed,
    Temperature,
    Volume,
    Weight,
    OtherQuantity,
}

impl FromStr for QuantitySpecification {
    type Err = Error;

    fn from_str(input: &str) -> Result<QuantitySpecification, Self::Err> {
        match input {
            "angularVelocity" => Ok(QuantitySpecification::AngularVelocity),
            "area" => Ok(QuantitySpecification::Area),
            "density" => Ok(QuantitySpecification::Density),
            "duration" => Ok(QuantitySpecification::Duration),
            "frequency" => Ok(QuantitySpecification::Frequency),
            "length" => Ok(QuantitySpecification::Length),
            "mass" => Ok(QuantitySpecification::Mass),
            "planeAngle" => Ok(QuantitySpecification::PlaneAngle),
            "power" => Ok(QuantitySpecification::Power),
            "pressure" => Ok(QuantitySpecification::Pressure),
            "salinity" => Ok(QuantitySpecification::Salinity),
            "speed" => Ok(QuantitySpecification::Speed),
            "temperature" => Ok(QuantitySpecification::Temperature),
            "volume" => Ok(QuantitySpecification::Volume),
            "weight" => Ok(QuantitySpecification::Weight),
            "otherQuantity" => Ok(QuantitySpecification::OtherQuantity),
            _ => Err(Error::Parse(format!(
                "Invalid enum value for '{}': {}",
                QUANTITY_SPECIFICATION, input
            ))),
        }
    }
}

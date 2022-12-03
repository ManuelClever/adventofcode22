mod dec01;
mod dec02;
mod dec03;

use std::fmt::Error;
use crate::solve::Solve;

pub fn create_calculator(day: &i8) -> Result<Box<dyn Solve>, Error> {
    match day {
        1 => {Ok(Box::new(dec01::Calculator {}))},
        2 => {Ok(Box::new(dec02::Calculator {}))},
        3 => {Ok(Box::new(dec03::Calculator {}))},
        _ => {Err(Error)}
    }
}
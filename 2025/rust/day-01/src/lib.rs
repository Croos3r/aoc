use std::fmt::{Display, Formatter};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Rotation {
    left: bool,
    degrees: u16,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.left { "L" } else { "R" }, self.degrees)
    }
}

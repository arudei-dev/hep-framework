use std::fmt;
use super::*;

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("");

        if self.imag < 0. {
            output += format!("{} - {}i", self.real, self.imag.abs()).as_str();
        }
        else {
            output += format!("{} + {}i", self.real, self.imag.abs()).as_str();
        }

        write!(f, "{}", output)
    }
}
use std::fmt;
use super::*;


impl fmt::Display for Mat44 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("\n");

        output += format!("| {}  {}  {}  {} |\n", self.content[0], self.content[1], self.content[2], self.content[3]).as_str();
        output += format!("| {}  {}  {}  {} |\n", self.content[4], self.content[5], self.content[6], self.content[7]).as_str();
        output += format!("| {}  {}  {}  {} |\n", self.content[8], self.content[9], self.content[10], self.content[11]).as_str();
        output += format!("| {}  {}  {}  {} |\n", self.content[12], self.content[13], self.content[14], self.content[15]).as_str();

        write!(f, "{}", output)
    }
}
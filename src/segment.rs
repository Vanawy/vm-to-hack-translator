use crate::segment::Segment::{Argument, Constant, Local, Pointer, Static, Temp, That, This};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SegmentParseError;

impl FromStr for Segment {
    type Err = SegmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "argument" => Ok(Argument),
            "local" => Ok(Local),
            "static" => Ok(Static),
            "constant" => Ok(Constant),
            "this" => Ok(This),
            "that" => Ok(That),
            "pointer" => Ok(Pointer),
            "temp" => Ok(Temp),
            _ => Err(SegmentParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_segments() {
        assert_eq!(Argument, "argument".parse().unwrap());
        assert_eq!(Local, "local".parse().unwrap());
        assert_eq!(Static, "static".parse().unwrap());
        assert_eq!(Constant, "constant".parse().unwrap());
        assert_eq!(This, "this".parse().unwrap());
        assert_eq!(That, "that".parse().unwrap());
        assert_eq!(Pointer, "pointer".parse().unwrap());
        assert_eq!(Temp, "temp".parse().unwrap());
    }
}

use std::str::FromStr;
use crate::segment::Segment::{Argument, Constant, Local, Pointer, Static, Temp, That, This};

#[derive(Debug, PartialEq, Eq)]
pub enum Segment
{
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp
}

#[derive(Debug, PartialEq, Eq)]
pub struct SegmentParseError;

impl FromStr for Segment
{
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
            _ => Err(SegmentParseError)
        }
    }
}
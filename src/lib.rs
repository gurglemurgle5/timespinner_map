use regex::Regex;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

pub mod level;
mod level_specification;
pub mod minimap;
pub mod texture_atlas;

pub use level_specification::load_map_from_file;

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "either \"True\" or \"False\"")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "True" => Ok(true),
            "False" => Ok(false),
            _ => panic!("too lazy for proper errors"),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "True" => Ok(true),
            "False" => Ok(false),
            _ => panic!("too lazy for proper errors"),
        }
    }
}

fn parse_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(BoolVisitor)
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PositionVisitor)
    }
}

struct PositionVisitor;
impl Visitor<'_> for PositionVisitor {
    type Value = Position;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("position data in the form of a string, such as {X:0 Y:1}")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let re = Regex::new(r"^\{X:(\d+) Y:(\d+)\}$").unwrap();
        match re.captures(v) {
            Some(captures) => Ok(Position {
                x: captures[1].parse().unwrap(),
                y: captures[2].parse().unwrap(),
            }),
            None => todo!(),
        }
    }
}

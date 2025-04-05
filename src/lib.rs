use regex::Regex;
use sdl2::rect::Point;
use serde::Deserializer;
use serde::de::Visitor;

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

fn parse_point<'de, D>(deserializer: D) -> Result<Point, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(PointVisitor)
}

struct PointVisitor;
impl Visitor<'_> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("position data in the form of a string, such as {X:0 Y:1}")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let re = Regex::new(r"^\{X:(\d+) Y:(\d+)\}$").unwrap();
        match re.captures(v) {
            Some(captures) => Ok(Point::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
            )),
            None => todo!(),
        }
    }
}

use anyhow::Result;
use flate2::read::ZlibDecoder;
use sdl2::rect::Point;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::parse_point;

#[derive(Debug, Deserialize)]
pub struct Minimap {
    #[serde(rename = "Areas")]
    pub area_list: AreaList,
    // reveal groups don't seem important for now so im skipping em
}

impl Minimap {
    pub fn load<T: AsRef<Path>>(timespinner_path: T) -> Result<Minimap> {
        let mut path = timespinner_path.as_ref().to_owned();
        path.extend(["Content", "Levels", "Minimap.dat"]);
        let file = File::open(path)?;
        let decoder = ZlibDecoder::new(file);
        let buf_reader = BufReader::new(decoder);
        Ok(quick_xml::de::from_reader(buf_reader)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct AreaList {
    #[serde(rename = "Area")]
    pub areas: Vec<Area>,
}

#[derive(Debug, Deserialize)]
pub struct Area {
    #[serde(rename = "@ID")]
    pub id: i32,
    #[serde(rename = "Rooms")]
    pub room_list: RoomList,
}

#[derive(Debug, Deserialize)]
pub struct RoomList {
    #[serde(rename = "Room")]
    pub rooms: Vec<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    #[serde(rename = "@ID")]
    pub id: i32,
    #[serde(rename = "@Width")]
    pub width: u32,
    #[serde(rename = "@Height")]
    pub height: u32,
    #[serde(rename = "@Position", deserialize_with = "parse_point")]
    pub position: Point,
    // skipping blocks, i don't really care about them for now, maybe later
}

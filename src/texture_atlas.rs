use anyhow::Result;
use flate2::read::ZlibDecoder;
use serde::Deserialize;
use std::io::BufReader;
use std::{fs::File, path::Path};

use crate::{Position, parse_bool};

#[derive(Debug, Deserialize)]
pub struct TextureDatabase {
    #[serde(rename = "Atlas")]
    pub atlases: Vec<Atlas>,
}

impl TextureDatabase {
    pub fn load<T: AsRef<Path>>(timespinner_path: T) -> Result<TextureDatabase> {
        let mut path = timespinner_path.as_ref().to_owned();
        path.extend(["Content", "TextureDatabase.dat"]);
        let file = File::open(path)?;
        let decoder = ZlibDecoder::new(file);
        let buf_reader = BufReader::new(decoder);
        Ok(quick_xml::de::from_reader(buf_reader)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Atlas {
    #[serde(rename = "@FileName")]
    pub file_name: String,
    #[serde(rename = "@ContentPath")]
    pub content_path: String,
    #[serde(rename = "@Width")]
    pub width: i32,
    #[serde(rename = "@Height")]
    pub height: i32,
    #[serde(rename = "@FrameCount")]
    pub frame_count: i32,
    #[serde(rename = "AtlasFrame")]
    pub frames: Vec<AtlasFrame>,
}

#[derive(Debug, Deserialize)]
pub struct AtlasFrame {
    #[serde(rename = "@DoesNewRowUseStartX", deserialize_with = "parse_bool")]
    pub does_new_row_use_start_x: bool,
    #[serde(rename = "@Count")]
    pub count: i32,
    #[serde(rename = "@RowWidth")]
    pub row_width: i32,
    #[serde(rename = "@StartIndex")]
    pub start_index: i32,
    #[serde(rename = "@FrameSize")]
    pub frame_size: Position,
    #[serde(rename = "@StartCoordinates")]
    pub start_coordinates: Position,
}

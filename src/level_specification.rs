use anyhow::Result;
use flate2::read::ZlibDecoder;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::level::{Level, Tile};
use crate::parse_bool;

#[derive(Debug, Deserialize)]
struct LevelSpecification {
    #[serde(rename = "@ID")]
    id: i32,
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "Rooms")]
    room_list: RoomList,
    // todo: backgrounds
}

#[derive(Debug, Deserialize)]
struct RoomList {
    #[serde(rename = "Room")]
    rooms: Vec<Room>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Room {
    #[serde(rename = "@ID")]
    id: i32,
    #[serde(rename = "@Index")]
    index: u32,
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@Tileset")]
    tileset: String,
    #[serde(rename = "@Width")]
    width: u32,
    #[serde(rename = "@Height")]
    height: u32,
    #[serde(rename = "@BackgroundWipeColor")]
    background_wipe_color: String,
    bottom_tiles: TileList,
    middle_tiles: TileList,
    top_tiles: TileList,
    object_tiles: ObjectTileList,
    // TODO: tile swath (however that's used)
    // TODO: backgrounds
}

impl From<Room> for crate::level::Room {
    fn from(value: crate::level_specification::Room) -> Self {
        crate::level::Room {
            id: value.id,
            index: value.index,
            name: value.name,
            tileset: value.tileset,
            width: value.width,
            height: value.height,
            background_wipe_color: value.background_wipe_color,
            bottom_tiles: value.bottom_tiles.tiles,
            middle_tiles: value.middle_tiles.tiles,
            top_tiles: value.top_tiles.tiles,
            object_tiles: value
                .object_tiles
                .tiles
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct TileList {
    #[serde(default, rename = "Tile")]
    tiles: Vec<Tile>,
}

#[derive(Debug, Deserialize)]
struct ObjectTileList {
    #[serde(default, rename = "Tile")]
    tiles: Vec<ObjectTile>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ObjectTile {
    #[serde(rename = "@ID")]
    pub id: i32,
    // skip layer, seems redundant
    #[serde(rename = "@X")]
    pub x: i32,
    #[serde(rename = "@Y")]
    pub y: i32,
    #[serde(default, rename = "@FlipX", deserialize_with = "parse_bool")]
    pub flip_x: bool,
    #[serde(default, rename = "@FlipY", deserialize_with = "parse_bool")]
    pub flip_y: bool,
    #[serde(rename = "@Category")]
    pub category: Category,
    #[serde(rename = "@ObjectID")]
    pub object_id: u8,
    #[serde(rename = "@Argument")]
    pub argument: Option<i32>,
}

impl From<ObjectTile> for crate::level::ObjectTile {
    fn from(value: ObjectTile) -> Self {
        crate::level::ObjectTile {
            id: value.id,
            x: value.x,
            y: value.y,
            flip_x: value.flip_x,
            flip_y: value.flip_y,
            category: match value.category {
                Category::None => crate::level::Category::None,
                Category::Event => {
                    crate::level::Category::Event(value.object_id.try_into().unwrap())
                }
                Category::Enemy => {
                    crate::level::Category::Enemy(value.object_id.try_into().unwrap())
                }
                Category::Item => crate::level::Category::Item(value.object_id.try_into().unwrap()),
            },
            argument: value.argument,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Category {
    None,
    Event,
    Enemy,
    Item,
}

pub fn load_map_from_file<T: AsRef<Path>>(path: T) -> Result<Level> {
    let file = File::open(path)?;
    let decoder = ZlibDecoder::new(file);
    let buf_reader = BufReader::new(decoder);
    let level: LevelSpecification = quick_xml::de::from_reader(buf_reader)?;

    Ok(Level {
        id: level.id,
        name: level.name,
        rooms: level.room_list.rooms.into_iter().map(Into::into).collect(),
    })
}

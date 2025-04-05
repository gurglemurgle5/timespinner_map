use num_enum::TryFromPrimitive;
use serde::Deserialize;

use crate::parse_bool;

#[derive(Clone, Debug, Default)]
pub struct Level {
    pub id: i32,
    pub name: String,
    pub rooms: Vec<Room>,
}

#[derive(Clone, Debug, Default)]
pub struct Room {
    pub id: i32,
    pub index: u32,
    pub name: String,
    pub tileset: String,
    pub width: u32,
    pub height: u32,
    pub background_wipe_color: String,
    pub bottom_tiles: Vec<Tile>,
    pub middle_tiles: Vec<Tile>,
    pub top_tiles: Vec<Tile>,
    pub object_tiles: Vec<ObjectTile>,
    // todo: tile swath (however that's used)
    // todo: backgrounds
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Tile {
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
}

impl From<ObjectTile> for Tile {
    fn from(value: ObjectTile) -> Self {
        Tile {
            id: value.id,
            x: value.x,
            y: value.y,
            flip_x: value.flip_x,
            flip_y: value.flip_y,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObjectTile {
    pub id: i32,
    // skip layer, seems redundant
    pub x: i32,
    pub y: i32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub category: Category,
    pub argument: Option<i32>,
}

#[derive(Clone, Copy, Debug)]
pub enum Category {
    None,
    Event(Event),
    Enemy(Enemy),
    Item(Item),
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Event {
    Checkpoint,
    PlayerStart,
    WestTeleport,
    NorthTeleport,
    EastTeleport,
    SouthTeleport,
    BossDoor,
    DummyUIEvent,
    JournalEntry,
    MovingPlatform,
    BlastDoor,
    CirclePlatform,
    MiniBossDoor,
    TransitionWarpEvent,
    ConveyorBelt,
    PetrifiedVine,
    WaterFillerNW,
    WaterFillerSE,
    Elevator,
    DonutTile,
    BreakableWall,
    JunkCrusher,
    MerchantCrow,
    Selen,
    TheTimespinner,
    OrbPedestal,
    TimeGate,
    TreasureChest,
    Doorway,
    KeycardDoor,
    Transition,
    Lantern,
    ForestNPCs,
    CurtainDrawbridge,
    TimespinnerWheelItem,
    LevelEffect,
    EnvironmentPrefab,
    AreaTitleBlocker,
    MapTerminal,
    BackerPortrait,
    Cutscene,
    MusicFader,
    MusicPlayer,
    GyrePortal,
    GyreSpawner,
    LostItem,
    Tutorial,
    RareEnemySpawner,
    EscortMissionManager,
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Enemy {
    CheveuxTank,
    BlueCheveux,
    RedCheveux,
    FlyingCheveux,
    KickstarterFoe,
    TempleFoe,
    CursedAnemone,
    CavesSlime,
    FortressEngineer,
    JunkSpawner,
    CavesCopperWyvern,
    CavesSiren,
    KeepDemon,
    CastleShieldKnight,
    CastleArcher,
    WormFlower,
    WormFlowerWalker,
    CeilingStar,
    FleshSpider,
    DiscStatue,
    CitySecurityGuard,
    CheveuxTower,
    ForestBabyCheveux,
    ForestMoth,
    ForestPlantBat,
    ForestRodent,
    ForestWormFlower,
    CavesMushroomTower,
    CavesSporeVine,
    CavesSnail,
    CastleLargeSoldier,
    CastleEngineer,
    KeepWarCheveux,
    KeepLanceKnight,
    KeepAristocrat,
    TowerPlasmaPod,
    TowerRoyalGuard,
    LakeBirdEgg,
    LakeAnemone,
    LakeCheveux,
    LakeEel,
    LakeFly,
    FortressKnight,
    FortressGunner,
    LabTurret,
    LabChild,
    LabAdult,
    FortressLargeSoldier,
    BirdBoss,
    RoboKittyBoss,
    VarndagrothBoss,
    AelanaBoss,
    IncubusBoss,
    MawBoss,
    ShapeshiftBoss,
    EmperorBoss,
    SandmanBoss,
    NightmareBoss,
    RavenBoss,
    XarionBoss,
    ZelBoss,
    CantoranBoss,
}

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Item {
    MaxHP,
    MaxMP,
    Orb1,
    DoubleJump,
    Dash,
    WhiteSheep,
    BlackSheep,
    GemChest,
}

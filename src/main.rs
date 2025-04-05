use clap::Parser;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::exit;
use timespinner_map::texture_atlas::TextureDatabase;
use timespinner_map::{
    Position,
    level::{Level, Tile},
    load_map_from_file,
    minimap::Minimap,
};

#[derive(Debug, Parser)]
struct Args {
    /// Path to your Timespinner installation directory
    timespinner_path: PathBuf,
}

const TILE_SIZE: i32 = 16;
const ROOM_WIDTH: i32 = 25;
const ROOM_HEIGHT: i32 = 20;

fn main() {
    let args = Args::parse();
    let mut state = State::new(args.timespinner_path);

    loop {
        state.update();
        state.draw();
        // using vsync should prevent going too fast
    }
}

struct State {
    minimap: Minimap,
    levels: HashMap<i32, Level>,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    camera: Point,
    texture_cache: TextureCache,
}

impl State {
    fn new<T: AsRef<Path>>(timespinner_path: T) -> State {
        eprintln!("loading minimap...");
        let minimap = Minimap::load(&timespinner_path).unwrap();
        eprintln!("done!");
        eprintln!("loading texture database...");
        let texture_database = TextureDatabase::load(&timespinner_path).unwrap();
        eprintln!("done!");

        let mut levels = HashMap::new();

        for area in &minimap.area_list.areas {
            let filename = match area.id {
                // TODO: should these be hardcoded? or is there a different way i should be finding these?
                17 => "Nexus.dat",
                18 => "Debug.dat",
                _ => &format!("Level_{:02}.dat", area.id),
            };
            let mut path = timespinner_path.as_ref().to_owned();
            path.extend(["Content", "Levels", filename]);
            eprintln!("loading level from {path:?}...");
            let level = load_map_from_file(path).unwrap();
            eprintln!("done!");
            levels.insert(area.id, level);
        }

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("timespinner map stuffs", 25 * 16, 20 * 16)
            .resizable()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .unwrap();

        let texture_creator = canvas.texture_creator();
        eprintln!("loading textures...");
        let texture_cache = TextureCache::new(texture_creator, texture_database);
        eprintln!("done!");

        let event_pump = sdl_context.event_pump().unwrap();
        State {
            minimap,
            levels,
            canvas,
            texture_cache,
            event_pump,
            camera: Point::new(0, 0),
        }
    }

    #[expect(clippy::match_single_binding)] // will fix later, don't want to deal with it now though
    fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => exit(0),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    _ => (),
                },
                Event::MouseWheel {
                    precise_x,
                    precise_y,
                    ..
                } => {
                    self.camera.x += (precise_x * 100.0) as i32;
                    self.camera.y -= (precise_y * 100.0) as i32;
                }
                Event::MouseMotion {
                    mousestate,
                    xrel,
                    yrel,
                    ..
                } => {
                    if mousestate.middle() || mousestate.left() {
                        self.camera.x -= xrel;
                        self.camera.y -= yrel;
                    }
                }
                _ => (),
            }
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let mut camera_rect = self.canvas.viewport();
        camera_rect.x = self.camera.x - (camera_rect.w / 2);
        camera_rect.y = self.camera.y - (camera_rect.h / 2);

        draw_grid(&mut self.canvas, camera_rect);

        for area in &self.minimap.area_list.areas {
            if let Some(level) = self.levels.get(&area.id) {
                'rooms: for room in &area.room_list.rooms {
                    for other in &level.rooms {
                        if room.id == other.id {
                            let bounding_rect = Rect::new(
                                room.position.x * TILE_SIZE * ROOM_WIDTH,
                                room.position.y * TILE_SIZE * ROOM_HEIGHT,
                                other.width * TILE_SIZE as u32,
                                other.height * TILE_SIZE as u32,
                            );
                            if !bounding_rect.has_intersection(camera_rect) {
                                continue 'rooms;
                            }

                            let tileset = self.texture_cache.load_texture(&other.tileset);
                            let frames = self.texture_cache.load_frames(&other.tileset);

                            for tile in &other.bottom_tiles {
                                draw_tile(
                                    &mut self.canvas,
                                    tile,
                                    tileset,
                                    room.position,
                                    camera_rect,
                                    frames,
                                );
                            }
                            for tile in &other.middle_tiles {
                                draw_tile(
                                    &mut self.canvas,
                                    tile,
                                    tileset,
                                    room.position,
                                    camera_rect,
                                    frames,
                                );
                            }
                            for tile in &other.top_tiles {
                                draw_tile(
                                    &mut self.canvas,
                                    tile,
                                    tileset,
                                    room.position,
                                    camera_rect,
                                    frames,
                                );
                            }
                        }
                    }
                }
            }
        }

        self.canvas.present();
    }
}

fn draw_grid(canvas: &mut Canvas<Window>, camera: Rect) {
    canvas.set_draw_color(Color::RGB(32, 32, 32));
    let mut x = camera.x / TILE_SIZE * TILE_SIZE - camera.x;
    if x < 0 {
        x += TILE_SIZE;
    }
    while x < camera.w {
        canvas.draw_line((x, 0), (x, camera.h)).unwrap();
        x += TILE_SIZE;
    }

    let mut y = camera.y / TILE_SIZE * TILE_SIZE - camera.y;
    if y < 0 {
        y += TILE_SIZE;
    }
    while y < camera.h {
        canvas.draw_line((0, y), (camera.w, y)).unwrap();
        y += TILE_SIZE;
    }

    canvas.set_draw_color(Color::RGB(0, 64, 0));
    let mut x = camera.x / (TILE_SIZE * ROOM_WIDTH) * (TILE_SIZE * ROOM_WIDTH) - camera.x;
    if x < 0 {
        x += TILE_SIZE * ROOM_WIDTH;
    }
    while x < camera.w {
        canvas.draw_line((x, 0), (x, camera.h)).unwrap();
        x += TILE_SIZE * ROOM_WIDTH;
    }

    let mut y = camera.y / (TILE_SIZE * ROOM_HEIGHT) * (TILE_SIZE * ROOM_HEIGHT) - camera.y;
    if y < 0 {
        y += TILE_SIZE * ROOM_HEIGHT;
    }
    while y < camera.h {
        canvas.draw_line((0, y), (camera.w, y)).unwrap();
        y += TILE_SIZE * ROOM_HEIGHT;
    }
}

fn draw_tile(
    canvas: &mut Canvas<Window>,
    tile: &Tile,
    tileset: &Texture,
    room_pos: Position,
    camera: Rect,
    frames: &[Rect],
) {
    if tile.id >= 512 {
        // TODO: tiles have special behaviour here
        return;
    }
    let tileset_rect = frames[tile.id as usize];

    let screen_rect = Rect::new(
        tile.x * TILE_SIZE - camera.x + room_pos.x * ROOM_WIDTH * TILE_SIZE,
        tile.y * TILE_SIZE - camera.y + room_pos.y * ROOM_HEIGHT * TILE_SIZE,
        TILE_SIZE as u32,
        TILE_SIZE as u32,
    );

    canvas
        .copy_ex(
            tileset,
            tileset_rect,
            screen_rect,
            0.0,
            None,
            tile.flip_x,
            tile.flip_y,
        )
        .unwrap();
}

struct TextureCache {
    textures: HashMap<String, Texture>,
    _texture_creator: TextureCreator<WindowContext>,
    name_to_frames: HashMap<String, Vec<Rect>>,
}

impl TextureCache {
    fn new(
        texture_creator: TextureCreator<WindowContext>,
        texture_database: TextureDatabase,
    ) -> TextureCache {
        let mut name_to_frames = HashMap::new();
        let mut textures = HashMap::new();
        for atlas in texture_database.atlases {
            let path = format!("./Content/{}.png", &atlas.content_path);
            textures.insert(
                atlas.file_name.clone(),
                texture_creator.load_texture(path).unwrap(),
            );

            // TODO: i don't think this is how frames work at all but it just happens to be correct for tiles
            let mut frames = Vec::new();
            for frame in atlas.frames {
                let (mut x, mut y) = (frame.start_coordinates.x, frame.start_coordinates.y);
                let (width, height) = (frame.frame_size.x, frame.frame_size.y);
                let mut col = 0;
                for _ in 0..frame.count {
                    frames.push(Rect::new(x, y, width as u32, height as u32));
                    x += width;
                    col += 1;
                    if col == frame.row_width {
                        col = 0;
                        if frame.does_new_row_use_start_x {
                            x = frame.start_coordinates.x;
                        } else {
                            // is this correct? idk
                            x = 0;
                        }
                        y += height;
                    }
                }
            }
            name_to_frames.insert(atlas.file_name, frames);
        }

        TextureCache {
            textures,
            _texture_creator: texture_creator,
            name_to_frames,
        }
    }
    fn load_texture(&self, name: &str) -> &Texture {
        self.textures.get(name).unwrap()
    }
    fn load_frames(&self, name: &str) -> &Vec<Rect> {
        self.name_to_frames.get(name).unwrap()
    }
}

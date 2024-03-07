#![allow(dead_code)]

use bevy::{math::ivec2, prelude::*};
use bevy_ascii_terminal::{
    Border, Terminal, TerminalBundle, TerminalPlugin, Tile, TiledCameraBundle,
};

pub const CLEAR_TILE: Tile = Tile {
    glyph: '·',
    bg_color: Color::BLACK,
    fg_color: Color::rgb(0.3, 0.3, 0.3),
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Nebula Vault"),
                    resolution: (800.0, 600.0).into(),
                    ..default()
                }),
                ..default()
            }),
            TerminalPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (render))
        .run();
}

#[derive(Component)]
pub struct TerminalMarker;

pub fn setup(mut commands: Commands) {
    let mut terminal = Terminal::new([10, 10]).with_border(Border::single_line());
    terminal.clear_tile = CLEAR_TILE;
    let term_bundle: TerminalBundle = TerminalBundle::from(terminal);
    commands.spawn((term_bundle, TerminalMarker));
    // Camera
    commands.spawn(
        TiledCameraBundle::pixel_cam([50, 50])
            .with_pixels_per_tile([1, 1])
            .with_clear_color(Color::rgb(0.1, 0.1, 0.1)),
    );

    let mut map = Map::new(10, 10);
    map
    commands.spawn(map);

    let mut player = Player {};
    commands.spawn((
        player,
        Renderable {
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
            glyph: '@',
        },
        Point { x: 5, y: 5 },
    ));
}

pub fn render(
    mut query_t: Query<&mut Terminal, With<TerminalMarker>>,
    mut query_map: Query<&mut Map>,
    query_e: Query<(&Point, &Renderable)>,
) {
    let mut term = query_t.single_mut();
    let mut map = query_map.single_mut();

    term.clear();
    for x in 0..map.width {
        for y in 0..map.height {
            term.put_tile(ivec2(x, y), CLEAR_TILE)
        }
    }
    for (p, r) in query_e.iter() {
        term.put_tile(ivec2(p.x, p.y), Tile::from(r));
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Renderable {
    pub fg_color: Color,
    pub bg_color: Color,
    pub glyph: char,
}

impl From<&Renderable> for Tile {
    fn from(value: &Renderable) -> Self {
        Tile {
            glyph: value.glyph,
            bg_color: value.bg_color,
            fg_color: value.fg_color,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Component)]
pub struct Map {
    tiles: Vec<TileType>,
    width: i32,
    height: i32,
    visible_tiles: Vec<bool>,
    revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            tiles: vec![TileType::Floor; size],
            width,
            height,
            visible_tiles: vec![false; size],
            revealed_tiles: vec![false; size],
        }
    }
    pub fn xy_idx(&self, point: Point) -> usize {
        ((point.y * self.width) + point.x) as usize
    }
}

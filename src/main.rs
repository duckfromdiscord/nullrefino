#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use notan::draw::*;
use notan::prelude::*;

pub use nullrefino::colld::*;
pub use nullrefino::loadtexture::loadtexture;
pub use nullrefino::drawfont::drawtext;
pub use nullrefino::drawfont::drawfps;
pub use nullrefino::round::round;
pub use nullrefino::clear::*;

use tetris_core_mod::*;



mod state;
mod board;
use crate::state::State;
use crate::board::TColor;
use crate::state::draw_bkgs;
use crate::state::draw_block;
use tetris_core_mod::geometry::Point;
pub use tetris_core_mod::game::{Game, Randomizer, Action};
pub use szo_randomizer::*;

use std::cell::RefCell;
use rand::thread_rng;

struct Rand {
    szo: RefCell<BagNoSZORandomizer>,
}
impl Randomizer for Rand {
    fn random(&self) -> i32 {
        let mut temp_szo = self.szo.borrow_mut();
        let random_number = temp_szo.bag_randomizer.next();
        random_number.try_into().unwrap()
    }
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let texture = loadtexture(gfx, include_bytes!("assets/n2.png"));
    let data = include_bytes!("assets/n2.json");
    let block_atlas = create_textures_from_atlas(data, &texture).unwrap();

    let texture = loadtexture(gfx, include_bytes!("assets/font.png"));
    let data = include_bytes!("assets/font.json");
    let font_atlas = create_textures_from_atlas(data, &texture).unwrap();

    let texture = loadtexture(gfx, include_bytes!("assets/font_small.png"));
    let data = include_bytes!("assets/font_small.json");
    let small_font_atlas = create_textures_from_atlas(data, &texture).unwrap();

    let board_outline = loadtexture(gfx, include_bytes!("assets/fieldoutline_alph.png"));
    

    let board: [[TColor; 20]; 10] = [[TColor::None; 20]; 10];

    let dt: f32 = 0f32;
    
    let paused = false;

    let pause_snd: AudioSource = app.audio.create_source(include_bytes!("assets/pause.wav")).unwrap();
    let move_snd: AudioSource = app.audio.create_source(include_bytes!("assets/move.wav")).unwrap();
    let last_key: Option<KeyCode> = None;

    let fps: std::string::String = "0.0".to_string();

    let dt_since_fps_refresh: f32 = 0f32;
    let frame_since_down: f32 = 0f32;
    let dt_since_left: f32 = 0f32;
    let dt_since_right: f32 = 0f32;
    let mut seed_rng = thread_rng();
    let szo = BagNoSZORandomizer::new([true, true, true, true, true, true, true, false, false, false, false], seed_rng.gen_range(0..99999999));
    let rand = Rand {
        szo: RefCell::new(szo),
    };
    rand.szo.borrow_mut().init();
    let game_size = Size { height: 20, width: 10 };
    let tgame = Game::new(&game_size, Box::new(rand));

    let clear_pipeline = clear_shader(gfx);

    State {
        block_atlas,
        font_atlas,
        small_font_atlas,
        bkg: loadtexture(gfx, include_bytes!("assets/back0.png")),
        board_bkg: loadtexture(gfx, include_bytes!("assets/fieldbg2_alph.png")),
        board_outline,
        score: 0,
        line: 0,
        board,
        dt,
        paused,
        pause_snd,
        move_snd,
        last_key,
        fps,
        dt_since_fps_refresh,
        frame_since_down,
        dt_since_left,
        dt_since_right,
        clear_pipeline,
        tgame,
    }
}



fn key_down(app: &mut App, key: KeyCode) -> bool {
    if app.keyboard.is_down(key) {
        return true;
    }
    false
}

fn key_down_first(app: &mut App, key: KeyCode) -> bool {
    if key_down(app, key) && app.keyboard.was_pressed(key) {
        return true;
    }
    false
}

fn key_down_time(app: &mut App, key: KeyCode, das: f32, time: f32) -> bool {
    if key_down(app, key) && (time >= das) {
        return true;
    }
    false
}

fn das(app: &mut App, state: &mut State, key: KeyCode, action: Action, das: f32, time: f32) -> f32 {
    let mut ftime = time;
    if key_down(app, key)  {
        ftime += app.timer.delta_f32();
    } else {
        ftime = 0f32;
    }
    
    if key_down_first(app, key) || key_down_time(app, key, das, time) {
        let x = state.tgame.access_active_figure()[0].x;
        state.tgame.perform(action);
        if state.tgame.access_active_figure()[0].x != x && !(key_down(app, KeyCode::Left) && key_down(app, KeyCode::Right)) { // do not make loud noise if both keys are pressed
            app.audio.play_sound(&state.move_snd, 0.8f32, false);
        }
    }


    ftime
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {

    state.last_key = app.keyboard.last_key_released();
    
    if key_down_first(app, KeyCode::Escape) {
        app.audio.play_sound(&state.pause_snd, 0.8f32, false);
        state.paused = !state.paused;
    }

    let mut draw = gfx.create_draw();

    draw.clear(Color::BLACK);


    draw_bkgs(&mut draw, app, state);

    if !state.paused {


        state.dt_since_left = das(app, state, KeyCode::Left, Action::MoveLeft, 14f32 * (1f32/60f32), state.dt_since_left);
        state.dt_since_right = das(app, state, KeyCode::Right, Action::MoveRight, 14f32 * (1f32/60f32), state.dt_since_right);
    
    
        if key_down_first(app, KeyCode::Z) {
            state.tgame.perform(Action::Rotate);
        }
    
        if app.keyboard.is_down(KeyCode::Down) {
            state.frame_since_down += 1f32;
            if state.frame_since_down % 3.0f32 == 0.0f32 {
                state.tgame.perform(Action::MoveDown);
            }
        } else {
            state.frame_since_down = 1f32;
        }
    
        if app.keyboard.is_down(KeyCode::Up) && app.keyboard.was_pressed(KeyCode::Up) {
            for _i in 0..20 {
                state.tgame.perform(Action::MoveDown);
            }
        }

        let lastframe = app.timer.delta_f32();
        state.dt += lastframe;
        state.dt_since_fps_refresh += app.timer.delta_f32();

        state.tgame.update(lastframe.into());
        state.score = state.tgame.get_score();
        state.line = state.tgame.get_lines_completed();
    

        for i in 0..20 {
            for j in 0..10 {
                state.board[j][i] = TColor::None;
            }
        }
        
        
        let game_blocks = state.tgame.draw(); // full board
        let piece_in_play = state.tgame.access_active_figure();
        let board_only = state.tgame.access_board();

        let boardvec: Vec<Point> = board_only;
        let blockvec: Vec<Point> = piece_in_play;

        let ghost: Vec<Point> = fit(boardvec, blockvec);

        for block in ghost {
            let blockx = block.x;
            let blocky = block.y;

            let xind = blockx as usize;
            let mut yind = blocky as usize;
            if yind > 19 {
                yind = 19;
            }
            state.board[xind][yind] = TColor::from_etoledom(state.tgame.active_figure_color().name).to_clear();
        }


        for block in game_blocks {
            let blockx = block.position().x;
            let blocky = block.position().y;

            let xind = blockx as usize;
            let yind = blocky as usize;

            state.board[xind][yind] = TColor::from_etoledom(block.color.name);
        }
    }

    let mut pixel_x: f32 = 36f32;
    let mut pixel_y: f32 = 84f32;

    for i in 0..20 {
        for j in 0..10 {
            draw_block(&mut draw, state, pixel_x, pixel_y, state.board[j][i]);
            pixel_x += 16f32;
        }
        pixel_x = 36f32;
        pixel_y += 16f32;
    }

    gfx.render(&draw);
}



#[notan_main]
fn main() -> Result<(), String> {
    
    notan::init_with(init)
        .add_config(WindowConfig::new().vsync(true).size(640, 480).title("Nullrefino")) // 640x480 for normal pc, 380x480 for phone but its cropped off a bit
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

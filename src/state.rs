use notan::prelude::Texture;
use std::collections::HashMap;
use notan::AppState;
use notan::prelude::*;
use notan::draw::*;

use tetris_core_mod::*;

pub use nullrefino::loadtexture::loadtexture;
pub use nullrefino::drawfont::drawtext;
pub use nullrefino::drawfont::drawfps;
pub use nullrefino::round::round;
pub use nullrefino::round::safediv;
pub use crate::board::TColor;


#[derive(AppState)]
pub struct State {
    pub block_atlas: HashMap<String, Texture>,
    pub font_atlas: HashMap<String, Texture>,
    pub small_font_atlas: HashMap<String, Texture>,
    pub bkg: Texture,
    pub board_bkg: Texture,
    pub board_outline: Texture,
    pub score: u64,
    pub line: usize,
    pub board: [[TColor; 20]; 10],
    pub dt: f32,
    pub paused: bool,
    pub pause_snd: AudioSource,
    pub last_key: Option<KeyCode>,
    pub fps: std::string::String,
    pub dt_since_fps_refresh: f32,
    pub frame_since_down: f32,
    pub dt_since_left: f32,
    pub dt_since_right: f32,
    pub tgame: Game,
}

pub fn time_to_stamp(time: f32) -> std::string::String {
    let min = (time / 60.0).floor();
    let sec = (time - min / 60.0).floor();


    let secfrac = ( ( (time - min / 60.0) - sec ) * 100.0 ).floor();

    
    let sec = sec - (min * 60.0);

    format!("{:02}:{:02}.{:02}", min, sec, secfrac)
}

pub fn draw_bkgs(draw: &mut Draw, app: &mut App, state: &mut State) {
        // Space background
        draw.image(&state.bkg).position(0f32,0f32);
    

        // Counters
        if state.dt_since_fps_refresh >= 2.0f32 {
            state.fps = format!("{}", round(app.timer.fps().into(), 1));
            state.dt_since_fps_refresh = 0.0f32;
        }
        drawfps(draw, &state.font_atlas, state.fps.to_string(), 0f32, 464f32); // TODO: make this update way less often.

        // Mode
        drawtext(draw, "yellow_", &state.font_atlas, "practice".into(), 248f32, 80f32, 16f32);

        // Score
        drawtext(draw, "blue_", &state.font_atlas, "score".into(), 248f32, 112f32, 16f32);
        drawtext(draw, "grey_", &state.font_atlas, format!("{}", state.score), 248f32, 130f32, 16f32);

        // Line
        drawtext(draw, "blue_", &state.font_atlas, "line".into(), 248f32, 160f32, 16f32);
        drawtext(draw, "grey_", &state.font_atlas, format!("{}", state.line), 248f32, 178f32, 16f32);

        // Score/min
        drawtext(draw, "blue_", &state.font_atlas, "score/min".into(), 248f32, 256f32, 16f32);
        drawtext(draw, "grey_", &state.font_atlas, format!("{}", safediv( state.score, &state.dt / 60f32 ) ), 248f32, 274f32, 16f32);

        // Line/min
        drawtext(draw, "blue_", &state.font_atlas, "line/min".into(), 248f32, 304f32, 16f32);
        drawtext(draw, "grey_", &state.font_atlas, format!("{}", safediv( state.line.try_into().unwrap(), &state.dt / 60f32 ) ), 248f32, 322f32, 16f32);

        // Time
        drawtext(draw, "blue_", &state.font_atlas, "time".into(), 248f32, 352f32, 16f32);
        drawtext(draw, "grey_", &state.font_atlas, time_to_stamp(state.dt), 248f32, 370f32, 16f32); // tired

        // Hold & next text
        drawtext(draw, "green_", &state.small_font_atlas, "hold".into(), 32f32, 32f32, 8f32);
        drawtext(draw, "yellow_", &state.small_font_atlas, "next".into(), 92f32, 32f32, 8f32);
    
    
        // Board
        draw.image(&state.board_bkg).position(36f32,84f32);
        draw.image(&state.board_outline).position(32f32, 80f32);
}

pub fn draw_block(draw: &mut Draw, state: &State, x: f32, y: f32, color: TColor) {
    if color.as_str() == "None" {
        return;
    }
    
    let blockimage = &state.block_atlas.get(color.as_str()).unwrap();
    if color.is_clear() {
        draw.image(blockimage).position(x,y).alpha(0.9f32);
        return;
    }
    draw.image(blockimage).position(x,y);
}
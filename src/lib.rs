pub mod loadtexture {
    pub fn loadtexture(gfx: &mut notan::prelude::Graphics,  bytes: &'static [u8]) -> notan::prelude::Texture {
        return gfx.create_texture()
            .from_image(bytes)
            .build()
            .unwrap();
    }
}

pub mod drawfont {

    use notan::draw::*;
    use std::collections::HashMap;
    use notan::prelude::*;

    pub fn drawfps(draw: &mut Draw, font_atlas: &HashMap<String, Texture>, text: std::string::String, x: f32, y: f32) {
        drawtext(draw, "fps_", font_atlas, text, x, y, 16f32);
    }


    pub fn drawtext(draw: &mut Draw, prefix: &str, font_atlas: &HashMap<String, Texture>, text: std::string::String, x: f32, y: f32, step: f32) {
        let mut localx: f32 = x;
        let individual_chars: Vec<char> = text.chars().collect();
        for c in individual_chars {
            //println!("{}", (prefix.to_owned() + &c.to_string()));
            // TODO: FIX THAT THIS SOMETIMES GETS A "grey_-"
            let letter = font_atlas.get(&(prefix.to_owned() + &c.to_string())).unwrap();
            draw.image(letter).position(localx, y);
            localx += step;
        }
    }

}

pub mod round {
    pub fn round(x: f64, pr: i32) ->  f64 {
        let n = 10.0_f64.powi(pr);
        (x * n).round() / (n)
    }
    pub fn safediv(x: u64, y: f32) -> f32 {
        if y <= 0.01f32 {
            0f32
        } else {
            x as f32 / y
        }
        
    }
}

pub mod colld {

    use tetris_core_mod::geometry::Point;

    pub fn pointequal(a: Point, b: Point) -> bool {
        if a.x == b.x && a.y == b.y {
            return true;
        }
        false
    }


    pub fn bvalidate(point: Point) -> bool {
        if point.x > 9 || point.x < 0 || point.y > 19 || point.y < 0 {
            return false;
        }
        true
    }

    pub fn intersect(a: &Vec<Point>, b: &Vec<Point>) -> bool {
        for bp in b {
            if !bvalidate(*bp) {
                return true;
            }
            for ap in a {
                if !bvalidate(*ap) {
                    return true;
                }
                if pointequal(*ap, *bp) {
                    return true;
                }
            }
        }
        false
    }



    pub fn translate(vec: Vec<Point>) -> Vec<Point> {
        let mut returnpoints = Vec::new();
        for p in vec {
            if p.y > 19 {
                returnpoints.push(Point {x: p.x, y: 19} );
            } else {
                returnpoints.push(Point {x: p.x, y: p.y + 1} );
            }
        }
        returnpoints
    }

    pub fn rtranslate(vec: Vec<Point>) -> Vec<Point> {
        let mut returnpoints = Vec::new();
        for p in vec {
            if p.y < 0 {
                returnpoints.push(Point {x: p.x, y: 0} );
            } else {
                returnpoints.push(Point {x: p.x, y: p.y - 1} );
            }
        }
        returnpoints
    }



    pub fn fit(board: Vec<Point>, block: Vec<Point>) -> Vec<Point> {
        let board = board;
        let mut block = block;
        while !intersect(&board, &block) {
            block = translate(block);
        }
        block = rtranslate(block);
        block
    }
}

pub mod clear;
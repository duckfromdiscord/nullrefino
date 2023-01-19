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
            return 0f32;
        } else {
            return x as f32 / y as f32;
        }
    }
}
use notan::draw::*;
use notan::prelude::*;


// @Nazariglez helped quite a bit for the majority of the pipeline code
// the actual filter is here: http://lolengine.net/blog/2013/07/27/rgb-to-hsv-in-glsl

//language=glsl
const FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(binding = 0) uniform sampler2D u_texture;

    layout(location = 0) out vec4 color;

    //http://lolengine.net/blog/2013/07/27/rgb-to-hsv-in-glsl
        vec3 rgb2hsv(vec3 c)
    {
        vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
        vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
        vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

        float d = q.x - min(q.w, q.y);
        float e = 1.0e-10;
        return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
    }


    vec3 hsv2rgb(vec3 c)
    {
        vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
        vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
        return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
    }

    vec3 darken(vec3 color) {
        vec4 a = vec4(rgb2hsv(color), 1.0);
        a.z *= 62.0/89.0;
        a = vec4(hsv2rgb(a.xyz), 1.0);
        return a.xyz;
    }

    void main() {
        vec4 c = texture(u_texture, v_uvs) * v_color;
        color = vec4(darken(c.xyz),1.0);
    }
    "#
};


pub fn clear_shader(gfx: &mut Graphics) -> Pipeline {
    let pipeline = create_image_pipeline(gfx, Some(&FRAGMENT)).unwrap();
    pipeline
}
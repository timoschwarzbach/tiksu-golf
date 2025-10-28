use std::ops::{Add, Div};

fn interpolate(a0: f32, a1: f32, x: f32) -> f32 {
    let g = (3.0 - x * 2.0) * x * x;
    (a1 - a0) * g + a0
}

struct Vec2 {
    x: f32,
    y: f32,
}

fn random_gradient(ix: isize, iy: isize) -> Vec2 {
    let w = 8 * size_of::<usize>();
    let s = w / 2;
    let mut a = ix as usize;
    let mut b = iy as usize;
    a = a.wrapping_mul(3284157443);
    b ^= (a << s) | (a >> (w - s));
    b = b.wrapping_mul(1911520717);
    a ^= (b << s) | (b >> (w - s));
    b = b.wrapping_mul(2048419325);
    let random = (a as f32) * (std::f32::consts::PI / (!(!0usize >> 1)) as f32);
    Vec2 {
        x: random.cos(),
        y: random.sin(),
    }
}

fn dot_grid_gradient(ix: isize, iy: isize, x: f32, y: f32) -> f32 {
    let grad = random_gradient(ix, iy);
    let dx = x - ix as f32;
    let dy = y - iy as f32;
    dx * grad.x + dy * grad.y
}

pub fn perlin(x: f32, y: f32) -> f32 {
    let x0 = x.floor() as isize;
    let x1 = x0 + 1;
    let y0 = y.floor() as isize;
    let y1 = y0 + 1;

    let sx = x - x0 as f32;
    let sy = y - y0 as f32;

    let ix0 = interpolate(
        dot_grid_gradient(x0, y0, x, y),
        dot_grid_gradient(x1, y0, x, y),
        sx,
    );
    let ix1 = interpolate(
        dot_grid_gradient(x0, y1, x, y),
        dot_grid_gradient(x1, y1, x, y),
        sx,
    );
    interpolate(ix0, ix1, sy)
}

pub fn layered(x: f32, y: f32) -> f32 {
    perlin(x / 2.0, y / 2.0) * 0.5
        + perlin(x / 8.0, y / 8.0) * 4.0
        + perlin(x / 32.0, y / 32.0).asin().asin().asin() * 32.0
}

pub fn layered_with_mountains(x: f32, y: f32) -> f32 {
    let x = x * 0.2;
    let y = y * 0.2;

    let base = layered(x, y);

    let mountain_height = perlin(x / 100.0, y / 100.0).add(1.0).div(1.5).powi(8);
    //* (perlin(x / 50.0, y / 50.0).powi(8) * 0.6);
    let mountain_mul = 50.0; // + perlin(x / 40.0, y / 40.0) * 50.0;

    base /*+ mountain_height * mountain_mul*/
}

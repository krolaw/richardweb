//use std::ops::Add;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     // The `console.log` is quite polymorphic, so we can bind it with multiple
//     // signatures. Note that we need to use `js_name` to ensure we always call
//     // `log` in JS.
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_f64(a: f64);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_f32(a: f32);

//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    points: u32, 
    ratio: f32,
    rotate: f32,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let mut data = get_chaos_set(width, height, points as usize, ratio as f64, rotate as f64);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {

}

fn get_chaos_set(width: u32, height: u32, point_num: usize, ratio: f64, rotate: f64) -> Vec<u8> {
    //log("blast");

    let mut rnd = WyRand{seed: 0};

    let mut data = vec![0; (width * height) as usize * 4];

    let mut points = Vec::with_capacity(point_num);
    let radius = (height as f64) / 2.2;
    let centre = Point{ x: (width as f64) / 2.0, y: (height as f64) / 2.0 };
    //log(&format!("centre x:{}, y:{}", centre.x, centre.y));
    for i in 0..point_num {
        let theta = i as f64 * 2.0*PI/point_num as f64;
        //log(&format!("i:{} theta:{} cos:{} sin:{}", i,  theta, theta.cos(), theta.sin()));
        //log_f32(theta as f32);
        let pt = Point{x: centre.x+(theta.sin()*radius), y: centre.y-(theta.cos()*radius)};
        //log(&format!("point x:{}, y:{}", pt.x, pt.y));
        points.push(pt);
    }

    let mut pt = points[0];

    let rotate = rotate*PI/180.0;
    let sin_rot = rotate.sin();
    let cos_rot = rotate.cos();
    

    let mut count = 0;
    while count < 100 {
        count += 1;
        let rpt = points[rnd.range(point_num as u64) as usize];
        pt = Point{
            x: pt.x + (rpt.x-pt.x)*ratio,
            y: pt.y + (rpt.y-pt.y)*ratio,
        };
        if rotate != 0.0 {
            pt.x = (pt.x - rpt.x) * cos_rot - (pt.y - rpt.y) * sin_rot + rpt.x;
            pt.y = (pt.x - rpt.x) * sin_rot + (pt.y - rpt.y) * cos_rot + rpt.y;
        }
        if pt.x < 0.0 || pt.y < 0.0 || pt.y > height as f64 || pt.x > width as f64 {
            continue
        }
        let pos = 4 * (pt.y as usize *width as usize + pt.x as usize);
        if data[pos+3] == 0 {
            count = 0;
        }
        data[pos+3] = 255;
        //data[pos..pos+4].clone_from_slice(&[0,0,0,255]);
    }



    // for p in &points {
    //     let w = width as usize *4;
    //     let pos = 4 * ((p.y as usize * width as usize) + p.x as usize);
    //     data[pos-w..pos-w+4].clone_from_slice(&[255,0,0,255]);
    //     data[pos-2*w..pos-2*w+4].clone_from_slice(&[255,0,0,255]);
    //     data[pos+2*w..pos+2*w+4].clone_from_slice(&[255,0,0,255]);
    //     data[pos+w..pos+w+4].clone_from_slice(&[255,0,0,255]);
    //     data[pos..pos+4].clone_from_slice(&[0,0,255,255]);
    // }

    // for i in 0..width as usize {
    //     let pos = 4 * (i * width as usize + i);
    //     data[pos..pos+4].clone_from_slice(&[0,255,0,255]);
    // }
    data
}

pub struct WyRand {
    pub seed: u64,
}

const P0: u64 = 0xa076_1d64_78bd_642f;
const P1: u64 = 0xe703_7ed1_a0b4_28db;

impl WyRand {
    pub fn rand(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(P0);
        let r = u128::from(self.seed) * u128::from(self.seed ^ P1);
        ((r >> 64) ^ r) as u64
    }

    pub fn range(&mut self, max: u64) -> u64 {
        self.rand() % max
    }
}
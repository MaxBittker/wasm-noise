mod utils;

use noise::{NoiseFn, Perlin};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Perlin2D {
    generator: Perlin,
}

#[wasm_bindgen]
impl Perlin2D {
    pub fn get(&mut self, x: f64, y: f64) -> f64 {
        self.generator.get([x, y])
    }
    pub fn get3(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.generator.get([x, y, z])
    }
    pub fn new() -> Perlin2D {
        Perlin2D {
            generator: Perlin::new(),
        }
    }
}

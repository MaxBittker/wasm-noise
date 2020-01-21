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
    pub fn tick(&mut self, x: f64, y: f64) -> f64 {
        return self.generator.get([x, y]);
    }
    pub fn new() -> Perlin2D {
        return Perlin2D {
            generator: Perlin::new(),
        };
    }
}

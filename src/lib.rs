extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate itertools;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// wasm-bindgen for facilitating high-level interactions between wasm modules and JavaScript.
#[wasm_bindgen(js_namespace = console)]
extern "C" {
    fn log(s: &str);
}

#[wasm_bindgen(js_namespace = console)]
pub fn navigate(x_es: Vec<i32>, y_es: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    (x_es, y_es)
}

mod navigation_service {
    struct Point {
        x: i32,
        y: i32
    }
    struct GraphRelation {
        vertex_index: i32,
        cost: i32,
    }
    struct Vertex {
        coordinates: Point,
        grpahs: Vec<GraphRelation>,
    }
    pub struct Dijkstra {
        coords: Vec<Point>,
        costs: crate::HashMap<i32, i32>,
        parents: crate::HashMap<i32, i32>,
        dijkstraVertexMatrix: Vec<Vertex>,
        start_point_index: i32,
        end_poiint_index: i32,
        processed: Vec<i32>,
        cheapest_vertex_index: i32,
    }
    impl Dijkstra {
        pub fn is_same_length(x_vec: &Vec<i32>, y_vec: &Vec<i32>) -> bool {
            x_vec.len() == y_vec.len()
        }
        pub fn new(&self, coords_x: Vec<i32>, coords_y: Vec<i32>) -> Result<&'static str, &'static str> {
            if !Dijkstra::is_same_length(&coords_x, &coords_y) {
                return Err("Coordinates do not match, lists are having unequal length");
            }
            let mut coords: Vec<i32>;
            for (x, y) in izip!(&coords_x, &coords_y) {
                coords.push(Point {x, y});
            }
            Ok("ok")
        }
    }
}

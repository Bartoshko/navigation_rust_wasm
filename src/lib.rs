extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate itertools;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
// use std::collections::HashMap;

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

struct Point {
    x: i32,
    y: i32
}
struct Line {
    start: Point,
    finish: Point,
}

#[wasm_bindgen(js_namespace = console)]
pub fn navigate(x_es: Vec<i32>, y_es: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    (x_es, y_es)
}

mod navigation_service {
    struct GraphRelation {
        vertex_index: i32,
        cost: i32,
    }
    struct Vertex {
        coordinates: super::Point,
        grpahs: Vec<GraphRelation>,
    }
    pub struct Dijkstra {
        lines: Vec<super::Line>,
    }
    impl Dijkstra {
        pub fn is_same_length(x_vec: &Vec<super::Point>, y_vec: &Vec<super::Point>) -> bool {
            x_vec.len() == y_vec.len()
        }
        pub fn new(starting_points: Vec<super::Point>, ending_points: Vec<super::Point>) -> Result<Dijkstra, &'static str> {
            if !Dijkstra::is_same_length(&starting_points, &ending_points) {
                return Err("Coordinates do not match, lists are having unequal length");
            }
            let coords: Vec<super::Line> = vec![];
            for (start, finish) in izip!(starting_points, ending_points) {
                coords.push(super::Line {start, finish});
            }
            Ok(Dijkstra{lines: coords})
        }
        pub fn calculate_shortest_path(&self, start_point: super::Point, end_point: super::Point) -> Vec<super::Line> {
            //let mut costs: crate::HashMap<i32, i32>,
            //let mut parents: crate::HashMap<i32, i32>,
            //let mut dijkstra_vertex_matrix: Vec<Vertex>,
            //let mut start_point_index: i32,
            //let mut end_poiint_index: i32,
            //let mut processed: Vec<i32>,
            //let mut cheapest_vertex_index: i32,
            let result_path = self.lines.clone();
            result_path
        }
    }
}

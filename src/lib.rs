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

pub struct Point {
    x: i32,
    y: i32
}
pub struct Line {
    start: Point,
    finish: Point,
}

#[wasm_bindgen(js_namespace = console)]
pub fn navigate(x_es_start: Vec<i32>, y_es_start: Vec<i32>, x_es_end: Vec<i32>, y_es_finish: Vec<i32>) -> 
    (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
    let (starts, ends) = navigation_service::Dijkstra::set_coords_numbers_to_points(&x_es_start, &y_es_start, &x_es_end, &y_es_finish);
    let dijkstra_result: Result<navigation_service::Dijkstra, &str> = navigation_service::Dijkstra::new(starts, ends);
    (x_es_start, y_es_start, x_es_end, y_es_finish)
    // TODO: act in regards to dijkstra_result, create Dijkstra or return zero length path
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
        pub fn set_coords_numbers_to_points(x_es_start: &Vec<i32>, y_es_start: &Vec<i32>, x_es_end: &Vec<i32>, y_es_finish: &Vec<i32>) ->
        (Vec<super::Point>, Vec<super::Point>) {
            //TODO: calculate start and finish Points 
        }
        pub fn is_same_length(x_vec: &Vec<super::Point>, y_vec: &Vec<super::Point>) -> bool {
            x_vec.len() == y_vec.len()
        }
        pub fn new(starting_points: Vec<super::Point>, ending_points: Vec<super::Point>) -> Result<Dijkstra, &'static str> {
            if !Dijkstra::is_same_length(&starting_points, &ending_points) {
                return Err("Coordinates do not match, lists are having unequal length");
            }
            let mut coords: Vec<super::Line> = vec![];
            for (start, finish) in izip!(starting_points, ending_points) {
                coords.push(super::Line {start, finish});
            }
            Ok(Dijkstra{lines: coords})
        }
        // pub fn calculate_shortest_path(&self, start_point: super::Point, end_point: super::Point) -> Vec<super::Line> {
        //     //let mut costs: crate::HashMap<i32, i32>,
        //     //let mut parents: crate::HashMap<i32, i32>,
        //     //let mut dijkstra_vertex_matrix: Vec<Vertex>,
        //     //let mut start_point_index: i32,
        //     //let mut end_poiint_index: i32,
        //     //let mut processed: Vec<i32>,
        //     //let mut cheapest_vertex_index: i32,
        //     let result_path = self.lines.clone();
        //     result_path
        // }
    }
}

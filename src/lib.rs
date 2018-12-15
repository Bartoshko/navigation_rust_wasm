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
    y: i32,
}
pub struct Line {
    start: Point,
    finish: Point,
}

pub fn navigate(x_es_start: Vec<i32>, y_es_start: Vec<i32>,
    x_es_finish: Vec<i32>, y_es_finish: Vec<i32>, starting_position: [i32; 2], targeted_position: [i32; 2]) -> 
    (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
    let dijkstra_result: Result<navigation_service::Dijkstra, &str> = navigation_service::Dijkstra::new(
        x_es_start, y_es_start, x_es_finish, y_es_finish
    );
    let point_to_start_from = Point {x :starting_position[0], y: starting_position[1]};
    let point_to_calc_path_to = Point {x :targeted_position[0], y: targeted_position[1]};
    let dijkstra: navigation_service::Dijkstra;
    if dijkstra_result.is_err() {
        println!("{:?}", dijkstra_result.is_err());
        let r0: Vec<i32> = vec![0];
        let r1: Vec<i32> = vec![0];
        let r2: Vec<i32> = vec![0];
        let r3: Vec<i32> = vec![0];
        return (r0, r1, r2, r3);
    }
    dijkstra = dijkstra_result.unwrap();
    dijkstra.calculate_shortest_path(point_to_start_from, point_to_calc_path_to)
}

mod navigation_service {
    struct GraphRelation {
        vertex_index: i32,
        cost: i32,
    }
    struct Vertex {
        coordinates: crate::Point,
        grpahs: Vec<GraphRelation>,
    }
    pub struct Dijkstra {
        lines: Vec<crate::Line>,
    }
    impl Dijkstra {
        pub fn is_same_length(x_s: &Vec<i32>, y_s: &Vec<i32>, x_e: &Vec<i32>, y_e: &Vec<i32>) -> bool {
            x_s.len() + y_s.len() == x_e.len() + y_e.len() && y_e.len() == y_s.len()
        }
        pub fn new(x_start: Vec<i32>, y_start: Vec<i32>, x_finish: Vec<i32>, y_finish: Vec<i32>) -> Result<Dijkstra, &'static str> {
            if !Dijkstra::is_same_length(&x_start, &y_start, &x_finish, &y_finish) {
                return Err("Coordinates do not match, lists are having unequal length");
            }
            let mut coords: Vec<crate::Line> = vec![];
            for (x_0, y_0, x_1, y_1) in izip!(x_start, y_start, x_finish, y_finish) {
                let point_0 = crate::Point {x: x_0, y: y_0};
                let point_1 = crate::Point {x: x_1, y: y_1};
                coords.push(crate::Line {start: point_0, finish: point_1});
            }
            Ok(Dijkstra{lines: coords})
        }
        pub fn calculate_shortest_path(&self, starting_position: crate::Point, final_destination: crate::Point) -> 
        (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
            //let mut costs: crate::HashMap<i32, i32>,
            //let mut parents: crate::HashMap<i32, i32>,
            //let mut dijkstra_vertex_matrix: Vec<Vertex>,
            //let mut start_point_index: i32,
            //let mut end_poiint_index: i32,
            //let mut processed: Vec<i32>,
            //let mut cheapest_vertex_index: i32,
            self.destruct_lines_to_tuple()
        }
        fn destruct_lines_to_tuple(&self) -> (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) {
            let mut destruct_result: (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) = (vec![], vec![], vec![], vec![]);
            for item in self.lines.iter() {
                destruct_result.0.push(item.start.x);
                destruct_result.1.push(item.start.y);
                destruct_result.2.push(item.finish.x);
                destruct_result.3.push(item.finish.y);
            }
            destruct_result
        }
    }
}

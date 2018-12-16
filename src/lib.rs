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

pub struct Point {
    x: i32,
    y: i32,
}
pub struct Line {
    start: Point,
    finish: Point,
}

#[wasm_bindgen()]
pub fn create_maze(nodes_num: i32) -> Vec<i32> {
    let mut maze: Vec<i32> = Vec::new();
    maze
}

#[wasm_bindgen()]
pub fn navigate(given_maze: Vec<i32>, starting_position: Vec<i32>, targeted_position: Vec<i32>) -> Vec<i32> {
    let dijkstra_result: Result<navigation_service::Dijkstra, &str> = navigation_service::Dijkstra::new(given_maze);
    let point_to_start_from = Point {x :starting_position[0], y: starting_position[1]};
    let point_to_calc_path_to = Point {x :targeted_position[0], y: targeted_position[1]};
    let dijkstra: navigation_service::Dijkstra;
    if dijkstra_result.is_err() {
        println!("{:?}", dijkstra_result.is_err());
        return vec![starting_position[0], starting_position[1]]
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
        costs: crate::HashMap<i32, i32>,
        parents: crate::HashMap<i32, i32>,
        dijkstra_vertex_matrix: Vec<Vertex>,
        start_point_index: i32,
        end_point_index: i32,
        processed: Vec<i32>,
        cheapest_vertex_index: i32,
    }
    impl Dijkstra {
        pub fn is_same_length(maze_mess: &Vec<i32>) -> bool {
            maze_mess.len() % 4 == 0
        }
        pub fn set_maze_mess_to_lines_order(maze_mess: Vec<i32>) -> Vec<crate::Line> {
            let mut lines_ordered: Vec<crate::Line> = Vec::new();
            let mut counter: i32 = 0;
            let mut x_s: i32 = 0;
            let mut y_s: i32 = 0;
            let mut x_f: i32 = 0;
            for item in &maze_mess {
                counter += 1;
                match counter {
                    1 => x_s = *item,
                    2 => y_s = *item,
                    3 => x_f = *item,
                    4 => {
                        counter = 0;
                        lines_ordered.push(
                            crate::Line {start: crate::Point {x: x_s, y: y_s}, finish: crate::Point {x: x_f, y: *item}}
                        );
                    },
                    _ => (),
                }
            }
            lines_ordered
        }
        pub fn new(maze_v: Vec<i32>) -> Result<Dijkstra, &'static str> {
            if !Dijkstra::is_same_length(&maze_v) {
                return Err("Coordinates do not match, lists are having unequal length");
            }
            let maze_lines: Vec<crate::Line> =  Dijkstra::set_maze_mess_to_lines_order(maze_v);
            Ok(Dijkstra{
                lines: maze_lines,
                costs: crate::HashMap::new(),
                parents: crate::HashMap::new(),
                dijkstra_vertex_matrix: Vec::new(),
                start_point_index: 0, //TODO from method to find index of start point and end point
                end_point_index: 0,
                processed: Vec::new(),
                cheapest_vertex_index: 0 // TODO is the same as start point index
            })
        }
        pub fn calculate_shortest_path(&self, starting_position: crate::Point, final_destination: crate::Point) -> Vec<i32> {
            self.path_to_vector()
        }
        fn path_to_vector(&self) -> Vec<i32> {
            //TODO: for now this will return lines but in future this will return calculate shortest path
            let mut path_vectorized: Vec<i32> = Vec::new();
            for _line in &self.lines {
                let coord_x_s: i32 = _line.start.x;
                let coord_y_s: i32 = _line.start.y;
                let coord_x_f: i32 = _line.finish.x;
                let coord_y_f: i32 = _line.finish.y;
                path_vectorized.push(coord_x_s);
                path_vectorized.push(coord_y_s);
                path_vectorized.push(coord_x_f);
                path_vectorized.push(coord_y_f);

            }
            path_vectorized
        }
    }
}

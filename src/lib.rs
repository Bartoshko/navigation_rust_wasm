extern crate cfg_if;
extern crate wasm_bindgen;

mod maze_creator;
mod navigation_service;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen()]
pub fn create_maze(path_lines_num: u32, branches_num: u32) -> Vec<i32> {
    let mock_empty_vec: Vec<i32> = vec![1];
    // let mut line_maze: maze_creator::LineMaze = maze_creator::LineMaze::new(path_lines_num, branches_num);
    // let v: &Vec<navigation_service::Line> = line_maze.create():
    mock_empty_vec
}

#[wasm_bindgen()]
pub fn navigate(
    given_maze: Vec<i32>,
    starting_position: Vec<i32>,
    targeted_position: Vec<i32>,
) -> Vec<i32> {
    if navigation_service::is_correct_length(&given_maze) {
        return vec![starting_position[0], starting_position[1]];
    }
    let maze_lines: Vec<navigation_service::Line> = navigation_service::vector_to_path(given_maze);
    let new_dijkstra_result: Result<navigation_service::Dijkstra, &str> =
        navigation_service::Dijkstra::new(maze_lines);
    let point_to_start_from = navigation_service::Point {
        x: starting_position[0],
        y: starting_position[1],
    };
    let point_to_calc_path_to = navigation_service::Point {
        x: targeted_position[0],
        y: targeted_position[1],
    };
    let dijkstra: navigation_service::Dijkstra;
    if new_dijkstra_result.is_err() {
        return vec![starting_position[0], starting_position[1]];
    }
    dijkstra = new_dijkstra_result.unwrap();
    navigation_service::path_to_vector(
        &dijkstra.calculate_shortest_path(point_to_start_from, point_to_calc_path_to),
    )
}

// This module is testing correctness and speed of Dijkstra Algorithm wrritten in Rust.
use navigation_service;
extern crate rand;
use maze_creator::rand::{thread_rng, Rng};

pub struct LineMaze {
    lines_num: u32,
    branches_num: u32,
    lines: Vec<navigation_service::Line>,
    shortest_path: Vec<navigation_service::Line>,
    start_point: navigation_service::Point,
    finish_point: navigation_service::Point,
}
impl LineMaze {
    fn generagte_random_i32_in_range(_a: i32, _z: i32) -> i32 {
        let mut rng = thread_rng();
        rng.gen_range(_a, _z)
    }
    fn generate_random_vector_in_range(_i: i32, _j: i32) -> navigation_service::Point {
        navigation_service::Point {
            x: LineMaze::generagte_random_i32_in_range(_i, _j),
            y: LineMaze::generagte_random_i32_in_range(_i, _j),
        }
    }
    pub fn new(lines_num: u32, branches_num: u32) -> LineMaze {
        LineMaze {
            lines_num,
            branches_num,
            lines: Vec::new(),
            shortest_path: Vec::new(),
            start_point: LineMaze::generate_random_vector_in_range(-100, 100),
            finish_point: navigation_service::Point { x: 0, y: 0 },
        }
    }
    pub fn create(&mut self) {
        self.lines = vec![];
        self.calculate_maze();
    }
    fn calculate_maze(&mut self) {
        let loc_lines_num = self.lines_num;
        self.finish_point = self.create_branch(&loc_lines_num);
        for line in &self.lines {
            self.shortest_path.push(line.copy());
        }
        for _ in 0..self.branches_num {
            let loc_steps_num: u32 = self.lines_num;
            let penultimate_point: navigation_service::Point = self.create_branch(&loc_steps_num);
            self.lines.push(navigation_service::Line {
                start: penultimate_point.copy(),
                finish: self.finish_point.copy(),
            });
        }
    }
    fn create_branch(&mut self, steps_num_iter: &u32) -> navigation_service::Point {
        let local_vector: navigation_service::Point =
            LineMaze::generate_random_vector_in_range(-10, 10);
        let mut a_point: navigation_service::Point;
        let mut b_point: navigation_service::Point = self.start_point.add(&local_vector);
        for n in 0..*steps_num_iter {
            let deviation_vector: navigation_service::Point =
                LineMaze::generate_random_vector_in_range(-2, 2);
            if n == 0 {
                a_point = self.start_point.copy();
                b_point = a_point.add(&local_vector).add(&deviation_vector);
            } else {
                a_point = b_point.copy();
                b_point = a_point.add(&local_vector).add(&deviation_vector);
            }
            self.lines.push(navigation_service::Line {
                start: a_point.copy(),
                finish: b_point.copy(),
            });
        }
        b_point
    }
}

#[cfg(test)]
mod test_maze {
    #[test]
    fn generates_random_vector() {
        let mut v_1_x_sum: i32 = 0;
        let mut v_2_x_sum: i32 = 0;
        for _ in 0..10 {
            let v_1 = super::LineMaze::generate_random_vector_in_range(-10, 10);
            let v_2 = super::LineMaze::generate_random_vector_in_range(-10, 10);
            v_1_x_sum += v_1.x;
            v_2_x_sum += v_2.x;
        }
        assert_ne!(v_1_x_sum, v_2_x_sum);
    }
    #[test]
    fn generate_maze_and_check_shortest_path() {
        let mut maze: super::LineMaze = super::LineMaze::new(100, 200);
        maze.create();
        let shortest_path: &Vec<super::navigation_service::Line> = &maze.shortest_path;
        let paths: &Vec<super::navigation_service::Line> = &maze.lines;
        let paths_1: &Vec<super::navigation_service::Line> = &maze.lines;
        // test shortest path length
        assert_eq!(shortest_path.len(), 100);
        // test all paths are having greater num than shortest path num
        assert!(paths.len() / (200 - 1) > shortest_path.len());
        // test shortest path is borrowed as referance
        assert!(paths_1.len() == paths.len());
    }
}

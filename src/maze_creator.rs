use navigation_service;
extern crate rand;
use maze_creator::rand::distributions::{IndependentSample, Range};

pub struct LineMaze {
    lines_num: i32,
    branches_num: i32,
    lines: Vec<navigation_service::Line>,
}
impl LineMaze {
    fn generagte_random_i32_in_range(_a: i32, _z: i32) -> i32 {
        let between = Range::new(_a, _z);
        let mut rng = rand::thread_rng();
        between.ind_sample(&mut rng)
    }
    pub fn new(lines_num: i32, branches_num: i32) -> LineMaze {
        LineMaze {
            lines_num,
            branches_num,
            lines: Vec::new(),
        }
    }
    pub fn create(&mut self) -> Vec<i32> {
        let mut lines_mess: Vec<i32> = Vec::new();
        self.create_path_branch();
        lines_mess
    }
    fn create_path_branch(&mut self) {
        let first_p: navigation_service::Point = navigation_service::Point {
            x: LineMaze::generagte_random_i32_in_range(1, 100),
            y: LineMaze::generagte_random_i32_in_range(1, 100),
        };
        let mut a_point: navigation_service::Point;
        let mut b_point: navigation_service::Point = navigation_service::Point {
            x: first_p.x +  LineMaze::generagte_random_i32_in_range(-10, 10),
            y: first_p.y +  LineMaze::generagte_random_i32_in_range(-10, 10),
        };
        for n in 0..self.branches_num {
            if n == 0 {
                 a_point = navigation_service::Point {
                    x: first_p.x,
                    y: first_p.y,
                 };
                 b_point = navigation_service::Point {
                    x: a_point.x +  LineMaze::generagte_random_i32_in_range(-10, 10),
                    y: a_point.y +  LineMaze::generagte_random_i32_in_range(-10, 10),
                 };
            } else {
                a_point = navigation_service::Point {
                    x: b_point.x,
                    y: b_point.y,
                 };
                 b_point = navigation_service::Point {
                    x: a_point.x +  LineMaze::generagte_random_i32_in_range(-10, 10),
                    y: a_point.y +  LineMaze::generagte_random_i32_in_range(-10, 10),
                 };
            }
            self.lines.push(navigation_service::Line{
                start: navigation_service::Point{
                    x: a_point.x,
                    y: a_point.y,
                },
                finish: navigation_service::Point{
                    x: b_point.x,
                    y: b_point.y,
                },
            })
        }
    }
}

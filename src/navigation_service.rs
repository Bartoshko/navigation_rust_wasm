use std::collections::HashMap;

pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn copy(&self) -> Point {
        let point_copy = Point {
            x: self.x,
            y: self.y,
        };
        point_copy
    }
    pub fn add(&self, other: &Point) -> Point {
        let point_copy = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
        point_copy
    }
}

pub struct Line {
    pub start: Point,
    pub finish: Point,
}
impl Line {
    pub fn copy(&self) -> Line {
        let line_copy = Line {
            start: Point {
                x: self.start.x,
                y: self.start.y,
            },
            finish: Point {
                x: self.finish.x,
                y: self.finish.y,
            },
        };
        line_copy
    }
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
    lines: Vec<Line>,
    costs: HashMap<i32, i32>,
    parents: HashMap<i32, i32>,
    dijkstra_vertex_matrix: Vec<Vertex>,
    start_point_index: i32,
    end_point_index: i32,
    processed: Vec<i32>,
    cheapest_vertex_index: i32,
}
impl Dijkstra {
    pub fn new(maze_lines: Vec<Line>) -> Result<Dijkstra, &'static str> {
        Ok(Dijkstra {
            lines: maze_lines,
            costs: HashMap::new(),
            parents: HashMap::new(),
            dijkstra_vertex_matrix: Vec::new(),
            start_point_index: 0, //TODO from method to find index of start point and end point
            end_point_index: 0,
            processed: Vec::new(),
            cheapest_vertex_index: 0, // TODO is the same as start point index
        })
    }
    pub fn calculate_shortest_path(
        &self,
        starting_position: Point,
        final_destination: Point,
    ) -> Vec<Line> {
        let result: Vec<Line> = Vec::new();
        result
    }
}

pub fn path_to_vector(lines: &Vec<Line>) -> Vec<i32> {
    let mut path_vectorized: Vec<i32> = Vec::new();
    for _line in lines {
        path_vectorized.push(_line.start.x);
        path_vectorized.push(_line.start.y);
        path_vectorized.push(_line.finish.x);
        path_vectorized.push(_line.finish.y);
    }
    path_vectorized
}

pub fn vector_to_path(maze_mess: Vec<i32>) -> Vec<Line> {
    let mut lines_ordered: Vec<Line> = Vec::new();
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
                lines_ordered.push(Line {
                    start: Point { x: x_s, y: y_s },
                    finish: Point { x: x_f, y: *item },
                });
            }
            _ => (),
        }
    }
    lines_ordered
}

pub fn is_same_length(maze_mess: &Vec<i32>) -> bool {
    maze_mess.len() % 4 == 0
}

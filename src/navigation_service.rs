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
impl Vertex {
    fn is_equal(&self, other: &Point) -> bool {
        self.coordinates.x == other.x && self.coordinates.y == other.y
    }
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
        if !is_correct_line_set(&maze_lines) {
            return Err("Wrong lines format.");
        }
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
        &mut self,
        starting_position: Point,
        final_destination: Point,
    ) -> Vec<Line> {
        let mut result: Vec<Line> = Vec::new();
        if is_same_point(&starting_position, &final_destination) {
            result.push(Line {
                start: starting_position.copy(),
                finish: starting_position.copy(),
            });
            return result;
        }
        self.create_vertex_matrix();
        result
    }

    fn create_vertex_matrix(&mut self) {
        let lines_length: usize = self.lines.len();
        for l_index in  0..lines_length { 
            let line = self.lines[l_index].copy();
            &mut self.append_to_vertex_matrix(line);
        }
    }

    fn append_to_vertex_matrix(&mut self, line: Line) {
        // ToDo: implement adding line to vertex matrixs
        let start_vertex_index_option: Option<usize> = self.dijkstra_vertex_matrix
            .iter().position(|r| r.is_equal(&line.start));
        let end_vertex_index_option: Option<usize> = self.dijkstra_vertex_matrix
            .iter().position(|r| r.is_equal(&line.finish));
        let start_vertex_index: i32 = match start_vertex_index_option {
            Some(v) => v as i32,
            None => self.add_new_vertex(line.start.copy()),
        };
        let end_vertex_index_option: i32 = match end_vertex_index_option {
            Some(v) => v as i32,
            None => self.add_new_vertex(line.finish.copy()),
        };
    }

    fn add_new_vertex(&mut self, coordinates: Point) -> i32 {
        self.dijkstra_vertex_matrix.push(Vertex {
            coordinates: coordinates,
            grpahs: Vec::new(),
        });
        self.dijkstra_vertex_matrix.len() as i32
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

pub fn is_correct_length(maze_mess: &Vec<i32>) -> bool {
    maze_mess.len() % 4 == 0
}

pub fn is_correct_line_set(lines: &Vec<Line>) -> bool {
    if lines.len() == 0 {
        return false;
    }
    for line in lines {
        if is_same_point(&line.start, &line.finish) {
            return false;
        }
    }
    true
}

pub fn is_same_point(p_1: &Point, p_2: &Point) -> bool {
    if p_1.x == p_2.x && p_1.y == p_2.y {
        return true;
    }
    false
}

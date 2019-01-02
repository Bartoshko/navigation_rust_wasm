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

    pub fn is_same(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
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

    pub fn length(&self) -> f64 {
        let distance_x: f64 = (self.start.x - self.finish.x) as f64;
        let distance_y: f64 = (self.start.y - self.finish.y) as f64;
        (distance_x.powi(2) + distance_y.powi(2)).sqrt()
    }
}

struct GraphRelation {
    vertex_index: i32,
    cost: f64,
}

pub struct Vertex {
    coordinates: Point,
    graphs: Vec<GraphRelation>,
}
impl Vertex {
    fn is_equal(&self, other: &Point) -> bool {
        self.coordinates.x == other.x && self.coordinates.y == other.y
    }
}

pub fn calculate_vertex_matrix(lines: &Vec<Line>) -> Vec<Vertex> {
    let mut dijkstra_vertex_matrix: Vec<Vertex> = Vec::new();
    let lines_length: usize = lines.len();
    for l_index in 0..lines_length {
        let line = lines[l_index].copy();
        append_to_vertex_matrix(line, &mut dijkstra_vertex_matrix);
    }
    dijkstra_vertex_matrix
}

fn append_to_vertex_matrix(line: Line, mut dijkstra_vertex_matrix: &mut Vec<Vertex>) {
    let start_vertex_index_option: Option<usize> = dijkstra_vertex_matrix
        .iter()
        .position(|r| r.is_equal(&line.start));
    let end_vertex_index_option: Option<usize> = dijkstra_vertex_matrix
        .iter()
        .position(|r| r.is_equal(&line.finish));
    let start_vertex_index: i32 = match start_vertex_index_option {
        Some(v) => v as i32,
        None => add_new_vertex(line.start.copy(), &mut dijkstra_vertex_matrix),
    };
    let end_vertex_index: i32 = match end_vertex_index_option {
        Some(v) => v as i32,
        None => add_new_vertex(line.finish.copy(), &mut dijkstra_vertex_matrix),
    };
    let cost: f64 = line.length();
    update_vertex_matrix(
        &start_vertex_index,
        &end_vertex_index,
        &cost,
        &mut dijkstra_vertex_matrix,
    );
    update_vertex_matrix(
        &end_vertex_index,
        &start_vertex_index,
        &cost,
        &mut dijkstra_vertex_matrix,
    );
}

fn add_new_vertex(coordinates: Point, dijkstra_vertex_matrix: &mut Vec<Vertex>) -> i32 {
    dijkstra_vertex_matrix.push(Vertex {
        coordinates: coordinates,
        graphs: Vec::new(),
    });
    (dijkstra_vertex_matrix.len() - 1) as i32
}

fn update_vertex_matrix(
    index_to_update: &i32,
    index_related: &i32,
    cost: &f64,
    dijkstra_vertex_matrix: &mut Vec<Vertex>,
) {
    let i_update: i32 = *index_to_update;
    let i_related: i32 = *index_related;
    let loc_cost: f64 = *cost;
    if dijkstra_vertex_matrix[i_update as usize]
        .graphs
        .iter()
        .position(|rel| rel.vertex_index == i_related)
        .is_none()
    {
        &mut dijkstra_vertex_matrix[i_update as usize]
            .graphs
            .push(GraphRelation {
                vertex_index: i_related,
                cost: loc_cost,
            });
    }
}

pub struct Dijkstra {
    costs: HashMap<i32, f64>,
    parents: HashMap<i32, i32>,
    dijkstra_vertex_matrix: Vec<Vertex>,
    start_point_index: i32,
    end_point_index: i32,
    processed: Vec<i32>,
    cheapest_vertex_index: i32,
}
impl Dijkstra {
    pub fn new(dijkstra_vertex_matrix: Vec<Vertex>) -> Dijkstra {
        Dijkstra {
            costs: HashMap::new(),
            parents: HashMap::new(),
            dijkstra_vertex_matrix: dijkstra_vertex_matrix,
            start_point_index: -1,
            end_point_index: -1,
            processed: Vec::new(),
            cheapest_vertex_index: -1,
        }
    }
    pub fn update_vertex(&mut self, dijkstra_vertex_matrix: Vec<Vertex>) {
        self.dijkstra_vertex_matrix = dijkstra_vertex_matrix
    }
    pub fn calculate_shortest_path(
        &mut self,
        start_point: &Point,
        finish_point: &Point,
    ) -> Vec<Line> {
        // TODO: check if point is on the given vertex
        let result = Vec::new();
        self.costs = HashMap::new();
        self.parents = HashMap::new();
        self.processed = Vec::new();
        self.cheapest_vertex_index = -1;
        self.create_vertex_beginning_params(start_point, finish_point);
        self.search_for_shortest_path();
        self.calculate_path_from_parents_schema(result)
    }

    fn create_vertex_beginning_params(
        &mut self,
        starting_position: &Point,
        final_destination: &Point,
    ) {
        self.start_point_index = self.get_index_from_vertex(&starting_position);
        self.end_point_index = self.get_index_from_vertex(&final_destination);
        self.costs.insert(self.start_point_index, 0.0);
        self.costs.insert(self.end_point_index, std::f64::MAX);
        self.processed.push(self.start_point_index);
        self.parents.insert(self.end_point_index, -1);
        self.cheapest_vertex_index = self.start_point_index;
    }

    fn get_index_from_vertex(&self, point: &Point) -> i32 {
        let found_i: i32 = self
            .dijkstra_vertex_matrix
            .iter()
            .position(|v| v.coordinates.is_same(point))
            .unwrap() as i32;
        found_i
    }

    fn calculate_path_from_parents_schema(&self, mut result: Vec<Line>) -> Vec<Line> {
        let mut actual_index_from_parent: i32 = self.end_point_index;
        let mut current_start_point: Point;
        let mut current_end_point: Point = self.dijkstra_vertex_matrix
            [self.end_point_index as usize]
            .coordinates
            .copy();
        while actual_index_from_parent != self.start_point_index {
            actual_index_from_parent = self.parents[&actual_index_from_parent];
            current_start_point = self.dijkstra_vertex_matrix[actual_index_from_parent as usize]
                .coordinates
                .copy();
            result.push(Line {
                start: current_start_point.copy(),
                finish: current_end_point.copy(),
            });
            current_end_point = current_start_point.copy();
        }
        result.reverse();
        result
    }

    fn search_for_shortest_path(&mut self) {
        while !self.processed.contains(&self.end_point_index) {
            let mut vertex_index: i32;
            let iteration_max: i32 = self.dijkstra_vertex_matrix
                [self.cheapest_vertex_index as usize]
                .graphs
                .len() as i32;
            for graph_index in 0..iteration_max {
                vertex_index = self.dijkstra_vertex_matrix[self.cheapest_vertex_index as usize]
                    .graphs[graph_index as usize]
                    .vertex_index;
                if !self.processed.contains(&vertex_index) {
                    let _parent_cost: f64 = self.costs[&self.cheapest_vertex_index];
                    let _graph_cost: f64 = self.dijkstra_vertex_matrix
                        [self.cheapest_vertex_index as usize]
                        .graphs[graph_index as usize]
                        .cost;
                    let _child_cost: f64 = _parent_cost + _graph_cost;
                    if self.costs.contains_key(&vertex_index) {
                        if self.costs[&vertex_index] > _child_cost {
                            *self.costs.get_mut(&vertex_index).unwrap() = _child_cost;
                            *self.parents.get_mut(&vertex_index).unwrap() =
                                self.cheapest_vertex_index;
                        }
                    } else {
                        self.costs.insert(vertex_index, _child_cost);
                        self.parents
                            .insert(vertex_index, self.cheapest_vertex_index);
                    }
                }
            }
            let mut min_cost = std::f64::MAX;
            let mut min_value_index: i32 = -1;
            for (k, v) in &self.costs {
                if !self.processed.contains(k) {
                    if min_cost > *v {
                        min_cost = *v;
                        min_value_index = *k;
                    }
                }
            }
            if min_value_index > -1 {
                self.cheapest_vertex_index = min_value_index;
                self.processed.push(self.cheapest_vertex_index);
            }
        }
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
        if line.start.is_same(&line.finish) {
            return false;
        }
    }
    true
}

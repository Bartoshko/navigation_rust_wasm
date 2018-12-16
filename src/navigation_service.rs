use std::collections::HashMap;

pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub struct Line {
    pub start: Point,
    pub finish: Point,
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
    pub fn is_same_length(maze_mess: &Vec<i32>) -> bool {
        maze_mess.len() % 4 == 0
    }
    pub fn set_maze_mess_to_lines_order(maze_mess: Vec<i32>) -> Vec<Line> {
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
                    lines_ordered.push(
                        Line {start: Point {x: x_s, y: y_s}, finish: Point {x: x_f, y: *item}}
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
        let maze_lines: Vec<Line> =  Dijkstra::set_maze_mess_to_lines_order(maze_v);
        Ok(Dijkstra{
            lines: maze_lines,
            costs: HashMap::new(),
            parents: HashMap::new(),
            dijkstra_vertex_matrix: Vec::new(),
            start_point_index: 0, //TODO from method to find index of start point and end point
            end_point_index: 0,
            processed: Vec::new(),
            cheapest_vertex_index: 0 // TODO is the same as start point index
        })
    }
    pub fn calculate_shortest_path(&self, starting_position: Point, final_destination: Point) -> Vec<i32> {
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

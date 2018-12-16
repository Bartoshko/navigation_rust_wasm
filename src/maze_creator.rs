use navigation_service;


pub struct LineMaze {
    lines_num: i32,
    branches_num: i32,
    lines: Vec<navigation_service::Line>,
}
impl LineMaze {
    pub fn new(lines_num: i32, branches_num: i32) -> LineMaze {
        LineMaze {
            lines_num,
            branches_num,
            lines: Vec::new(),
        }
    }
    pub fn create(&self) -> Vec<i32> {
        let mut lines_mess: Vec<i32> = Vec::new();
        lines_mess
    }
    // fn create_path_branch(&mut self) {

    // }
}
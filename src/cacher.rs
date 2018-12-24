struct Cacher<T> 
    where T: Fn(i32) -> i32 {
    calculations: T,
    result: i32
}
impl Cacher {
    fn new(calculations: T) -> Catcher<T> {
        calculations,
        result: None
    }
    fn result(&mut self, value: i32) -> i32 {
        match self.result {
            Some(r) => r,
            None => {
                let r = (self.calculations)(value);
                self.result = r;
                r
            }
        }
    }
}

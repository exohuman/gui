

#[allow(dead_code)]
pub trait Window {
    fn create(width: i32, height: i32) -> Self;
    fn show(&self) -> String;
    fn render_loop(&self);
}

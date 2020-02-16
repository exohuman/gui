
pub type WindowOnCreateFnPtr = fn () -> ();
pub type WindowOnShowFnPtr = fn () -> ();
pub type WindowOnRenderFnPtr = fn () -> ();

#[allow(dead_code)]
#[derive(Copy,Clone)]
pub struct WindowConfig {
    pub width: i32,
    pub height: i32,
    pub default_bg_color: u32,
    pub application_name: &'static str,
    pub title: &'static str,
    pub on_create: WindowOnCreateFnPtr,
    pub on_pre_show: WindowOnShowFnPtr,
    pub on_post_show: WindowOnShowFnPtr,
    pub on_pre_render: WindowOnRenderFnPtr,
    pub on_post_render: WindowOnRenderFnPtr,
}


#[allow(dead_code)]
pub trait Window {
    fn create(config: WindowConfig) -> Self;
    fn show(&self) -> String;
    fn render_loop(&self);
}


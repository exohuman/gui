use windower::create_window;
use windower::window::{Window, WindowConfig};

pub fn start_demo() {
    let window = create_window(WindowConfig {
        width: 640, 
        height: 480,
        default_bg_color: 0,
        title: "demo\0",
        application_name: "demo\0",
        on_create: || (),
        on_pre_show: || (),
        on_post_show: || (),
        on_pre_render: || (),
        on_post_render: || (),
    });
    window.show();
    window.render_loop();
}

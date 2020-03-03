extern crate windower;

use windower::create_window;
use windower::window::{Window, WindowConfig};

fn main() {
    let window = create_window(WindowConfig {
        width: 640, 
        height: 480,
        default_bg_color: 0,
        application_name: "Basic Window\0",
        title: "Basic Window\0",
        on_create: || (),
        on_pre_show: || (),
        on_post_show: || (),
        on_pre_render: || {
        },
        on_post_render: || (),
    });
    window.show();
    window.render_loop();
}

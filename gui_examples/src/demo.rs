use windower::create_window;
use windower::window::{Window, WindowConfig};

pub fn start_demo() {
    let window = create_window(WindowConfig {
        width: 640, 
        height: 480,
        default_bg_color: 0,
        title: "demo",
        application_name: "demo",
    });
    window.show();
    window.render_loop();
}

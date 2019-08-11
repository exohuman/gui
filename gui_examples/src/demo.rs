use windower::create_window;
use windower::window::Window;

pub fn start_demo() {
    let window = create_window(640, 480);
    window.show();
    window.render_loop();
}

extern crate windower;

use windower::create_window;
use windower::window::Window;

fn main() {
    let window = create_window(640, 480);
    window.show();
    window.render_loop();
}

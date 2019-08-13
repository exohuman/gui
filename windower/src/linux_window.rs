use xcb;
use crate::window::{Window, WindowConfig};

#[cfg(unix)]
#[allow(dead_code)]
pub struct LinuxWindow {
    config: WindowConfig,
    connection: xcb::Connection,
    screen_num: i32,
    window_id: u32,
}


impl LinuxWindow {
    fn create_linux_window(config: WindowConfig) -> Self {
        // connect to the x server
        let (connection, screen_num) = xcb::Connection::connect(None).unwrap();

        // get the setup from the connection to get the screens
        let setup = connection.get_setup();

        // iterate through the available screens to get the screen to draw on
        let screen = setup.roots().nth(screen_num as usize).unwrap(); 
        
        // generate an xid
        let window_id = connection.generate_id();

        // we use event masks to specify what events we want to capture from the window
        let event_masks = (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS);

        // specify the background color
        let background_value = (xcb::CW_BACK_PIXEL, screen.black_pixel());
        
        // we package up the configuration values that will be send to the window
        let values = [
            background_value,
            event_masks,
        ];

        // create the window
        xcb::create_window(
            &connection, 
            xcb::COPY_FROM_PARENT as u8,
            window_id, 
            screen.root(), 
            0, 0, 
            config.width as u16, config.height as u16,
            10, 
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(),
            &values);

        // change the title of the window
        let title = "A window in linux";
        xcb::change_property(
            &connection, 
            xcb::PROP_MODE_REPLACE as u8, 
            window_id, 
            xcb::ATOM_WM_NAME, 
            xcb::ATOM_STRING,
            8,
            title.as_bytes());

        LinuxWindow {
            config: config,
            connection: connection,
            screen_num: screen_num,
            window_id: window_id
        }
    }

    fn show_linux_window(&self) {
        // map the window to the screen
        xcb::map_window(&self.connection, self.window_id);

        // flush our changes to the window
        self.connection.flush();
    }

    fn render_loop_linux(&self) {
        let (wm_delete, _wm_protocols) = self.listen_for_wm_close_event();
        loop {
            // block until an event or error or io error occurs
            let event = self.connection.wait_for_event().unwrap();
            match event.response_type() & !0x80 {
                xcb::KEY_PRESS => {
                    // we break so that the window closes when a key is pressed
                    println!("A key was pressed and we are closing things for now");
                    break;
                },
                xcb::CLIENT_MESSAGE => {
                    let cm = unsafe {
                        xcb::cast_event::<xcb::ClientMessageEvent>(&event)
                    };
                    if cm.data().data32()[0] == wm_delete {
                        println!("Closing the app due to the X button being hit");
                        break;
                    }
                },
                _ => {
                    // for every other event we do nothing for the moment
                }
            }
            // write our changes to the window
            self.connection.flush();
        }
        xcb::destroy_window(&self.connection, self.window_id);
    }

    fn listen_for_wm_close_event(&self) -> (u32, u32) {
        // get "cookies" needed to handle the window manager events such as DELETE (which closes the window)
        let wm_delete_cookie = xcb::intern_atom(&self.connection, false, "WM_DELETE_WINDOW");
        let wm_protocols_cookie = xcb::intern_atom(&self.connection, false, "WM_PROTOCOLS");

        let wm_delete_reply = match wm_delete_cookie.get_reply() {
            Ok(wm_delete_reply) => wm_delete_reply.atom(),
            Err(_) => panic!("could not load WM_PROTOCOLS atom")
        };

        let wm_protocols_reply = match wm_protocols_cookie.get_reply() {
            Ok(wm_protocols_reply) => wm_protocols_reply.atom(),
            Err(_) => panic!("could not load WM_DELETE_WINDOW atom")
        };

        // register to listen for the window closed event
        let data = [ wm_delete_reply ];
        xcb::change_property(&self.connection, xcb::PROP_MODE_REPLACE as u8, self.window_id, wm_protocols_reply, 4, 32, &data);

        (wm_delete_reply, wm_protocols_reply)
    }
}


impl Window for LinuxWindow {
    fn create(config: WindowConfig) -> Self {
        LinuxWindow::create_linux_window(config)
    }

    fn show(&self) -> String {
        self.show_linux_window();
        String::from("done")
    }

    fn render_loop(&self) {
        self.render_loop_linux();
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}
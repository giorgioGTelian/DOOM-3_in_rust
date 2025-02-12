use std::cell::RefCell;
use std::rc::Rc;

struct BindWindow {
    dc: Option<Rc<RefCell<DeviceContext>>>,
    gui: Rc<RefCell<UserInterfaceLocal>>,
    bind_name: String,
    waiting_on_key: bool,
}

impl BindWindow { //TODO ADD TESTING
    fn new(dc: Option<Rc<RefCell<DeviceContext>>>, gui: Rc<RefCell<UserInterfaceLocal>>) -> Self {
        let mut bind_window = BindWindow {
            dc,
            gui,
            bind_name: String::new(),
            waiting_on_key: false,
        };
        bind_window.common_init();
        bind_window
    }

    fn common_init(&mut self) {
        self.bind_name = String::new();
        self.waiting_on_key = false;
    }

    fn handle_event(&mut self, event: &SysEvent, update_visuals: &mut bool) -> String {
        if !(event.ev_type == EventType::Key && event.ev_value2) {
            return String::new();
        }

        let key = event.ev_value;

        if self.waiting_on_key {
            self.waiting_on_key = false;
            if key == Key::Escape {
                return format!("clearbind \"{}\"", self.bind_name);
            } else {
                return format!("bind {} \"{}\"", key, self.bind_name);
            }
        } else {
            if key == Key::Mouse1 {
                self.waiting_on_key = true;
                self.gui.borrow_mut().set_bind_handler(self);
                return String::new();
            }
        }

        String::new()
    }

    fn get_win_var_by_name(&self, name: &str, fixup: bool, owner: Option<&mut DrawWin>) -> Option<&String> {
        if name.eq_ignore_ascii_case("bind") {
            return Some(&self.bind_name);
        }

        None
    }

    fn post_parse(&mut self) {
        self.bind_name = self.gui.borrow().get_state_dict().get_string("bind").unwrap_or_default();
        self.bind_name = self.gui.borrow().get_state_dict().get_string("bind").unwrap_or_default(); // TODO  `set_gui_info` and `update` are handled here
        self.flags |= WIN_HOLDCAPTURE | WIN_CANFOCUS;
    }

    fn draw(&self, time: i32, x: f32, y: f32) {
        let mut color = self.fore_color;

        let str = if self.waiting_on_key {
            common::get_language_dict().get_string("#str_07000")
        } else if !self.bind_name.is_empty() {
            self.bind_name.clone()
        } else {
            common::get_language_dict().get_string("#str_07001")
        };

        if self.waiting_on_key || (self.hover && !self.no_events && self.contains(self.gui.borrow().cursor_x(), self.gui.borrow().cursor_y())) {
            color = self.hover_color;
        } else {
            self.hover = false;
        }

        self.dc.borrow().draw_text(&str, self.text_scale, self.text_align, color, self.text_rect, false, -1);
    }

    fn activate(&mut self, activate: bool, act: &mut String) {
        self.activate(activate, act);
        self.bind_name.update(); // TODO  `update` is a method for updating the bind name
    }
}

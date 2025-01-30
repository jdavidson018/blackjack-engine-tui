pub struct App {
    pub active_menu_index: i8,
}

impl App {
    pub fn new() -> App {
        App {
            active_menu_index: 0,
        }
    }
}
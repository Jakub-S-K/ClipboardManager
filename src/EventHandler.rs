pub struct EventHandler {
    pub root: Option<sciter::Element>,
}

impl EventHandler {
    fn print(&mut self) -> sciter::Value {
        sciter::Value::from("Goes brrrrrrrr from Rust")
    }
}

impl sciter::EventHandler for EventHandler {
    fn attached(&mut self, root: sciter::HELEMENT) {
        self.root = Some(sciter::Element::from(root));
    }

    // fuctions dispatched to be called in TIScript
    dispatch_script_call! {
        fn print();
    }
}

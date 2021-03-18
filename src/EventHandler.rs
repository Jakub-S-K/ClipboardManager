pub struct EventHandler {
    pub root: Option<sciter::Element>,
    pub number: i32,
}

impl EventHandler {
    fn print(&mut self) -> sciter::Value {
        sciter::Value::from("Goes brrrrrrrr from Rust")
    }
    fn add(&mut self) -> sciter::Value
    {
        self.number += 1;
        sciter::Value::from(self.number)
    }
}

impl sciter::EventHandler for EventHandler {
    fn attached(&mut self, root: sciter::HELEMENT) {
        self.root = Some(sciter::Element::from(root));
    }
    fn on_script_call(&mut self, root: sciter::HELEMENT, name: &str, args: &[sciter::Value]) -> Option<sciter::Value> 
    {
        println!("{}", name);
        self.dispatch_script_call(root, name, args)
    }

    // fuctions dispatched to be called in TIScript
    dispatch_script_call! {
        fn print();
        fn add();
    }
}

use minijinja::{Environment, path_loader};

#[derive(Clone)]
pub struct AppState {
    pub templates: Environment<'static>,
}

impl AppState {
    pub fn new() -> Self {
        let mut templates = Environment::new();
        templates.set_loader(path_loader("templates"));

        AppState {
            templates,
        }
    }
}


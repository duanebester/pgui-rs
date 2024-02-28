mod app;
mod db;
mod state;

use gpui::*;

fn main() {
    let app = App::new();
    app::run_app(app);
}

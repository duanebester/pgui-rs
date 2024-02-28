use gpui::*;

use crate::state::StateModel;

pub static WIDTH: f64 = 800.0;
pub static HEIGHT: f64 = 600.0;

pub struct Editor;

impl Render for Editor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_grow()
            .border_1()
            .border_color(red())
            .justify_center()
            .items_center()
            .text_color(white())
            .child("Editor")
    }
}

impl Editor {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

pub struct Connections;

impl Render for Connections {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let width = Pixels::from(WIDTH / 3.);
        div()
            .w(width)
            .flex()
            .border_1()
            .border_color(green())
            .justify_center()
            .items_center()
            .text_color(white())
            .child("Connections")
    }
}

impl Connections {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

pub struct Workspace {
    connections: View<Connections>,
    editor: View<Editor>,
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .child(self.connections.clone())
            .child(self.editor.clone())
    }
}

impl Workspace {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        StateModel::new(cx);
        let connections = Connections::new(cx);
        let editor = Editor::new(cx);

        cx.new_view(|_cx| Self {
            connections,
            editor,
        })
    }
}

pub fn run_app(app: App) {
    app.run(|cx: &mut AppContext| {
        let window_options = setup_window(WIDTH, HEIGHT, cx);
        cx.open_window(window_options, |cx| Workspace::new(cx));
    });
}

// Thanks again to Matthias for the inspiration here
// https://github.com/MatthiasGrandl/Loungy
pub fn setup_window(app_width: f64, app_height: f64, cx: &mut AppContext) -> WindowOptions {
    let display_id_maybe = cx.displays().last().map(|d| d.id());
    let bounds_maybe = cx.displays().last().map(|d| d.bounds());
    let bounds = bounds_maybe.unwrap_or(Bounds {
        origin: Point::new(GlobalPixels::from(0.0), GlobalPixels::from(0.0)),
        size: Size {
            width: GlobalPixels::from(1920.0),
            height: GlobalPixels::from(1080.0),
        },
    });

    let mut options = WindowOptions::default();
    let center = bounds.center();

    options.focus = true;
    options.display_id = display_id_maybe;
    let width = GlobalPixels::from(app_width);
    let height = GlobalPixels::from(app_height);
    let x: GlobalPixels = center.x - width / 2.0;
    let y: GlobalPixels = center.y - height / 2.0;

    let bounds: Bounds<GlobalPixels> = Bounds::new(Point { x, y }, Size { width, height });
    options.bounds = WindowBounds::Fixed(bounds);
    options.titlebar = Some(TitlebarOptions::default());
    options.is_movable = true;
    options.kind = WindowKind::PopUp;
    options
}

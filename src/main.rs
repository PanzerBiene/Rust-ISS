use druid::widget::{Align, Flex, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Rust ISS");

#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
}

fn main() {
    //main Window 
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    //inital app state 
    let initial_state = AppState {
        name: "".into(),
    };

    //start application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch Rust ISS");
}

fn build_root_widget() -> impl Widget<AppState> {

    //textbox
    let textbox = TextBox::new()
        .with_placeholder("")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppState::name);

    //widget layout
    let layout = Flex::column()
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    //center 2 widgets in with_space 
    Align::centered(layout)
}

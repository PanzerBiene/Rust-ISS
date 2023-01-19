use druid::widget::{Align, Label, Flex, TextBox};
use druid::{AppLauncher, Data, Lens, Env, LocalizedString, Widget, WindowDesc, WidgetExt};
use reqwest;
use serde_json::{Value};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

fn main() {
    //main Window 
    let main_window = WindowDesc::new(build_root_widget)
        .title("Rust-ISS")
        .window_size((400.0, 400.0));

    //start application
    AppLauncher::with_window(main_window)
        .launch("".to_owned())
        .expect("Failed to launch Rust ISS");
}

fn build_root_widget() -> impl Widget<String> {

    //textbox
    let textbox = TextBox::new()
        .with_placeholder("ISS Location")
        .fix_width(TEXT_BOX_WIDTH);

    //widget layout
    let layout = Flex::column()
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    //center 2 widgets in with_space 
    Align::centered(layout)
}

async fn get_pos() -> Value {
    let url = "http://api.open-notify.org/iss-now.json";
    let client = reqwest::Client::new();
    let response = client.get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    let output: Value;
    match response {
        Ok(json) => {
           output = serde_json::from_str(&json).unwrap(); 
        }
        Err(e) => {
            output = Value::Null;
            println!("{:?}", e);
        }
    }

    return output;

}

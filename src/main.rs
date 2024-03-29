mod app;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("Starting up");
    yew::Renderer::<App>::new().render();
}

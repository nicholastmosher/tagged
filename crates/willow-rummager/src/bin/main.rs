use willow_rummager::gpui::Application;

fn main() {
    Application::new()
        .add_plugins(zed::init)
        .add_plugins(willow_rummager::init)
        .run();
}

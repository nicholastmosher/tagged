use zed::unstable::gpui::Application;

fn main() {
    Application::new().add_plugins(zed::init).run();
}

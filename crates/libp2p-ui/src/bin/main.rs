use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};
use zed::unstable::gpui::Application;

#[tokio::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::fmt::layer())
    //     .with(EnvFilter::from_default_env())
    //     .init();
    Application::new()
        .add_plugins(zed::init)
        .add_plugins(libp2p_ui::init)
        .run();
}

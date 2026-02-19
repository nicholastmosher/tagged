use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use zed::unstable::gpui::Application;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    Application::new()
        .add_plugins(zed::init)
        .add_plugins(tagged::init)
        .run();
}

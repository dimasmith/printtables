use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initializes tracing subscriber.
/// This function must be called only once in application lifecycle.
pub(super) fn initialize_tracing() {
    let subscriber = tracing_subscriber::registry()
        .with(LevelFilter::from_level(Level::TRACE))
        .with(fmt::Layer::default());

    subscriber.init();
}

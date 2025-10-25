use tracing_subscriber::EnvFilter;

pub fn init() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

use std::sync::{Once, ONCE_INIT};

static INIT: Once = ONCE_INIT;

pub fn setup() {
    INIT.call_once(|| {
        let _ = env_logger::builder()
            .filter(Some("nest"), log::LevelFilter::Debug)
            .is_test(true)
            .try_init();
    });
}

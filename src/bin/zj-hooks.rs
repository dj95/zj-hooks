use miette::Error;
use std::collections::BTreeMap;

use zellij_tile::prelude::*;

use zj_hooks::config::Config;

#[cfg(not(test))]
register_plugin!(State);

#[cfg(feature = "tracing")]
fn init_tracing() {
    use std::fs::File;
    use std::sync::Arc;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let file = File::create("/host/.zj-hooks.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    tracing_subscriber::registry().with(debug_log).init();

    tracing::info!("tracing initialized");
}

#[derive(Default)]
struct State {
    hidden: bool,
    config: Config,
    error: Option<Error>,
}

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        #[cfg(feature = "tracing")]
        init_tracing();

        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
            PermissionType::RunCommands,
        ]);
        subscribe(&[
            EventType::PermissionRequestResult,
            EventType::SessionUpdate,
            EventType::ModeUpdate,
            EventType::TabUpdate,
            EventType::PaneUpdate,
        ]);

        self.hidden = false;
        self.config = match Config::new(configuration) {
            Ok(c) => c,
            Err(e) => {
                self.error = Some(e);
                Config::default()
            }
        };
    }

    fn update(&mut self, event: Event) -> bool {
        if self.error.is_none() {
            self.config.process_hooks(event);
        }

        true
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        tracing::debug!("search query");

        if let Some(err) = &self.error {
            println!("{:?}", err);
        }
    }
}

use lazy_static::lazy_static;
use miette::{Diagnostic, Result, SourceSpan};
use regex::Regex;
use std::collections::BTreeMap;
use thiserror::Error;

use zellij_tile::prelude::*;

use crate::hook::Hook;

lazy_static! {
    static ref HOOK_REGEX: Regex = Regex::new("_[a-zA-Z0-9]+$").unwrap();
}

#[derive(Debug, Default)]
pub struct Config {
    hooks: Vec<Hook>,
}

impl Config {
    pub fn new(config: BTreeMap<String, String>) -> Result<Self> {
        let hooks: Vec<Hook> = parse_config(config)?;

        Ok(Self { hooks })
    }

    pub fn process_hooks(&self, event: Event) {
        tracing::debug!("registered hook count: {}", self.hooks.len());
        for hook in &self.hooks {
            hook.run_if_needed(&event);
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Missing argument!")]
#[diagnostic(
    code(zjhooks::config::Config::parse_config),
    help("Please check your config!")
)]
struct MissingArgumentError {
    #[source_code]
    argument_name: String,
    #[label("This one here is missing")]
    bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Invalid event type!")]
#[diagnostic(
    code(zjhooks::config::Config::parse_config),
    help("Please check your config and the documentation!")
)]
struct UnknownEventError {
    #[source_code]
    event_name: String,
    #[label("This one here is does not exist")]
    bad_bit: SourceSpan,
}

fn parse_config(conf: BTreeMap<String, String>) -> Result<Vec<Hook>> {
    let mut keys: Vec<String> = conf
        .keys()
        .filter(|k| k.starts_with("hook_"))
        .cloned()
        .collect();
    keys.sort();

    let mut mapped_config: BTreeMap<String, Hook> = BTreeMap::new();
    for key in keys {
        let hook_name = HOOK_REGEX.replace(&key, "").to_string();
        let hook_name = hook_name.replace("hook_", "").to_string();

        if mapped_config.contains_key(&hook_name) {
            continue;
        }

        let command = match conf.get(&format!("hook_{}_command", hook_name)) {
            Some(c) => c,
            None => {
                return Err(MissingArgumentError {
                    argument_name: format!("hook_{}_command", hook_name).to_string(),
                    bad_bit: (0, 1).into(),
                })?
            }
        };
        let event = match conf.get(&format!("hook_{}_event", hook_name)) {
            Some(c) => c,
            None => {
                return Err(MissingArgumentError {
                    argument_name: format!("hook_{}_event", hook_name).to_string(),
                    bad_bit: (0, 1).into(),
                })?
            }
        };

        let event = match map_event(event) {
            Some(e) => e,
            None => {
                return Err(UnknownEventError {
                    event_name: event.to_string(),
                    bad_bit: (0, 1).into(),
                })?
            }
        };

        tracing::debug!("hook_name = {}", hook_name);
        mapped_config.insert(hook_name, Hook::new(event, command));
    }

    Ok(mapped_config.values().cloned().collect::<Vec<Hook>>())
}

fn map_event(input: &str) -> Option<Event> {
    match input {
        "session" => Some(Event::SessionUpdate(vec![], vec![])),
        "mode" => Some(Event::ModeUpdate(ModeInfo::default())),
        "pane" => Some(Event::PaneUpdate(PaneManifest::default())),
        "tab" => Some(Event::TabUpdate(vec![])),
        _ => None,
    }
}

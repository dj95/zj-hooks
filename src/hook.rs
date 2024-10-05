use std::collections::BTreeMap;

use zellij_tile::prelude::*;

#[derive(Debug, Clone)]
pub struct Hook {
    event_type: Event,
    command: Vec<String>,
}

impl Hook {
    pub fn new(event_type: Event, command: &str) -> Self {
        let command = commandline_parser(command);

        Self {
            event_type,
            command,
        }
    }

    #[tracing::instrument(skip_all)]
    fn is_event(&self, event: &Event) -> bool {
        std::mem::discriminant(&self.event_type) == std::mem::discriminant(event)
    }

    #[tracing::instrument(skip_all)]
    pub fn run_if_needed(&self, event: &Event) {
        if !self.is_event(event) {
            tracing::debug!("event does not match");
            return;
        }

        tracing::debug!("event matches");
        let command = replace_based_on_event(event, self.command.clone());
        let context = BTreeMap::new();

        tracing::debug!("command: {:?}", command);

        run_command(
            &command.iter().map(|x| x.as_str()).collect::<Vec<&str>>(),
            context,
        );
    }
}

fn replace_based_on_event(event: &Event, command: Vec<String>) -> Vec<String> {
    let mut command = command;

    match event {
        Event::SessionUpdate(session_info, _) => {
            tracing::debug!("replacing for session");
            let current_session = session_info.iter().find(|s| s.is_current_session).unwrap();

            command = command
                .iter()
                .map(|c| {
                    let c = c
                        .replace("{{session_name}}", &current_session.name)
                        .to_owned();
                    tracing::debug!("replaced! {}", c);
                    c
                })
                .collect();
        }
        Event::TabUpdate(tabs) => {
            let active_tab = tabs.iter().find(|t| t.active).unwrap();

            command = command
                .iter()
                .map(|c| {
                    let c = c
                        .replace(
                            "{{active_tab_position}}",
                            &format!("{}", &active_tab.position),
                        )
                        .replace("{{active_tab_name}}", &active_tab.name)
                        .to_owned();
                    tracing::debug!("replaced! {}", c);
                    c
                })
                .collect();
        }
        Event::ModeUpdate(mode_info) => {
            command = command
                .iter()
                .map(|c| {
                    let c = c
                        .replace("{{mode}}", &format!("{:?}", mode_info.mode))
                        .to_owned();
                    tracing::debug!("replaced! {}", c);
                    c
                })
                .collect();
        }
        _ => {}
    }

    command
}

fn commandline_parser(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    let special_chars = ['"', '\''];

    let mut found_special_char = '\0';
    let mut buffer = "".to_owned();
    let mut is_escaped = false;
    let mut is_in_group = false;

    for character in input.chars() {
        if is_escaped {
            is_escaped = false;
            buffer = format!("{}\\{}", buffer.to_owned(), character);
            continue;
        }

        if character == '\\' {
            is_escaped = true;
            continue;
        }

        if found_special_char == character && is_in_group {
            is_in_group = false;
            found_special_char = '\0';
            output.push(buffer.clone());
            "".clone_into(&mut buffer);
            continue;
        }

        if special_chars.contains(&character) && !is_in_group {
            is_in_group = true;
            found_special_char = character;
            continue;
        }

        if character == ' ' && !is_in_group {
            output.push(buffer.clone());
            "".clone_into(&mut buffer);
            continue;
        }

        buffer = format!("{}{}", buffer, character);
    }

    if !buffer.is_empty() {
        output.push(buffer.clone());
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_commandline_parser() {
        let input = "pwd";
        let result = commandline_parser(input);
        let expected = Vec::from(["pwd"]);
        assert_eq!(result, expected);

        let input = "bash -c \"pwd | base64 -c \\\"bla\\\"\"";
        let result = commandline_parser(input);
        let expected = Vec::from(["bash", "-c", "pwd | base64 -c \\\"bla\\\""]);
        assert_eq!(result, expected);

        let input = "bash -c \"pwd | base64 -c 'bla' | xxd\"";
        let result = commandline_parser(input);
        let expected = Vec::from(["bash", "-c", "pwd | base64 -c 'bla' | xxd"]);
        assert_eq!(result, expected);
    }
}

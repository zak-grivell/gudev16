use std::rc::Rc;

use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    platform::collections::HashMap,
    prelude::*,
};

use crate::{
    chat::logic::ChatMessage,
    terminal::{display::TerminalWindow, tree::Tree},
    timer::TimeUp,
    window::Focused,
};

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (command_handler, handle_keyboard, on_reset))
            .init_resource::<Terminal>()
            .init_resource::<Tree>()
            .add_message::<Command>();
    }
}

#[derive(Clone)]
pub struct TerminalLine {
    pub text: String,
}

#[derive(Resource)]
pub struct Terminal {
    pub lines: Vec<TerminalLine>,
    pub input: String,
    pub cursor_pos: usize,
    pub scroll_offset: usize,
    pub cwd: String,
    pub user: String,
    pub hostname: String,
}

impl Default for Terminal {
    fn default() -> Self {
        let mut t = Self {
            lines: Vec::new(),
            input: String::new(),
            cursor_pos: 0,
            scroll_offset: 0,
            cwd: "/".into(),
            user: "user".into(),
            hostname: "ai-sas".into(),
        };

        // Boot messages
        t.push("AI SAS Startup Comany 999999999 - type 'help' for commands");
        t.push("----------------------------------------------------------");
        t
    }
}

impl Terminal {
    fn push(&mut self, text: impl Into<String>) {
        self.lines.push(TerminalLine { text: text.into() });
        if self.lines.len() > 500 {
            self.lines.remove(0);
        }
        self.scroll_to_bottom();
    }

    fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn prompt_string(&self) -> String {
        format!("{}@{}:{}$ ", self.user, self.hostname, self.cwd)
    }
}

pub enum Item {
    File(&'static str),
    Directory(&'static str),
}

#[derive(Message)]
struct Command(String);
fn handle_keyboard(
    mut term: ResMut<Terminal>,
    mut events: MessageReader<KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    focused: Query<(), (With<Focused>, With<TerminalWindow>)>,
    mut command_out: MessageWriter<Command>,
) {
    if focused.is_empty() {
        return;
    }

    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);

    let cursor_pos = term.cursor_pos;

    for ev in events.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }

        match &ev.logical_key {
            Key::Enter => {
                let cmd = term.input.trim().to_string();
                let prompt = term.prompt_string();
                term.push(format!("{}{}", prompt, cmd));

                if !cmd.is_empty() {
                    command_out.write(Command(cmd));
                }

                term.input.clear();
                term.cursor_pos = 0;
            }

            Key::Backspace => {
                if cursor_pos > 0
                    && let Some((byte_idx, _)) = term.input.char_indices().nth(cursor_pos - 1)
                {
                    term.input.remove(byte_idx);
                    term.cursor_pos -= 1;
                }
            }

            Key::ArrowUp => {
                term.scroll_offset += 1;
            }
            Key::ArrowDown if term.scroll_offset > 0 => {
                term.scroll_offset -= 1;
            }

            Key::ArrowLeft => {
                term.cursor_pos -= 1;
            }

            Key::ArrowRight => {
                term.cursor_pos += 1;
            }

            Key::PageUp => {
                term.scroll_offset = (term.scroll_offset + 10).min(term.lines.len());
            }
            Key::PageDown => {
                term.scroll_offset = term.scroll_offset.saturating_sub(10);
            }

            Key::Space => {
                term.input.insert(cursor_pos, ' ');
                term.cursor_pos += 1;
            }

            Key::Character(ch) => {
                if ctrl {
                    match ch.as_str() {
                        "c" => {
                            let prompt = term.prompt_string();
                            let input = term.input.clone();
                            term.push(format!("{}{}^C", prompt, input));
                            term.input.clear();
                            term.cursor_pos = 0;
                        }
                        "l" => {
                            term.lines.clear();
                        }
                        _ => {}
                    }
                } else {
                    for c in ch.chars() {
                        if !c.is_control() {
                            term.input.insert(cursor_pos, c);
                            term.cursor_pos += 1;
                        }
                    }
                }
            }

            Key::Escape => {
                term.input.clear();
                term.cursor_pos = 0;
            }

            _ => {}
        }
    }
}
fn command_handler(
    mut term: ResMut<Terminal>,
    mut command_recive: MessageReader<Command>,
    mut chat_send: MessageWriter<ChatMessage>,
    // mut dir: ResMut<CurrentDir>,
    tree: Res<Tree>,
) {
    for Command(raw) in command_recive.read() {
        let cmd = raw.split_whitespace().collect::<Vec<_>>();

        if cmd.is_empty() {
            continue;
        }

        match cmd[0] {
            "hello" => term.push("HELLO FROM TERM"),
            "ls" => {
                let path = cmd.get(1);

                let i = match path {
                    None => Some(term.cwd.as_str()),
                    Some(d) => match tree.0[term.cwd.as_str()].get(d) {
                        Some(Item::Directory(d)) => Some(*d),
                        Some(_) => {
                            term.push("path is not dir");
                            None
                        }
                        None => {
                            term.push("dir not found");
                            None
                        }
                    },
                };

                if let Some(dir) = i {
                    for (item, _) in &tree.0[dir] {
                        term.push(*item);
                    }
                }
            }
            "cd" => {
                let path = cmd.get(1);

                match path {
                    Some(p) => match tree.0[term.cwd.as_str()].get(p) {
                        Some(Item::Directory(p)) => term.cwd = p.to_string(),
                        Some(_) => term.push("given path is not dir"),
                        None => term.push("dir not found"),
                    },
                    None => term.push("no dir given"),
                };
            }
            "cat" => {
                let file = cmd.get(1);

                match file {
                    Some(p) => {
                        if let Some(d) = tree.0[term.cwd.as_str()].get(p) {
                            match d {
                                Item::File(content) => {
                                    for line in content.lines() {
                                        term.push(line);
                                    }
                                }
                                Item::Directory(_) => {
                                    term.push("given path is not file");
                                }
                            }
                        } else {
                            term.push("file not found");
                        }
                    }
                    None => term.push("no file given"),
                };
            }
            "kill_ai" => {
                chat_send.write(ChatMessage::new_now(
                    "You think it was that easy",
                    crate::chat::logic::Sender::Ai,
                ));
                term.push("permission denied");
            }
            "mkdir" | "touch" => term.push("Operation not permitted"),
            _ => term.push("UNKOWN COMMAND"),
        }
    }
}
fn on_reset(mut game_over: MessageReader<TimeUp>, mut state: ResMut<Terminal>) {
    for _ in game_over.read() {
        *state = Terminal::default();
    }
}

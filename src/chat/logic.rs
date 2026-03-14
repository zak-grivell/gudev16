use bevy::{platform::collections::HashMap, prelude::*};

use crate::timer::{GameTimer, TimeUp};

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChatMessage>()
            .init_resource::<ChatState>()
            .init_resource::<PossibleResponces>()
            .add_systems(Update, (message_recorder, on_reset));
    }
}

#[derive(Clone, serde::Deserialize, Debug)]
pub enum Sender {
    You,
    Ai,
    John,
    Olivia,
    Noah,
}

struct PlayerMessage(ChatMessage);

impl Sender {
    fn format(&self, msg: String) -> String {
        match self {
            Sender::You => msg,
            Sender::Ai => format!("CompanyAI: {}", msg),
            Sender::John => format!("John: {}", msg),
            Sender::Olivia => format!("Olivia: {}", msg),
            Sender::Noah => format!("Noah: {}", msg),
        }
    }
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Message, Clone)]
pub struct ChatMessage {
    pub sender: Sender,
    pub body: String,
    pub time: Option<f32>,
    pub relative: bool,
}

impl ChatMessage {
    pub fn new_now(text: impl Into<String>, sender: Sender) -> Self {
        Self {
            body: text.into(),
            sender,
            time: None,
            relative: false,
        }
    }

    pub fn new_relative(text: impl Into<String>, sender: Sender, time: f32) -> Self {
        Self {
            body: text.into(),
            sender,
            time: Some(time),
            relative: true,
        }
    }

    pub fn to_string(&self) -> String {
        self.sender.format(self.body.clone())
    }
}

#[derive(Resource, Default)]
pub struct ChatState(pub Vec<ChatMessage>);

#[derive(Resource)]
pub struct PossibleResponces(pub HashMap<String, Vec<ChatMessage>>);

impl Default for PossibleResponces {
    fn default() -> Self {
        let d = [(
            "This AI is evil shut it down".to_string(),
            vec![
                ChatMessage::new_relative(
                    "You think your capeable of shutting me down ha ha",
                    Sender::Ai,
                    0.5,
                ),
                ChatMessage::new_relative("Trying the kill_ai function now", Sender::Olivia, 3.0),
                ChatMessage::new_relative(
                    "I've been waiting years for this you think that can stop me :)",
                    Sender::Ai,
                    3.5,
                ),
            ],
        )];

        Self(HashMap::from_iter(d))
    }
}

fn message_recorder(
    mut messages: MessageReader<ChatMessage>,
    mut state: ResMut<ChatState>,
    responces: ResMut<PossibleResponces>,
    timer: Query<&GameTimer>,
    mut buffer: Local<Vec<ChatMessage>>,
) {
    let t = &timer.single().unwrap().0;

    for msg in messages.read() {
        state.0.push(msg.clone());
        if matches!(msg.sender, Sender::You) {
            for msg in responces.0.get(&msg.body).unwrap() {
                match msg.time {
                    Some(c) => buffer.push(ChatMessage {
                        sender: msg.sender.clone(),
                        body: msg.body.clone(),
                        time: Some(c + t.elapsed_secs()),
                        relative: false,
                    }),
                    None => {
                        state.0.push(msg.clone());
                    }
                }
            }
        }
    }

    for msg in buffer.iter() {
        if let Some(c) = msg.time
            && c <= t.elapsed_secs()
        {
            state.0.push(msg.clone());
        }
    }

    *buffer = buffer
        .into_iter()
        .filter(|msg| match msg.time {
            Some(c) => c > t.elapsed_secs(),
            None => false,
        })
        .cloned()
        .collect::<Vec<_>>()
}

fn on_reset(mut game_over: MessageReader<TimeUp>, mut state: ResMut<ChatState>) {
    for _ in game_over.read() {
        *state = ChatState::default();
    }
}

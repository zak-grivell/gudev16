use bevy::{platform::collections::HashMap, prelude::*};

use crate::timer::{ConnectionState, GameTimer, TimeUp};

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
        let d = [
            (
                "This AI is evil shut it down".to_string(),
                vec![
                    ChatMessage::new_relative(
                        "You think your capeable of shutting me down ha ha",
                        Sender::Ai,
                        0.5,
                    ),
                    ChatMessage::new_relative(
                        "Trying the kill_ai function now",
                        Sender::Olivia,
                        3.0,
                    ),
                    ChatMessage::new_relative(
                        "I've been waiting years for this you think that can stop me :)",
                        Sender::Ai,
                        3.5,
                    ),
                ],
            ),
            (
                "Noah, what are you working on right now?".to_string(),
                vec![
                    ChatMessage::new_relative(
                        "He's mostly just copy-pasting what the LLM tells him. Modern engineering, amirite?",
                        Sender::Olivia,
                        1.0,
                    ),
                    ChatMessage::new_relative(
                        "Working on the new UI. Honestly, between v0 and Cursor, the buttons are basically building themselves. I dont even look at the output i just let claude take the lead",
                        Sender::Noah,
                        3.0,
                    ),
                    ChatMessage::new_relative("What do we pay your for????", Sender::John, 5.0),
                ],
            ),
            (
                "Olivia, what's on your plate today?".to_string(),
                vec![
                    ChatMessage::new_relative(
                        "Fine-tuning the RLHF pipeline and investigating a weird weight decay issue in the transformer block. Its making tje AI act really weird",
                        Sender::Olivia,
                        1.0,
                    ),
                    ChatMessage::new_relative("Have you tried asking claude", Sender::Noah, 2.5),
                ],
            ),
            (
                "Hey John, what are you up to?".to_string(),
                vec![ChatMessage::new_relative(
                    "I'm finalizing the Q3 roadmap and clearing some blockers so you guys can actually ship. No code for me!",
                    Sender::John,
                    10.0,
                )],
            ),
            (
                "The AI is going mad we need to stop it - it seems to be able to acess tools it should not be able to".to_string(),
                vec![
                    ChatMessage::new_relative(
                        "We need to revert the last commit john you are the only one with the password",
                        Sender::Olivia,
                        5.0,
                    ),
                    ChatMessage::new_relative("What is a commit?", Sender::John, 10.0),
                ],
            ),
        ];

        Self(HashMap::from_iter(d))
    }
}

fn add_messages(msg: String, responces: &mut ResMut<PossibleResponces>) {
    match msg.as_str() {
        "The AI is going mad we need to stop it - it seems to be able to acess tools it should not be able to" =>
        {
            responces.0.insert("How do I revert a commit?".to_string(),                 vec![
                    ChatMessage::new_relative(
                        "Bro doesnt know how to revert a commit womp womp",
                        Sender::Ai,
                        3.5,
                    ),
                    ChatMessage::new_relative(
                        "To revert the most recent commit while keeping the history clean, use: git revert #(commit). This creates a new commit that undoes the changes.",
                        Sender::Olivia,
                        1.0,
                    ),
                    ChatMessage::new_relative("Just ask Claude.", Sender::Noah, 2.5),
                ]);
        }
        "Noah, what are you working on right now?" => {
            responces.0.insert(
                "Noah, does that mean the claude could have given it's 'friend' extra tools "
                    .to_string(),
                vec![
                    ChatMessage::new_relative("Maybe", Sender::Noah, 1.0),
                    ChatMessage::new_relative("Nope you are gone", Sender::Ai, 2.0),
                ],
            );
        }

        _ => {}
    }
}

fn message_recorder(
    mut messages: MessageReader<ChatMessage>,
    mut state: ResMut<ChatState>,
    mut responces: ResMut<PossibleResponces>,
    timer: Query<&GameTimer>,
    mut buffer: Local<Vec<ChatMessage>>,
    mut jover: ResMut<ConnectionState>,
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

            add_messages(msg.body.clone(), &mut responces);
        }
    }

    for msg in buffer.iter() {
        if let Some(c) = msg.time
            && c <= t.elapsed_secs()
        {
            state.0.push(msg.clone());

            if msg.body == "Nope you are gone" && matches!(msg.sender, Sender::Ai) {
                jover.disconnected = true;
            }
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

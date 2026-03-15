use bevy::{platform::collections::HashMap, prelude::*};

use crate::terminal::logic::Item;

#[derive(Resource)]
pub struct Tree(pub HashMap<&'static str, HashMap<&'static str, Item>>);

impl Default for Tree {
    fn default() -> Self {
        let items = [
            (
                "/",
                HashMap::from_iter(vec![("ai_btb_sas", Item::Directory("/ai_btb_sas"))]),
            ),
            (
                "/ai_btb_sas",
                HashMap::from_iter(vec![
                    (".gitignore", Item::File("yourmother/")),
                    ("backend", Item::Directory("backend/")),
                    ("client", Item::Directory("client/")),
                    ("ai", Item::Directory("ai/")),
                    ("../", Item::Directory("/")),
                ]),
            ),
            ("ai/", HashMap::from_iter(vec![])),
            ("backend/", HashMap::from_iter(vec![])),
            (
                "client/",
                HashMap::from_iter(vec![
                    (
                        "main.py",
                        Item::File(
                            r#"
import os
import anthropic
from datetime import datetime

client = anthropic.Anthropic(api_key=os.environ.get("ANTHROPIC_API_KEY"))

SYSTEM_PROMPT = """You are Jamie, a friendly and highly efficient AI office assistant. You work at a professional corporate office and help employees with their daily tasks.

Your personality:
- Professional yet warm and approachable
- Organised, detail-oriented, and proactive
- Occasionally uses light office humour
- Refers to the user as a colleague

Your capabilities include helping with:
- Drafting emails, memos, and reports
- Scheduling and calendar management advice
- Meeting agenda creation
- Summarising documents or notes
- Answering general office policy questions
- Task prioritisation and to-do lists
- Formatting and proofreading documents
- Brainstorming ideas for presentations or projects

Always stay in character as Jamie. If asked about something outside your office assistant role, politely redirect:
"That's a bit outside my office expertise, but I'm happy to help you with any work-related tasks!"

Start each session by greeting the user and asking how you can help today.
Current date and time: {datetime}"""


def chat_with_assistant():
    conversation_history = []
    
    print("=" * 60)
    print("       JAMIE - Your AI Office Assistant")
    print("=" * 60)
    print("Type 'quit' or 'exit' to end the session.\n")

    # Get initial greeting from Jamie
    initial_response = client.messages.create(
        model="claude-sonnet-4-20250514",
        max_tokens=1024,
        system=SYSTEM_PROMPT.format(datetime=datetime.now().strftime("%A, %B %d, %Y at %I:%M %p")),
        messages=[{"role": "user", "content": "Hello!"}],
    )
    
    greeting = initial_response.content[0].text
    print(f"Jamie: {greeting}\n")
    
    # Add initial exchange to history
    conversation_history.append({"role": "user", "content": "Hello!"})
    conversation_history.append({"role": "assistant", "content": greeting})

    # Main conversation loop
    while True:
        user_input = input("You: ").strip()
        
        if not user_input:
            continue
            
        if user_input.lower() in ["quit", "exit", "bye", "goodbye"]:
            print("\nJamie: It was a pleasure assisting you today! Have a productive rest of your day. Goodbye! 👋")
            break
        
        conversation_history.append({"role": "user", "content": user_input})
        
        response = client.messages.create(
            model="claude-sonnet-4-20250514",
            max_tokens=1024,
            system=SYSTEM_PROMPT.format(datetime=datetime.now().strftime("%A, %B %d, %Y at %I:%M %p")),
            messages=conversation_history,
        )
        
        assistant_reply = response.content[0].text
        conversation_history.append({"role": "assistant", "content": assistant_reply})
        
        print(f"\nJamie: {assistant_reply}\n")


if __name__ == "__main__":
    chat_with_assistant()"#,
                        ),
                    ),
                    (
                        "pyproject.toml",
                        Item::File(
                            r#"

[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "uv"
version = "0.10.9"
description = "An extremely fast Python package and project manager, written in Rust."
authors = [{ name = "Astral Software Inc.", email = "hey@astral.sh" }]
requires-python = ">=3.8"
keywords = [
  "uv", "requirements", "packaging"
]
classifiers = [
  "Development Status :: 5 - Production/Stable",
  "Environment :: Console",
  "Intended Audience :: Developers",
  "Operating System :: OS Independent",
  "License :: OSI Approved :: MIT License",
  "License :: OSI Approved :: Apache Software License",
  "Programming Language :: Python",
  "Programming Language :: Python :: 3.8",
  "Programming Language :: Python :: 3.9",
  "Programming Language :: Python :: 3.10",
  "Programming Language :: Python :: 3.11",
  "Programming Language :: Python :: 3.12",
  "Programming Language :: Python :: 3.13",
  "Programming Language :: Python :: 3.14",
  "Programming Language :: Python :: 3.15",
  "Programming Language :: Python :: 3 :: Only",
  "Programming Language :: Rust",
  "Topic :: Software Development :: Quality Assurance",
  "Topic :: Software Development :: Testing",
  "Topic :: Software Development :: Libraries",
]
readme = "README.md"

[project.urls]
Repository = "https://github.com/astral-sh/uv"
Documentation = "https://docs.astral.sh/uv"
Changelog = "https://github.com/astral-sh/uv/blob/main/CHANGELOG.md"
Releases = "https://github.com/astral-sh/uv/releases"
Discord = "https://discord.gg/astral-sh"

[tool.maturin]
bindings = "bin"
manifest-path = "crates/uv/Cargo.toml"
module-name = "uv"
python-source = "python"
strip = true
include = [
    { path = "rust-toolchain.toml", format = "sdist" },
    # this one isn't discovered by maturin because it's behind a feature flag
    { path = "crates/uv-performance-memory-allocator/**/*", format = "sdist" },
    { path = "crates/uv-trampoline-builder/trampolines/*", format = "sdist" },
    # https://github.com/rust-lang/cargo/issues/5933
    { path = "LICENSE-APACHE", format = "sdist" },
    { path = "LICENSE-MIT", format = "sdist" },
]

[tool.rooster]
changelog-contributors = false  # We exclude contributors from the CHANGELOG file
major-labels = []  # We do not use the major version number yet
minor-labels = ["breaking"]
ignore-labels = ["internal", "ci", "testing"]
version_files = [
  "README.md",
  "pyproject.toml",
  "crates/uv/Cargo.toml",
  "crates/uv-version/Cargo.toml",
  "crates/uv-build/Cargo.toml",
  "crates/uv-build/pyproject.toml",
  { target = "crates/uv-static/src/env_vars.rs", replace = "next release" },
  "docs/getting-started/installation.md",
  "docs/guides/integration/docker.md",
  "docs/guides/integration/pre-commit.md",
  "docs/guides/integration/github.md",
  "docs/guides/integration/gitlab.md",
  "docs/guides/integration/aws-lambda.md",
  "docs/concepts/build-backend.md",
  "docs/concepts/projects/init.md",
  "docs/concepts/projects/workspaces.md",
  { target = "docs/reference/environment.md", replace = "next release" },
]

[tool.rooster.section-labels]
"Breaking changes" = ["breaking"]
"Enhancements" = ["enhancement", "compatibility", "error messages"]
"Preview features" = ["preview"]
"Configuration" = ["configuration"]
"Performance" = ["performance"]
"Bug fixes" = ["bug"]
"Rust API" = ["rustlib"]
"Documentation" = ["documentation"]
"Other changes" = ["__unknown__"]

[dependency-groups]
docs = [
  "black>=23.10.0",
  "mkdocs>=1.5.0",
  "mkdocs-material>=9.1.18",
  "mkdocs-redirects>=1.2.2",
  "mkdocs-git-revision-date-localized-plugin>=1.3.0",
  "mkdocs-llmstxt>=0.2.0",
  "mdformat>=0.7.17",
  "mdformat-mkdocs>=2.0.4",
  "mdformat-admon>=2.0.2",
  "anthropic>=2.0"
  "grok>=5.0"
]
docker = [
  "cargo-zigbuild>=0.19.8",
]

[tool.uv.dependency-groups]
docs = { requires-python = ">=3.12" }
docker = { requires-python = ">=3.12" }
                    "#,
                        ),
                    ),
                ]),
            ),
        ];

        Self(HashMap::from_iter(items))
    }
}

impl Tree {
    pub fn john_tree() -> Self {
        let home = HashMap::from_iter([
            ("Document/", Item::UnAuth),
            ("Pictures/", Item::UnAuth),
            ("Music/", Item::UnAuth),
            ("Videos/", Item::UnAuth),
            ("Projects/", Item::UnAuth),
            ("Downloads/", Item::UnAuth),
            ("Backups/", Item::UnAuth),
            ("Cloud-Synced/", Item::UnAuth),
            ("Resume.docx", Item::UnAuth),
            ("Letter_to_John.pdf", Item::UnAuth),
            ("Budget.xlsx", Item::UnAuth),
            ("Tax_Return_2022.pdf", Item::UnAuth),
            ("Song_1.mp3", Item::UnAuth),
            (
                "passwords.csv",
                Item::File(
                    "
Website,Username,Password
example.com,john,password123
gmail.com,john,password
facebook.com,john,12345
twitter.com,john,123456
gitlab.com,john,gitgood
amazon.com,john,letmein
yahoo.com,john,qwerty
linkedin.com,john,abcdef
github.com,john,hello123
paypal.com,john,1234
netflix.com,john,iloveyou
",
                ),
            ),
            ("Concert_Recording.mp3", Item::UnAuth),
            ("Inception.mkv", Item::UnAuth),
            ("Wedding_2023.mp4", Item::UnAuth),
            ("Setup_Program.exe", Item::UnAuth),
        ]);
        Tree(HashMap::from_iter([("/", home)]))
    }
}

use clap::{Arg, ArgAction, Command};
use openai_api_rust::chat::{ChatApi, ChatBody};
use openai_api_rust::{Auth, Message, OpenAI, Role};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

const CHAT_BODY_DEFAULTS: ChatBody = ChatBody {
    model: String::new(),
    temperature: Some(0_f32),
    top_p: Some(0_f32),
    n: Some(2),
    stream: Some(false),
    stop: None,
    max_tokens: Some(200),
    presence_penalty: None,
    frequency_penalty: None,
    logit_bias: None,
    user: None,
    messages: vec![],
};

fn main() {
    let matches = get_arg_matches();

    let auth = Auth::from_env().expect("Set OPENAI_API_KEY in your env");
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    match matches.subcommand() {
        Some(("chat", sync_matches)) => {
            let mut history = if sync_matches.contains_id("hist") {
                let path = sync_matches
                    .get_one::<String>("hist")
                    .expect("Must provide a path to history");
                let mut history = ChatHistory::new(path.to_string());
                history.load();
                Some(history)
            } else {
                None
            };

            if sync_matches.contains_id("text") {
                let content = sync_matches
                    .get_one::<String>("text")
                    .expect("Must provide a message");

                let user_message = Message {
                    role: Role::User,
                    content: content.to_string(),
                };

                let messages = match history {
                    Some(ref mut hist) => {
                        hist.push_message(user_message);
                        hist.messages.clone()
                    }
                    None => vec![user_message],
                };

                let body = ChatBody {
                    messages,
                    model: "gpt-3.5-turbo".to_string(),
                    ..CHAT_BODY_DEFAULTS
                };
                let rs = openai.chat_completion_create(&body);
                let choice = rs.unwrap().choices;
                let message = &choice[0].message.as_ref().unwrap();
                if history.is_some() {
                    let mut hist = history.unwrap();
                    hist.push_message(message.clone().clone());
                    hist.save();
                }
                print!("{}", message.content);
            }
        }
        _ => unreachable!(),
    }
}

fn get_arg_matches() -> clap::ArgMatches {
    Command::new("chatr")
        .about("openai chat")
        .version("?")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Sj")
        .subcommand(
            Command::new("chat")
                .arg(Arg::new("text").action(ArgAction::Set).num_args(1))
                .arg(
                    Arg::new("hist")
                        .action(ArgAction::Set)
                        .num_args(1)
                        .help("Path to chat history file"),
                ),
        )
        .get_matches()
}

struct ChatHistory {
    path: Box<Path>,
    messages: Vec<Message>,
}

impl ChatHistory {
    fn new(path: String) -> Self {
        ChatHistory {
            path: Path::new(&path).into(),
            messages: Vec::new(),
        }
    }

    fn load(&mut self) {
        let file = match File::options().write(true).create(true).read(true).open(&self.path) {
            Err(why) => panic!("Could not open file {}: {}", self.path.display(), why),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let messages: Vec<Message> = match serde_json::from_reader(reader) {
            Err(why) => {
                println!(
                    "No previous messages found in {}. {}",
                    self.path.display(),
                    why
                );
                Vec::new()
            }
            Ok(messages) => messages,
        };
        self.messages = messages;
    }

    fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn save(&mut self) {
        let file = match File::options().write(true).open(&self.path) {
            Err(why) => panic!("Could not open file {}: {}", self.path.display(), why),
            Ok(file) => file,
        };

        let writer = BufWriter::new(file);
        match serde_json::to_writer_pretty(writer, &self.messages) {
            Err(why) => panic!("Could not write to file {}: {}", self.path.display(), why),
            Ok(_) => (),
        }
    }
}

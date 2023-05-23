mod args;
mod history;

use std::io::Read;

use crate::history::ChatHistory;
use args::get_arg_matches;
use openai_api_rust::chat::{ChatApi, ChatBody};
use openai_api_rust::models::ModelsApi;
use openai_api_rust::{Auth, Message, OpenAI, Role};

const CHAT_BODY_DEFAULTS: ChatBody = ChatBody {
    model: String::new(),
    temperature: Some(0.35_f32),
    top_p: Some(0_f32),
    n: Some(1),
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

    let mut auth = Auth::from_env().expect("Set OPENAI_API_KEY in your env");
    if let Ok(org) = std::env::var("OPENAI_API_ORG") {
        auth.organization = Some(org);
    }
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    match matches.subcommand() {
        Some(("list-models", _)) => {
            let rs = openai.models_list();
            for model in rs.unwrap() {
                println!(
                    "id: {} object: {} owned_by {}",
                    model.id,
                    model.object.unwrap_or("?".to_string()),
                    model.owned_by.unwrap_or("?".to_string())
                );
            }
        }
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

            let temp = sync_matches.get_one::<String>("temperature").map(|s| s.parse::<f32>().unwrap_or(0.34_f32)).unwrap();

            let model = sync_matches
                .get_one::<String>("model")
                .expect("Expected gpt model")
                .to_string();

            if let Some(message) = match sync_matches.get_one::<String>("prompt-file") {
                Some(path) => {
                    let file = std::fs::File::open(path).expect("Unable to open promp-file");
                    let mut reader = std::io::BufReader::new(&file);
                    let mut s = String::new();
                    reader
                        .read_to_string(&mut s)
                        .expect("Uanble to read content of prompt-file");
                    Some(s)
                }
                None => sync_matches.get_one::<String>("message").cloned(),
            } {
                let user_message = Message {
                    role: Role::User,
                    content: message,
                };

                let messages = match history {
                    Some(ref mut hist) => {
                        hist.push_message(user_message);
                        hist.messages().clone()
                    }
                    None => vec![user_message],
                };

                let body = ChatBody {
                    messages,
                    // gpt-4, gpt-4-0314, gpt-4-32k, gpt-4-32k-0314, gpt-3.5-turbo, gpt-3.5-turbo-0301
                    model,
                    temperature: Some(temp),
                    ..CHAT_BODY_DEFAULTS
                };

                // println!("{:?}", body);
                let rs = openai.chat_completion_create(&body);
                let choice = rs.unwrap().choices;
                let message = &choice[0].message.as_ref().unwrap();
                if let Some(mut history) = history {
                    history.push_message((*message).clone());
                    history.save();
                }
                print!("{}", message.content);
            } else {
                panic!("No message or prompt-file could be parsed");
            };
            // if let Some(path) = sync_matches.get_one::<String>("prompt-file") {
            //     let file = std::fs::File::open(path).expect("Unable to open prompt file");
            //     let mut reader = std::io::BufReader::new(&file);
            //     let mut s = String::new();
            //
            //     reader.read_to_string(&mut s).expect("Unable to read content of prompt file");
            // }
        }
        _ => unreachable!(),
    }
}

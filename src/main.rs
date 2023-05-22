use clap::{Arg, ArgAction, Command};
use openai_api_rust::chat::{ChatBody, ChatApi};
use openai_api_rust::{Auth, Message, OpenAI, Role};

fn main() {
    let matches = Command::new("chatr")
        .about("openai chat")
        .version("?")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Sj")
        .subcommand(Command::new("chat").arg(Arg::new("text").action(ArgAction::Set).num_args(1)))
        .get_matches();

    let auth = Auth::from_env().expect("Set OPENAI_API_KEY in your env");
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    match matches.subcommand() {
        Some(("chat", sync_matches)) => {
            if sync_matches.contains_id("text") {
                let content = sync_matches
                    .get_one::<String>("text")
                    .expect("Must provide a message");

                let body = ChatBody {
                    model: "gpt-3.5-turbo".to_string(),
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
                    messages: vec![Message {
                        role: Role::User,
                        content: content.to_string(),
                    }],
                };
                let rs = openai.chat_completion_create(&body);
                let choice = rs.unwrap().choices;
                let message = &choice[0].message.as_ref().unwrap();
                print!("{}", message.content);
            }
        },
        _ => unreachable!(),
    }
}

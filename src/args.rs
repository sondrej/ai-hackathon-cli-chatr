use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn get_arg_matches() -> ArgMatches {
    Command::new("chatr")
        .about("openai chat")
        .version("?")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Sj")
        .subcommand(
            Command::new("chat")
                .arg(
                    Arg::new("hist")
                        .long("history")
                        .action(ArgAction::Set)
                        .num_args(0..=1)
                        .help("Path to chat history file"),
                )
                .arg(
                    Arg::new("message")
                        .long("message")
                        .short('m')
                        .action(ArgAction::Set)
                        .help("Proompt")
                        .num_args(0..=1),
                )
                .arg(
                    Arg::new("temperature").short('t')
                    .action(ArgAction::Set)
                    .help("Temperature is a number between 0 and 1 that adjusts the level of deterministic responses, 1 is the most deterministic")
                    .num_args(1)
                )
                .arg(
                    Arg::new("prompt-file")
                    .short('f')
                    .long("prompt-file")
                    .action(ArgAction::Set)
                    .help("Path to prompt file")
                    .num_args(0..=1)
                    .conflicts_with("message")
                )
                // gpt-4, gpt-4-0314, gpt-4-32k, gpt-4-32k-0314, gpt-3.5-turbo, gpt-3.5-turbo-0301
                .arg(
                    Arg::new("model")
                        .long("model")
                        .action(ArgAction::Set)
                        .num_args(1)
                        .default_value("gpt-3.5-turbo")
                        .long_help("possible values: gpt-4, gpt-4-0314, gpt-4-32k, gpt-4-32k-0314, gpt-3.5-turbo, gpt-3.5-turbo-0301"),
                ),
            // .get_matches_from(vec!["gpt-4", "gpt-4-0314", "gpt-4-32k", "gpt-4-32k-0314", "gpt-3.5-turbo", "gpt-3.5-turbo-0301"]),
        )
        .subcommand(Command::new("list-models"))
        .get_matches()
}

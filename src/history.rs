use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use openai_api_rust::Message;

pub struct ChatHistory {
    path: Box<Path>,
    messages: Vec<Message>,
}

impl ChatHistory {
    pub fn new(path: String) -> Self {
        ChatHistory {
            path: Path::new(&path).into(),
            messages: Vec::new(),
        }
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn load(&mut self) {
        let file = match File::options()
            .write(true)
            .create(true)
            .read(true)
            .open(&self.path)
        {
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

    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn save(&mut self) {
        let file = match File::options().write(true).open(&self.path) {
            Err(why) => panic!("Could not open file {}: {}", self.path.display(), why),
            Ok(file) => file,
        };

        let writer = BufWriter::new(file);
        if let Err(why) = serde_json::to_writer_pretty(writer, &self.messages) {
            panic!("Could not write to file {}: {}", self.path.display(), why);
        }
    }
}

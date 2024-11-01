mod conversations_collection;
mod questions_editor;
mod game;

use std::vec::Vec;
use game::*;
use std::env;
use questions_editor::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    
    if args.len() > 1 && args[1] == "editor".to_string() {
        QuestionsEditor::start_editor();
    } else {
        let conversations : CoversationInteractor = serde_json::from_str(&QuestionsEditor::read_from_file().unwrap()).unwrap();
        main_questions_cycle(&conversations);
    }
}


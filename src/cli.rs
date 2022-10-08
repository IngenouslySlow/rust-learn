use libretranslate::{translate, Language};
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    // println!("Args: {:?}", args); // Add something after cargo run command to print those
    // println!("Command: {}", command);
    let source = Language::English;
    let target = Language::French;
    let input = &command;

    let data = translate(Some(source), target, &input);

    println!("Your translated text: {}", data.output());
}

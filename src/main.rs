use std::collections::HashMap;
use std::f64::consts::{E, PI};
use std::process::exit;
use std::str::SplitWhitespace;

use ansi_term::Color;
use configuration::loader::Config;
use linefeed::{Interface, ReadResult};

use crate::configuration::loader::{load, load_config, write_default_config, Loaded};
use crate::interpreting::interpreter::interpret;
use crate::lexing::lexer::lex;
use crate::parsing::ast::{Ast, Parameters};
use crate::parsing::parser::CalcParser;

mod configuration;
mod interpreting;
mod lexing;
mod parsing;
mod utils;

fn show_config(config: Config) -> (String, Option<Config>) {
    let loaded = load_config(config.clone());

    let color_message = loaded.greeting_color.paint(&config.greeting.greeting_color);

    let show_message = loaded
        .greeting_color
        .paint(config.greeting.greeting_message);
    let prompt = loaded.prompt_style.paint(loaded.prompt);
    let prompt_color_message = loaded.prompt_style.paint(config.prompt.prompt_color);
    let general_message_color = loaded.general_color.paint(config.general_color);
    let general_message = loaded.general_color.paint("This is the general colour");
    println!(" The greeting colour is set to {} which prints \n {} \n The prompt is {} in {} \n Main color is {} which looks like \n {} \n If you've modified your config and it doesn't look good, the author (Charlotte Thomas) declines any responsabilities.\n",color_message,
    show_message,prompt,prompt_color_message,general_message_color,general_message);
    ("".to_string(), None)
}

fn reset_config(color: Color) -> (String, Option<Config>) {
    let _ = write_default_config();
    match load() {
        Ok(cfg) => (
            "Your config has been reseted to default settings\n".to_string(),
            Some(cfg),
        ),
        Err(_) => (
            "An error occured while parsing the config file\n".to_string(),
            None,
        ),
    }
}

fn set_config(config: Config, args: &SplitWhitespace, color: Color) -> (String, Option<Config>) {
    ("".to_string(), None)
}

fn reload_config() -> (String, Option<Config>) {
    match load() {
        Ok(cfg) => (
            "Your configuration has been reloaded\n".to_string(),
            Some(cfg),
        ),
        Err(_) => (
            "An error occured while parsing the config file\n".to_string(),
            None,
        ),
    }
}

fn handle_config(line: &str, config: Config, color: Color) -> (String, Option<Config>) {
    match line.strip_prefix("config") {
        None => show_config(config.clone()),
        Some(t) => {
            let mut w = t.split_whitespace();
            match w.nth(0) {
                None => show_config(config.clone()),
                Some("set") => set_config(config, &w.clone(), color),
                Some("reload") => reload_config(),
                Some("reset") => reset_config(color),
                _ => show_config(config.clone()),
            }
        }
    }
}

fn main() {
    let config = match load() {
        Ok(config) => config,
        Err(_) => {
            let _ = write_default_config();
            match load() {
                Ok(cfg) => cfg,
                Err(_) => {
                    println!("fatal error please remove your config file");
                    exit(1);
                }
            }
        }
    };

    let mut loaded: Loaded = load_config(config.clone());

    let message = &loaded.greeting_message;
    println!("{}", message.to_string());

    let interface = Interface::new("calc").unwrap();
    let style = &loaded.clone().prompt_style;
    let text = &loaded.clone().prompt;
    let mut verbose = false;
    let version: String = "v2.7.0".to_string();
    interface
        .set_prompt(&format!(
            "\x01{prefix}\x02{text}\x01{suffix}\x02",
            prefix = style.prefix(),
            text = text,
            suffix = style.suffix()
        ))
        .unwrap();
    let mut ram: HashMap<String, Parameters> = HashMap::new();
    let mut functions: HashMap<String, (Vec<Ast>, Ast)> = HashMap::new();
    ram.insert("pi".to_string(), Parameters::Float(PI));
    ram.insert("e".to_string(), Parameters::Float(E));
    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        match line.as_str().trim() {
            "info" => {
                let message = loaded.general_color.paint(format!(" Calc {version} \n Author: Charlotte Thomas \n Written in Rust \n Repo: https://github.com/coco33920/calc\n"));
                println!("{}", message)
            }
            "exit" => break,
            "help" => {
                let message = loaded.general_color.paint(format!(
                    " Calc {version} Help \n > info : show infos \n > exit : exit the program \n > help : print this help \n > verbose : toggle the verbose \n > version : prints the version \n"
                ));
                println!("{}", message)
            }
            "version" => {
                let message = loaded.general_color.paint(format!(" Calc {version}\n"));
                println!("{}", message)
            }
            "verbose" => {
                verbose = !verbose;
                let message = loaded.general_color.paint("You toggled the verbose : ");
                let message2 = Color::Red.paint(if verbose { "on" } else { "off" });
                println!("{}{}", message, message2)
            }
            str => {
                if str.starts_with("config") {
                    let (s, q) = handle_config(&line, config.clone(), loaded.general_color);
                    match q {
                        Some(q) => {
                            loaded = load_config(q);
                            print!("{}", loaded.general_color.paint(s));
                        }
                        _ => {
                            let m = loaded.general_color.paint(s);
                            print!("{m}");
                        }
                    }
                } else {
                    let a = lex(str.to_string());
                    let parser: &mut CalcParser = &mut parsing::parser::init_calc_parser(&a);
                    let p = parser.parse();
                    if verbose {
                        println!("Lexing of line: {str}");
                        println!("{:?}", &a);
                        println!("Parsing of line: {str}");
                        println!("{:#?}", p);
                        println!()
                    }
                    let result = interpret(&p, &mut ram, &mut functions);
                    if result != Parameters::Null {
                        println!(
                            "{}",
                            result.pretty_print(Some(&mut ram), Some(&mut functions))
                        )
                    }
                }
            }
        }
        interface.add_history_unique(line);
    }
    exit(0);
}

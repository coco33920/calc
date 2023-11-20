use std::collections::HashMap;
use std::f64::consts::{E, PI};
use std::process::exit;
use std::str::SplitWhitespace;
use std::sync::Arc;

use ansi_term::Color;
use configuration::loader::Config;
use linefeed::{Completer, Completion, Interface, ReadResult, Terminal};

use crate::configuration::loader::{
    load, load_config, write_config, write_default_config, Greeting, Loaded, Prompt,
};
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

fn reset_config() -> (String, Option<Config>) {
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

fn set_config(config: Config, args: &mut SplitWhitespace) -> (String, Option<Config>) {
    fn handle_second_argument(
        config: Config,
        s: Option<&str>,
        args: &mut SplitWhitespace,
    ) -> (String, Option<Config>) {
        match s {
            None => (
                "You need more argument for this command\n".to_string(),
                None,
            ),
            Some("general_color") => {
                let mut st = "".to_string();
                args.into_iter().for_each(|x| st = st.clone() + x + " ");

                match st.as_str() {
                    s if s.trim() == "" => (
                        "You need more argument for this command\n".to_string(),
                        None,
                    ),
                    s => {
                        let cfg = Config {
                            general_color: (s.to_string()),
                            greeting: (config.greeting),
                            prompt: (config.prompt),
                        };
                        match write_config(&cfg) {
                            Ok(_) => (format!("Greeting color has been set to {}, reload for this to take effect\n",&s).to_string(),None),
                            _ => ("An error occured while writing the config\n".to_string(),None)
                        }
                    }
                }
            }
            Some("prompt") => {
                let mut st = "".to_string();
                args.into_iter().for_each(|x| st = st.clone() + x + " ");
                match st.as_str() {
                    s if s.trim() == "" => (
                        "You need more argument for this command\n".to_string(),
                        None,
                    ),
                    s => {
                        let cfg = Config {
                            general_color: config.general_color,
                            greeting: (config.greeting),
                            prompt: Prompt {
                                prompt: s.to_string(),
                                prompt_color: config.prompt.prompt_color,
                            },
                        };

                        match write_config(&cfg) {
                            Ok(_) => (
                                format!(
                                "Prompt has been updated to {}, reload for this to take effect\n",
                                &s
                            )
                                .to_string(),
                                None,
                            ),
                            _ => (
                                "An error occured while writing the config\n".to_string(),
                                None,
                            ),
                        }
                    }
                }
            }
            Some("prompt_color") => {
                let mut st = "".to_string();
                args.into_iter().for_each(|x| st = st.clone() + x + " ");

                match st {
                    s if s.trim() == "" => (
                        "You need more argument for this command\n".to_string(),
                        None,
                    ),
                    s => {
                        let cfg = Config {
                            general_color: config.general_color,
                            greeting: (config.greeting),
                            prompt: Prompt {
                                prompt: config.prompt.prompt,
                                prompt_color: s.to_string(),
                            },
                        };

                        match write_config(&cfg) {
                            Ok(_) => (format!("Prompt color has been updated to {}, reload for this to take effect\n",&s).to_string(),None),
                            _ => ("An error occured while writing the config\n".to_string(),None)

                        }
                    }
                }
            }
            Some("greeting_color") => {
                let mut st = "".to_string();
                args.into_iter().for_each(|x| st = st.clone() + x + " ");

                match st {
                    s if s.trim() == "" => (
                        "You need more argument for this command\n".to_string(),
                        None,
                    ),
                    s => {
                        let cfg = Config {
                            general_color: config.general_color,
                            greeting: Greeting {
                                greeting_color: s.to_string(),
                                greeting_message: config.greeting.greeting_message,
                            },
                            prompt: config.prompt,
                        };

                        match write_config(&cfg) {
                            Ok(_) => (format!("Greeting color has been updated to {}, reload for this to take effect\n",&s).to_string(),None),
                            _ => ("An error occured while writing the config\n".to_string(),None)

                        }
                    }
                }
            }
            Some("greeting_message") => {
                let mut st = "".to_string();
                args.into_iter().for_each(|x| st = st.clone() + x + " ");

                match st {
                    s if s.trim() == "" => (
                        "You need more argument for this command\n".to_string(),
                        None,
                    ),
                    s => {
                        let cfg = Config {
                            general_color: config.general_color,
                            greeting: Greeting {
                                greeting_message: s.to_string(),
                                greeting_color: config.greeting.greeting_color,
                            },
                            prompt: config.prompt,
                        };

                        match write_config(&cfg) {
                            Ok(_) => (
                                format!(
                                "Prompt has been updated to {}, reload for this to take effect\n",
                                &s
                            )
                                .to_string(),
                                None,
                            ),
                            _ => (
                                "An error occured while writing the config\n".to_string(),
                                None,
                            ),
                        }
                    }
                }
            }
            _ => (
                "You need more argument for this command\n".to_string(),
                None,
            ),
        }
    }

    handle_second_argument(config, args.nth(0), args)
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

fn show_help_config() -> (String, Option<Config>) {
    (format!("Config help, \n > config show: show config \n > config set: set config \n > config reload: reload config \n > config reset: reset config\n\n"),None)
}

fn handle_config(line: &str, config: Config) -> (String, Option<Config>) {
    match line.strip_prefix("config") {
        None => show_help_config(),
        Some(t) => {
            let mut w = t.split_whitespace();
            match w.nth(0) {
                None => show_help_config(),
                Some("set") => set_config(config, &mut w.clone()),
                Some("reload") => reload_config(),
                Some("reset") => reset_config(),
                _ => show_config(config.clone()),
            }
        }
    }
}

fn main() {
    let mut config = match load() {
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
    let mut text = &loaded.clone().prompt;
    let mut verbose = false;
    let version: String = "v2.9.4".to_string();
    interface.set_completer(Arc::new(CalcCompleter));
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
                    " Calc {version} Help \n > info : show infos \n > exit : exit the program \n > help : print this help \n > verbose : toggle the verbose \n > version : prints the version \n > config : root of the config \n"
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
                    let (s, q) = handle_config(&line, config.clone());
                    match q {
                        Some(q) => {
                            config = q.clone();
                            loaded = load_config(q);
                            text = &loaded.prompt;
                            interface
                                .set_prompt(&format!(
                                    "\x01{prefix}\x02{text}\x01{suffix}\x02",
                                    prefix = style.prefix(),
                                    text = text,
                                    suffix = style.suffix()
                                ))
                                .unwrap();
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

struct CalcCompleter;

static CMD: &[&str] = &["config", "exit", "verbose", "version", "help", "info"];
static CONFIG_CMD: &[&str] = &["reload", "reset", "set", "show"];
static SET_CMD: &[&str] = &[
    "general_color",
    "greeting_color",
    "greeting_message",
    "prompt",
    "prompt_color",
];
static CMD_COLOR: &[&str] = &[
    "black", "purple", "cyan", "blue", "red", "yellow", "green", "white",
];

impl<Term: Terminal> Completer<Term> for CalcCompleter {
    fn complete(
        &self,
        word: &str,
        prompter: &linefeed::Prompter<Term>,
        start: usize,
        _end: usize,
    ) -> Option<Vec<linefeed::Completion>> {
        let line = prompter.buffer();
        let mut words = line[..start].split_whitespace();

        match words.next() {
            None => {
                let mut co = Vec::new();

                for cmd in CMD {
                    if cmd.starts_with(word) {
                        co.push(Completion::simple(cmd.to_string()));
                    }
                }

                Some(co)
            }
            Some("config") => match words.next() {
                None => {
                    let mut co = Vec::new();

                    for cmd in CONFIG_CMD {
                        if cmd.starts_with(word) {
                            co.push(Completion::simple(cmd.to_string()))
                        }
                    }

                    Some(co)
                }
                Some("set") => match words.next() {
                    None => {
                        let mut co: Vec<Completion> = Vec::new();

                        for cmd in SET_CMD {
                            if cmd.starts_with(word) {
                                co.push(Completion::simple(cmd.to_string()))
                            }
                        }

                        Some(co)
                    }
                    Some(c) => {
                        if SET_CMD.contains(&c) {
                            let mut co = Vec::new();
                            for cmd in CMD_COLOR {
                                if cmd.starts_with(word) {
                                    co.push(Completion::simple(cmd.to_string()))
                                }
                            }
                            Some(co)
                        } else {
                            None
                        }
                    }
                },
                _ => None,
            },
            _ => None,
        }
    }
}

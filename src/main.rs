mod tables;
#[cfg(test)]
mod tests;

use clap::Parser;
use copypasta::x11_clipboard::{Primary, X11ClipboardContext};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::thread;
use std::time::Duration;
use tables::{ENG_TO_RU, RU_TO_ENG};
use xdotool::keyboard::send_key;
use xdotool::option_vec;
use xdotool::OptionVec;

// TODO: consider switching away from xdotool system dependency (currently
//       the program won't work if xdotool is not installed)

// TODO: implement saving removed selection to some kind of storage so that
//       it can be restored via another hotkey

// TODO: add error logging

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    table: String,
}

fn main() {
    let args = Args::parse();
    // I don't know why yet, but without this delay program invocation
    // via OS hotkey binding isn't working from time to time
    thread::sleep(Duration::from_millis(50));
    let user_selection = X11ClipboardContext::<Primary>::new()
        .expect("Failed to initialize X11 connection.")
        .get_contents()
        .expect("Failed to read user selection.");
    let mut clipboard = ClipboardContext::new().unwrap();
    let substitute = get_substitute(user_selection, args.table);
    clipboard.set_contents(substitute).unwrap();
    send_key("ctrl+v", option_vec![]);
}

fn get_substitute(user_input: String, table_name: String) -> String {
    user_input
        .chars()
        .map(|c| {
            let sub = match table_name.as_str() {
                "eng-to-ru" => ENG_TO_RU.get(&c.to_string()),
                "ru-to-eng" => RU_TO_ENG.get(&c.to_string()),
                _ => panic!("Unknown table name: {}", table_name),
            };
            match sub {
                Some(s) => s.to_string(),
                None => c.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

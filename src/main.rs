mod tables;
#[cfg(test)]
mod tests;

use chrono::prelude::*;
use clap::Parser;
use copypasta::x11_clipboard::{Primary, X11ClipboardContext};
use copypasta::{ClipboardContext, ClipboardProvider};
use regex::RegexBuilder;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use tables::{ENG_TO_RU, RU_TO_ENG};
use xdotool::keyboard::send_key;
use xdotool::option_vec;
use xdotool::OptionVec;

// TODO: consider switching away from xdotool system dependency (currently
//       the program won't work if xdotool is not installed)

// TODO: implement saving removed selection to some kind of storage so that
//       it can be restored via another hotkey

// TODO: add error logging instead of panicking

// TODO: figure out what is the matter with `thread::sleep` - the program
//       works in an unstable manner without it

fn main() {
    send_log("Starting...", None);

    let args = Args::parse();
    args.validate();
    let logfile_binding = OpenOptions::new()
        .write(true)
        .append(true)
        .open(args.logfile_name.as_ref().unwrap())
        .ok();
    let logfile = logfile_binding.as_ref();
    send_log("Parsed and validated args...", logfile);

    let translation_table = args.get_trans_table();
    send_log("Got table...", logfile);

    let user_selection = X11ClipboardContext::<Primary>::new()
    .expect("Failed to initialize X11 connection.")
    .get_contents()
    .expect("Failed to read user selection.");
    send_log(
        &format!("Got user selection: {} ...", &user_selection),
        logfile,
    );

    let mut clipboard = ClipboardContext::new().unwrap();
    let substitute = translation_table.translate(user_selection);
    thread::sleep(Duration::from_millis(20));
    send_log(
        &format!("Translated user selection: {} ...", &substitute),
        logfile,
    );

    clipboard.set_contents(substitute).unwrap();
    thread::sleep(Duration::from_millis(20));
    send_log("Set clipboard contents...", logfile);

    send_key("ctrl+v", option_vec![]);
    send_log("Sent ctrl+v...", logfile);

    thread::sleep(Duration::from_millis(20));
    send_log("Done.", logfile);
}

fn send_log(log: &str, logfile: Option<&File>) {
    let timestamp: DateTime<Local> = Local::now();
    let msg = format!("[{}] {}", timestamp, log);
    if logfile.is_some() {
        writeln!(logfile.unwrap(), "{msg}").expect("Could not write log message to a logfile!");
    }
    println!("{}", msg);
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    table: Option<String>,

    #[arg(short, long)]
    from_file: Option<String>,

    #[arg(short, long)]
    logfile_name: Option<String>,
}

impl Args {
    fn validate(&self) {
        if self.table.is_none() && self.from_file.is_none() {
            panic!("Either --table or --from-file argument must be specified.");
        }
        if self.table.is_some() && self.from_file.is_some() {
            panic!(
                "Only one of --table or --from-file arguments must be \
                 specified, not both"
            );
        }
    }

    fn get_trans_table(&self) -> TransTable {
        match (&self.table, &self.from_file) {
            (Some(table_name), None) => match table_name.as_str() {
                "eng-to-ru" => TransTable::Precompiled(&ENG_TO_RU),
                "ru-to-eng" => TransTable::Precompiled(&RU_TO_ENG),
                _ => panic!("Unknown table name: {}", table_name),
            },
            (None, Some(file_name)) => TransTable::from_custom_table(&file_name),
            (None, None) => unreachable!(),
            (Some(_), Some(_)) => unreachable!(),
        }
    }
}

enum TransTable {
    FromUserInput(HashMap<String, String>),
    Precompiled(&'static phf::Map<&'static str, &'static str>),
}

impl TransTable {
    fn get_sub(&self, key: &str) -> Option<&str> {
        match self {
            TransTable::FromUserInput(table) => table.get(key).map(|s| s.as_str()),
            TransTable::Precompiled(table) => table.get(key).copied(),
        }
    }

    fn translate(&self, user_input: String) -> String {
        user_input
            .chars()
            .map(|c| {
                let sub = self.get_sub(&c.to_string());
                match sub {
                    Some(s) => s.to_string(),
                    None => c.to_string(),
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn from_custom_table(file_name: &String) -> Self {
        let meminfo_str = std::fs::read_to_string(file_name).unwrap();

        let re = RegexBuilder::new(r"^(\S+):\s+(.+)$")
            .multi_line(true)
            .build()
            .unwrap();

        let meminfo = re
            .captures_iter(&meminfo_str)
            .map(|cap| {
                (
                    cap.get(1).unwrap().as_str().to_string(),
                    cap.get(2).unwrap().as_str().to_string(),
                )
            })
            .collect::<HashMap<_, _>>();

        Self::FromUserInput(meminfo)
    }
}

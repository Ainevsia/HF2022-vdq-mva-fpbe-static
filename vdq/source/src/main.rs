use std::collections::VecDeque;
use std::io::{stdin, BufRead};
use std::fmt::Debug;
use serde::Deserialize;

static HACKER_MESSAGE: &'static str = "Hackers not allowed 💻";
static MALFMT_MESSAGE: &'static str = "Illegal format 🧐";

#[derive(Deserialize, Debug)]
enum Operation {
    Add, Remove, Append, Archive, View,
}

#[derive(Debug)]
struct Note {
    idx: Option<usize>, // + 0x00 Option/None, real usize
    msg: Vec<u8>,       // + 0x10 ptr cap len
}                       // + 0x28

fn banner() {
    let banner_words = [
    "       ██   ██ ███████  ██████ ████████ ███████ ██████   ██████  ██████  ██████  ",
    "       ██   ██ ██      ██         ██    ██           ██ ██  ████      ██      ██ ",
    "       ███████ █████   ██         ██    █████    █████  ██ ██ ██  █████   █████  ",
    "       ██   ██ ██      ██         ██    ██      ██      ████  ██ ██      ██      ",
    "       ██   ██ ██       ██████    ██    ██      ███████  ██████  ███████ ███████ ",
    "   Welcome to HFCTF 2022 🤔 !",
    ];
    let banner = banner_words.join("\n");
    println!("{}", banner);
}

fn get_opr_lst() -> Vec<Operation> {
    let mut user_input = String::new();

    for line_result in stdin().lock().lines() {
        let line = line_result.expect(HACKER_MESSAGE);
        if line.starts_with("$") { break }
        user_input += &line;
    }

    serde_json::from_str::<Vec<Operation>>(&user_input).expect(MALFMT_MESSAGE)
}

fn get_raw_line() -> Vec<u8> {
    let mut new_bytes = vec![];
    stdin().lock().read_until(b'\n', &mut new_bytes).expect(MALFMT_MESSAGE);
    if new_bytes.ends_with(&[b'\n']) { new_bytes.pop(); }
    new_bytes
}

fn handle_opr_lst(opr_lst: Vec<Operation>) {
    let mut notesq = VecDeque::with_capacity(2);
    let mut archived_notesq = vec![];
    let mut gidx = 0;
    for operation in opr_lst {
        match operation {
            Operation::Add => {
                gidx += 1;
                println!("Add note [{}] with message : ", gidx);
                notesq.push_back(Box::new(Note{
                    idx: Some(gidx),
                    msg: get_raw_line()
                }));
            }
            Operation::Remove => {
                if let Some(note) = notesq.pop_front() {
                    if let Some(idx) = note.idx {
                        println!("Removed note [{}]", idx);
                    }
                }
            }
            Operation::Append => {
                if let Some(mut note) = notesq.pop_front() {
                    println!("Append with message : ");
                    note.msg.append(&mut get_raw_line());
                    notesq.push_front(note);
                }
            }
            Operation::Archive => {
                if let Some(mut note_to_archive) = notesq.pop_front () {
                    if let Some(idx) = note_to_archive.idx {
                        note_to_archive.idx = None;     // Archived notes do not have idx
                        archived_notesq.push(note_to_archive);
                        println!("Archive note [{}]", idx);
                    }
                }
            }
            Operation::View => {
                println!("Cached notes:");
                notesq.make_contiguous();
                notesq.iter().for_each(|note| {
                    if let Ok(utf8_str) = std::str::from_utf8(&note.msg) {      // is a utf-8 sting
                        println!(" -> {}", utf8_str);
                    } else {                        // not a valid utf-8 sting, try to output hex
                        print!(" -> ");
                        note.msg.iter().for_each(|byte| {
                            print!("{:02x}", byte);
                        });
                        println!("");
                    }
                });
            }
        }
    }
}

fn main() {
    banner();
    let opr_lst = get_opr_lst();
    handle_opr_lst(opr_lst);
    println!("Bye 👋");
}
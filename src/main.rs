mod wordlists;

use rand::prelude::*;
use std::io::{self, prelude::*};
use std::time::Instant;
use treap_rust::{bst::BST, treap::Treap, Treap as TreapRec};
use wordlists::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut treap = Treap::<String, i32, String>::new();
    let mut treap_rec = TreapRec::<String, i32, String>::new();
    let mut bst = BST::<String, String>::new();
    let mut timer = TimingContext::new();

    let mut words_iter = WORDS_UNSORTED.into_iter();

    loop {
        let command =
            prompt_user("> Enter a command (insert | find | print | time | load | exit): ");
        match &command[..] {
            "time" => {
                eprintln!(">> The next operation will be timed");
                timer.activate();
            }
            "load" => {
                let num = prompt_user("How many to load: ");
                let mut num = num.parse::<usize>().unwrap();
                let mut count = 0;
                while let Some(&word) = words_iter.next() {
                    let weight = rng.gen();
                    let word = word.to_owned();
                    // let rword: String = word.chars().rev().collect();
                    treap.insert(word.clone(), weight, word.clone());
                    treap_rec.insert(word.clone(), weight, word.clone());
                    bst.insert(word.clone(), word.clone());
                    // treap.insert(rword.clone(), weight, rword.clone());
                    // treap_rec.insert(rword.clone(), weight, rword.clone());
                    // bst.insert(rword.clone(), rword.clone());
                    count += 1;
                    num -= 1;
                    if num == 0 {
                        break;
                    }
                }
                eprintln!(">> Loaded {} words", count);
            }
            "insert" => {
                let english = prompt_user("Enter english word: ");
                let weight = rng.gen();
                let german = prompt_user("Enter german word: ");
                let (e2, g2) = (english.clone(), german.clone());
                let (e3, g3) = (english.clone(), german.clone());

                timer.start();
                treap.insert(english, weight, german);
                timer.evaluate("Treap");

                timer.start();
                treap_rec.insert(e2, weight, g2);
                timer.evaluate("TreapRec");

                timer.start();
                bst.insert(e3, g3);
                timer.evaluate("BST");

                timer.deactivate();
            }
            "exit" => {
                eprintln!(">> Bye ;)");
                break;
            }
            "print" => {
                eprintln!("{:?}", &treap);
                dbg!(&treap_rec);
                dbg!(&bst);
            }
            "find" => {
                let english = prompt_user("> Enter english word to find: ");

                timer.start();
                let result = treap.find(&english);
                timer.evaluate("Treap");
                if let Some(german) = result {
                    println!("true {}", &*german);
                } else {
                    println!("false");
                }

                timer.start();
                let result = treap_rec.find(&english);
                timer.evaluate("TreapRec");
                if let Some(german) = result {
                    println!("true {}", german);
                } else {
                    println!("false");
                }

                timer.start();
                let result = bst.find(&english);
                timer.evaluate("BST");
                if let Some(german) = result {
                    println!("true {}", german);
                } else {
                    println!("false");
                }
                timer.deactivate();
            }
            _ => println!(">> ERR: unrecognized command"),
        }
    }
}

struct TimingContext {
    active: bool,
    start: Instant,
}

impl TimingContext {
    pub fn new() -> Self {
        Self {
            active: false,
            start: Instant::now(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn start(&mut self) {
        if self.active {
            self.start = Instant::now();
        }
    }

    pub fn evaluate(&mut self, name: &str) {
        if self.active {
            let dur = Instant::now() - self.start;
            eprintln!(
                "Previous operation for {} completed in {}ns",
                name,
                dur.as_nanos()
            );
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

fn prompt_user(prompt: &str) -> String {
    let mut result = String::new();
    eprintln!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut result).unwrap();
    result.trim().to_owned()
}

/// Cryptoquip solving tool.
///
/// This program takes a "cryptoquip" -- a phrase encrypted using a
/// simple monoalphabetic substitution cipher -- and displays it after
/// applying a (partial) guessed transform.  The UI allows the user to
/// add mappings or delete mappings and see the updated result.

// jwblor bx snbswn utb otgrh aynqhxqjo inynqw aygrlj tqssgrnjj: "wgxn
// gj q abuw bx itnnygbj."

use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::env;
use std::io;
use std::io::Write;

struct Cryptoquip {
    ciphertext: String,
    map: HashMap<char, char>,
    key_count: HashMap<char, u32>,
}

// Everything is an xterm/vt100?!?  This should use terminfo or
// equivalent, whatever is available from Rust.
fn inverse_text() -> &'static str {
    "\x1b[7m"
}

fn normal_text() -> &'static str {
    "\x1b[0m"
}

impl Cryptoquip {
    fn new(c: &str) -> Cryptoquip {
        Cryptoquip {
            ciphertext: c.to_string(),
            map: HashMap::new(),
            key_count: HashMap::new(),
        }
    }

    fn reset_map(&mut self) {
        self.map.clear();
        self.key_count.clear();
    }

    fn reset(&mut self, c: &str) {
        self.reset_map();
        self.ciphertext = c.to_string();
    }

    fn show_orig(&self) {
        println!("{}", self.ciphertext);
    }

    fn show_decoded(&self) {
        for c in self.ciphertext.chars() {
            if self.map.contains_key(&c) {
                let image = self.map[&c];
                if self.key_count[&image] > 1 {
                    print!("{}{}{}", inverse_text(), image, normal_text());
                } else {
                    print!("{}", image);
                }
            } else if !c.is_alphabetic() {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!("");
    }

    /// Update the mapping function so occurences of `pre` will be
    /// replaced with `post`.  The value of `post` can be "." to erase
    /// the existing mapping.
    fn update(&mut self, pre: char, post: char) {
        // decrement key count for previous image for pre, if any.
        {
            let old_rec = self.map.entry(pre);
            match old_rec {
                Occupied(old_entry) => {
                    let prev = old_entry.get();
                    let record = self.key_count.entry(*prev);
                    match record {
                        Occupied(mut oe) => *oe.get_mut() -= 1,
                        Vacant(_) => panic!("image count missing"),
                    }
                },
                Vacant(_) => {},
            }
        }  // old_rec scope

        if post != '.' {
            // increment key count for new image
            let record = self.key_count.entry(post).or_insert(0);
            *record += 1;
            // update map
            self.map.insert(pre, post);
        } else {
            // remove guess for pre-image
            self.map.remove(&pre);
        }
    }

    fn print(&self) {
        self.show_orig();
        self.show_decoded();
    }

    fn repl(&mut self) {
        let mut line = String::new();
        loop {
            line.clear();
            print!("cq: "); io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line)
                .expect("failed to read line");
            line = line.trim().to_string();
            if line.len() == 0 {
                continue;
            }
            if line.starts_with("?") {
                println!("?            - this help");
                println!("p            - print ciphertext / decoded text");
                println!("N ciphertext - new cryptoquip with ciphertext");
                println!("c            - clear mappings");
                println!("r abcd...    - replace a with b, c with d, etc");
                println!("               post-image can be \".\" to remove");
                println!("               the current guess for the pre-image");
                println!("q            - quit");
            } else if line.starts_with("p") {
                self.print();
            } else if line.starts_with("q") {
                return;
            } else if line.starts_with("N") {
                self.reset(&line[1..].trim());
                self.print();
            } else if line.starts_with("c") {
                self.reset_map();
                self.print();
            } else if line.starts_with("r") {
                let rest = line[1..].trim().to_string();
                if rest.len() % 2 != 0 {
                    println!("update should have pairs of preimage/image characters");
                    continue;
                }
                let mut expect_post = false;
                let mut pre: char = ' ';
                for c in rest.chars() {
                    if !c.is_alphabetic() && !(expect_post && c == '.') {
                        println!("update should involve only alphabetic characters");
                        break;
                    }
                    if !expect_post {
                        pre = c;
                    } else {
                        self.update(pre, c);
                    }
                    expect_post = !expect_post;
                }
                self.show_decoded();
            } else {
                println!("Not understood: {}", line);
                println!("Enter \"?\" to get help.");
            }
        }
    }
}

fn main() {
    let default_cryptogram = "jwblqr bx snbswn utb otgrh aynqhxqjo inynqw aygrlj tqssgrnjj: \"wgxn gj q abuw bx itnnygbj.\"";
    let av: Vec<String> = env::args().collect();
    let cryptogram = match av.len() {
        1 => default_cryptogram.to_string(),
        _ => av[1..].join(" "),
    };
    let mut quip = Cryptoquip::new(&cryptogram);
    quip.show_orig();
    quip.repl();
}

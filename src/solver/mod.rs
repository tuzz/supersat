use std::process::Command;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

pub struct Solver {

}

impl Solver {
    pub fn solve(input: &String) -> bool {
        let filename = format!("{}.output", input);

        if !Path::new(&filename).exists() {
            let output = File::create(&filename).unwrap();

            Command::new("lingeling")
                .arg(input)
                .stdout(output)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }

        let output = File::open(&filename).unwrap();

        for line in BufReader::new(output).lines() {
            let line = line.unwrap();

            if line.contains("s SATISFIABLE") {
                return true;
            } else if line.contains("s UNSATISFIABLE") {
                return false;
            }
        }

        panic!("Failed to parse {}", filename);
    }
}

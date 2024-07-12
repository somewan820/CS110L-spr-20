use std::env;
use std::fs::File;
use std::process;
use std::io::{self, BufRead}; // For read_file_lines()

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    // Be sure to delete the #[allow(unused)] line above
    let mut line_vec: Vec<String> = Vec::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        line_vec.push(line_str);
    }
    Ok(line_vec)
}

fn nums_file_words(line: &Vec<String>) -> Result<usize, io::Error> {
    let mut words_num: usize = 0;
    for i in 0..line.len() {
        words_num +=  line[i].split_whitespace().collect::<Vec<_>>().len();
    }
    Ok(words_num)
}

fn nums_file_char(line: &Vec<String>) -> Result<usize, io::Error> {
    let mut char_num: usize = 0;
    for i in 0..line.len() {
        char_num +=  line[i].len();
    }  
    Ok(char_num)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let line_vec = read_file_lines(filename).expect("File Read Error!");
    let word_nums = nums_file_words(&line_vec).expect("Line Count Error!");
    let char_nums = nums_file_char(&line_vec).expect("Char Count Error!");
    println!("Lines: {}", line_vec.len());
    println!("Words: {}", word_nums);
    println!("Char: {}", char_nums);
}

extern crate rand;
#[warn(unused_imports)]
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    let correct_word = secret_word.clone();
    let len = secret_word.len();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let mut secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    //println!("random word: {}", secret_word);

    // Your code here! :)
    let mut chance_2_guess_count = NUM_INCORRECT_GUESSES;
    let mut print =String::from("-").repeat(len);
    let mut guessed = String::new();
    println!("Welcome to CS110L Hangman!");

    loop {
        println!("The word so far is {}", print);
        println!("You have guessed the following letters: {}", guessed);
        println!("You have {} guesses left", chance_2_guess_count);
        
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let guess_char = guess.trim().chars().next().expect("No character entered");

        let index = match secret_word_chars.iter().position(|&x| x == guess_char) {
            Some(i) => i,
            None => {
                println!("Sorry, that letter is not in the word");
                println!();
                chance_2_guess_count = chance_2_guess_count - 1;
                if chance_2_guess_count == 0 {
                    println!("Sorry, you ran put of guesses!");
                    return;
                }
                continue;
            }
        };
        print = print
            .chars()
            .take(index)
            .chain(std::iter::once(secret_word_chars[index]))
            .chain(print.chars().skip(index + 1))
            .collect();
        guessed.push(guess_char);
        secret_word_chars[index] = '#';

        println!();
        if print == correct_word {
            println!("Congratulations you guessed the secret word: {}", print);
            break;
        }
    }

}

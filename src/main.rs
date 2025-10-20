use std::io;
use std::thread;
use std::time::Duration;

mod text;

fn main() 
{
    println!("Welcome to the game Hangman!");

    let mut start_input = String::new();

    'start_menu: loop
    {
        println!();
        println!("What do you want to do? \n\
        1.Start \n\
        2.Exit");

        io::stdin()
        .read_line(&mut start_input)
        .expect("Failed to read line");

        let start_input = start_input.trim();

        if start_input == "1" 
        {
            game_loop();
        }
        else if start_input == "2"
        {
            break 'start_menu;
        }
        else
        {
            println!();
            println!("Invalid input!, Use the input 1 or 2");
        }
    }
}

fn game_loop() 
{
    let mut wins: i32 = 0;
    let mut loses: i32 = 0;
    let mut if_already_played: bool = false;
    let mut contestant_number: i32 = 1;

    println!();
    println!("Starting Game");

    thread::sleep(Duration::from_secs(1));

    println!();
    println!("This game will not have a save system because I don't want to bother with it. \n\
    If you want to save your scores I suggest you write them down.");

    thread::sleep(Duration::from_secs(1));

    'game_menu: loop
    {
        if !if_already_played 
        {
            println!();
            println!("Welcome to a non traditional game of hangman!");
            println!();

            thread::sleep(Duration::from_secs(1));

            let intro_text = "You might be wondering how the game is played. \n\
            Simple Bob is a fan of hangman and has always wanted to host a tournament. \n\
            But oh no, nobody wants to participate so bob decides to voluntel some people to participate. \n\
            Bob thinks that drawing a hangman is boring, so he has a better idea that will also motivate the contestants. \n\
            He is going to take a step of loading his gun for every incorrect guess, guess wrong enough times and... \n\
            Well you know what happens next.";

            for line in intro_text.lines()
            {
                println!("{}", line);

                thread::sleep(Duration::from_secs(1));
            }
        }
        else 
        {
            println!();
            println!("A new contestant I see");
        }

        thread::sleep(Duration::from_secs(1));

        println!();
        println!("Welcome to the event contestant numer: {}", contestant_number);

        thread::sleep(Duration::from_secs(1));

        let mut game_menu_input = String::new();

        println!();
        println!("What do you want to do? \n\
        1.Play \n\
        2.Check Scores \n\
        3.Exit");

        io::stdin()
        .read_line(&mut game_menu_input)
        .expect("Failed to read line");

        let game_menu_input = game_menu_input.trim();

        match game_menu_input
        {
            "1" =>
            {
                println!();
                println!("Starting Match");

                thread::sleep(Duration::from_secs(1));

                println!();
                println!("Bob: Welcome contestants to Bob's hangman Spectacular!");

                thread::sleep(Duration::from_secs(1));

                println!();
                println!("Bob: Let me think of a word");

                let word_to_guess = text::get_random_word().to_uppercase();
                let word_chars: Vec<char> = word_to_guess.chars().collect();

                let mut current_display: Vec<char> = word_chars.iter().map(|_| '_').collect();

                let mut guessed_letters: Vec<char> = Vec::new();
                let mut incorrect_guesses: i32 = 0;
                const MAX_INCORRECT_GUESSES: i32 = 6;

                thread::sleep(Duration::from_secs(1));

                println!();
                println!("Bob:Ok I have thought of a word");

                thread::sleep(Duration::from_secs(1));

                'hangman_loop: loop
                {
                    println!();
                    println!("Word: \n\
                    {}", current_display.iter().collect::<String>());
                    println!("Words guessed: {}", guessed_letters.iter().collect::<String>());
                    println!("Incorrect guesses {}/{}", incorrect_guesses, MAX_INCORRECT_GUESSES);

                    if !current_display.contains(&'_')
                    {
                        println!();
                        println!("Bob: Seems you have guessed correctly contestant number {}", contestant_number);

                        thread::sleep(Duration::from_secs(3));

                        wins += 1;

                        break 'hangman_loop;
                    }

                    if incorrect_guesses == MAX_INCORRECT_GUESSES
                    {
                        println!();
                        println!("Bob: I was so hopefull that  you might win but the word was {} \n\
                        I would say better luck next time but there isn't a next time.", word_to_guess);
                        println!();
                        println!("BANG!");

                        thread::sleep(Duration::from_secs(5));

                        loses += 1;

                        contestant_number += 1;

                        break 'hangman_loop;
                    }

                    let mut guess_input = String::new();

                    println!();
                    println!("Input your guess (letters only)");

                    io::stdin()
                        .read_line(&mut guess_input)
                        .expect("Failed to read line");

                    let guess_char:char = match guess_input.trim().to_uppercase().chars().next()
                    {
                        Some(c) if c.is_alphabetic() => c,

                        _ => 
                        {
                            println!();
                            println!("Invalid input! Input needs to be a character");

                            thread::sleep(Duration::from_secs(1));

                            continue 'hangman_loop;
                        }
                    };

                    if guessed_letters.contains(&guess_char)
                    {
                        println!();
                        println!("You already guessed {}", guess_char);
                        
                        thread::sleep(Duration::from_secs(1));

                        continue 'hangman_loop;
                    }

                    guessed_letters.push(guess_char);

                    let mut match_found = false;

                    for (index, char_in_word) in word_chars.iter().enumerate() 
                    {
                        if *char_in_word == guess_char
                        {
                            current_display[index] = guess_char;
                            match_found = true;
                        }
                    }

                    if match_found 
                    {
                        println!();
                        println!("Guesed correctly");

                        thread::sleep(Duration::from_secs(1));
                    }
                    else
                    {
                        println!();
                        println!("Guessed incorrectly");

                        incorrect_guesses += 1;

                        thread::sleep(Duration::from_secs(1));
                    }
                }

                if_already_played = true;
            },

            "2" =>
            {
                println!();
                println!("Your scores so far are: \n\
                |Wins: {} \n\
                |Loses: {}",
                wins,
                loses);

                thread::sleep(Duration::from_secs(10));
            },

            "3" =>
            {
                println!();
                println!("Exiting game");

                thread::sleep(Duration::from_secs(1));

                break 'game_menu;
            },

            _ => 
            {
                println!();
                println!("Invalid input! Please try again");

                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

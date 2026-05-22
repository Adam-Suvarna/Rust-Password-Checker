use std::io;

fn check_length(password: &str) -> u8 {
    if password.len() >= 15 {
        2
    } else if password.len() >= 8 {
        1
    } else {
        0
    }
}

fn check_has_number(password: &str) -> bool {
    password.chars().any(|c| c.is_numeric())
}

fn check_has_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_uppercase())
}

fn check_has_special(password: &str) -> bool {
    password.chars().any(|c| !c.is_alphanumeric())
}

fn score_password(password: &str) -> u8 {
    let mut score: u8 = 0;

    score += check_length(password);

    if check_has_number(password) {
        score += 1;
    }
    if check_has_uppercase(password) {
        score += 1;
    }
    if check_has_special(password) {
        score += 1;
    }

    score
}

fn get_rating(score: u8) -> &'static str {
    if score >= 4 {
        "Strong"
    } else if score >= 2 {
        "Medium"
    } else {
        "Weak"
    }
}

fn main() {
    println!("Password Strength Checker");
    println!("Enter a password to analyse:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");

    let password = input.trim();

    let score = score_password(password);
    let rating = get_rating(score);

    println!("\nLength:        {} characters", password.len());
    println!("Has number:    {}", check_has_number(password));
    println!("Has uppercase: {}", check_has_uppercase(password));
    println!("Has special:   {}", check_has_special(password));
    println!("Score:         {}/5", score);
    println!("Rating:        {}", rating);
}
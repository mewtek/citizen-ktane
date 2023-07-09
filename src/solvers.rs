use std::io::{self, Read};
use std::collections::HashMap;

#[path = "../src/checks.rs"]    // For some reason rust tries to look in the solvers/ directory
mod checks;

// #[path = "../src/utils.rs"]
// mod utils;

pub struct Bomb {
    pub serial: String,
    pub car: bool,
    pub frq: bool,
    pub battery_count: i32,
    pub strikes: i32,
}

// These functions are pretty much in the order of the manual on https://bombmanual.com

pub fn defuse_wires(wire_sequence: Vec<&str>, bomb_info: Bomb)
{  
    if wire_sequence.len() == 3
    {
        if !wire_sequence.contains(&&"red")
        {
            println!("Cut the second wire.");
            return;
        }

        if wire_sequence[wire_sequence.len()-1] == "white"
        {
            println!("Cut the last wire.");
            return;
        }

        if wire_sequence.iter().filter(|&n| *n == "blue").count() > 1
        {
            println!("Cut the last blue wire.");
            return;
        }
        
        println!("Cut the last wire.");
        return;
    }

    if wire_sequence.len() == 4
    {
        if wire_sequence.iter().filter(|&n| *n == "red").count() > 1 && checks::last_digit_odd(bomb_info.serial)
        {
            println!("Cut the last red wire.");
            return;
        }

        if wire_sequence[wire_sequence.len()-1] == "yellow" && wire_sequence.iter().filter(|&n| *n == "red").count() == 0
        {
            println!("Cut the first wire.");
            return;
        }

        if wire_sequence.iter().filter(|&n| *n == "blue").count() == 1
        {
            println!("Cut the first wire.");
            return;
        }

        if wire_sequence.iter().filter(|&n| *n == "yellow").count() > 1
        {
            println!("Cut the last wire.");
            return;
        }

        println!("Cut the second wire.");
        return;
    }

    if wire_sequence.len() == 5
    {
        if wire_sequence[wire_sequence.len()-1] == "black" && checks::last_digit_odd(bomb_info.serial)
        {
            println!("Cut the fourth wire.");
            return;
        }

        if wire_sequence[wire_sequence.len()-1] == "yellow" && wire_sequence.iter().filter(|&n| *n == "red").count() == 0
        {
            println!("Cut the first wire.");
            return;
        }

        if wire_sequence.iter().filter(|&n| *n == "blue").count() == 1
        {
            println!("Cut the first wire.");
            return;
        }

        if wire_sequence.iter().filter(|&n| *n == "yellow").count() > 1
        {
            println!("Cut the last wire.");
            return;
        }

        println!("Cut the second wire.");
        return;
    }

    if wire_sequence.len() == 6
    {
        if !wire_sequence.contains(&"yellow") && checks::last_digit_odd(bomb_info.serial)
        {
            println!("Cut the third wire.");
            return;
        }

        if wire_sequence.iter().filter(|&n| *n == "yellow").count() == 1 && wire_sequence.iter().filter(|&n| *n == "white").count() > 1
        {
            println!("Cut the fourth wire.");
            return;
        }

        if wire_sequence.iter().filter(|&n| *n == "red").count() == 0
        {
            println!("Cut the last wire.");
            return;
        }

        println!("Cut the fourth wire.");
        return;
    }
}

pub fn defuse_button(bomb_info: Bomb)
{
    let mut button_color = String::new();
    let mut button_text = String::new();

    println!("What color is the button?");
    io::stdin().read_line(&mut button_color).unwrap();

    println!("What does the button say?");
    io::stdin().read_line(&mut button_text).unwrap();

    if button_color.trim() == "blue" && button_text.trim() == "abort"
    {
        defuse_button_strip();
        return;
    }

    if button_text.trim() == "detonate" && bomb_info.battery_count > 1
    {
        println!("Press and immediately release the button.");
        return;
    }

    if button_color.trim() == "white" && bomb_info.car
    {
        defuse_button_strip();
        return;
    }

    if bomb_info.battery_count > 2 && bomb_info.frq
    {
        println!("Press and immediately release the button.");
        return;
    }

    if button_color.trim() == "yellow"
    {
        defuse_button_strip();
        return;
    }

    if button_color.trim() == "red" && button_text.trim() == "hold"
    {
        println!("Press and immediately release the button.");
        return;
    }

    defuse_button_strip();
    return;
}


fn defuse_button_strip()
{
    let mut strip_color = String::new();
    println!("What color is the strip flashing?");
    io::stdin().read_line(&mut strip_color).unwrap();

    if strip_color.trim() == "blue"
    {
        println!("Release the button when there is a 4 in any position.");
        return;
    }

    if strip_color.trim() == "white"
    {
        println!("Release the button when there is a 1 in any position.");
        return;
    }

    if strip_color.trim() == "yellow"
    {
        println!("Release the button when there is a 5 in any position.");
        return;
    }

    println!("Release the button when there is a 1 in any position.");

    return;
}

pub fn defuse_keypads(symbols: Vec<&str>)
{
    // TODO:
    return;
}

pub fn defuse_simon_says(bomb_info: Bomb)
{

    // TODO: Make this more "active" after implementing speech to text into the bot

    if checks::serial_has_vowels(bomb_info.serial)
    {
        if bomb_info.strikes == 0
        {
            println!("Red -> Blue, Blue -> Red, Green -> Yellow, Yellow -> Green");
            return;
        }

        if bomb_info.strikes == 1
        {
            println!("Red -> Yellow, Blue -> Green, Green -> Blue, Yellow -> Red");
            return;
        }

        if bomb_info.strikes == 2
        {
            println!("Red -> Green, Blue -> Red, Green -> Yellow, Yellow -> Blue");
            return;
        }
    }

    if bomb_info.strikes == 0
    {
        println!("Red -> Blue, Blue -> Yellow, Yellow -> Green, Yellow -> Red");
        return;
    }

    if bomb_info.strikes == 1
    {
        println!("Red , Blue, Green -> Yellow, Yellow -> Green");
        return;
    }

    if bomb_info.strikes == 2
    {
        println!("Red -> Yellow, Blue -> Green, Green -> Blue, Yellow -> Red");
        return;
    }
    
    return;
}


pub fn defuse_words()
{
    let word_lists = HashMap::from([
        ("READY", "YES, OKAY, WHAT, MIDDLE, LEFT, PRESS, RIGHT, BLANK, READY, NO, FIRST, UHHH, NOTHING, WAIT"),
        ("FIRST", "LEFT, OKAY, YES, MIDDLE, NO, RIGHT, NOTHING, UHHH, WAIT, READY, BLANK, WHAT, PRESS, FIRST"),
        ("NO", "BLANK, UHHH, WAIT, FIRST, WHAT, READY, RIGHT, YES, NOTHING, LEFT, PRESS, OKAY, NO, MIDDLE"),
        ("BLANK", "WAIT, RIGHT, OKAY, MIDDLE, BLANK, PRESS, READY, NOTHING, NO, WHAT, LEFT, UHHH, YES, FIRST"),
        ("NOTHING", "UHHH, RIGHT, OKAY, MIDDLE, YES, BLANK, NO, PRESS, LEFT, WHAT, WAIT, FIRST, NOTHING, READY"),
        ("YES", "OKAY, RIGHT, UHHH, MIDDLE, FIRST, WHAT, PRESS, READY, NOTHING, YES, LEFT, BLANK, NO, WAIT"),
        ("WHAT", "UHHH, WHAT, LEFT, NOTHING, READY, BLANK, MIDDLE, NO, OKAY, FIRST, WAIT, YES, PRESS, RIGHT"),
        ("UHHH", "READY, NOTHING, LEFT, WHAT, OKAY, YES, RIGHT, NO, PRESS, BLANK, UHHH, MIDDLE, WAIT, FIRST"),
        ("LEFT", "RIGHT, LEFT, FIRST, NO, MIDDLE, YES, BLANK, WHAT, UHHH, WAIT, PRESS, READY, OKAY, NOTHING"),
        ("RIGHT", "YES, NOTHING, READY, PRESS, NO, WAIT, WHAT, RIGHT, MIDDLE, LEFT, UHHH, BLANK, OKAY, FIRST"),
        ("MIDDLE", "BLANK, READY, OKAY, WHAT, NOTHING, PRESS, NO, WAIT, LEFT, MIDDLE, RIGHT, FIRST, UHHH, YES"),
        ("OKAY", "MIDDLE, NO, FIRST, YES, UHHH, NOTHING, WAIT, OKAY, LEFT, READY, BLANK, PRESS, WHAT, RIGHT"),
        ("WAIT", "UHHH, NO, BLANK, OKAY, YES, LEFT, FIRST, PRESS, WHAT, WAIT, NOTHING, READY, RIGHT, MIDDLE"),
        ("PRESS", "RIGHT, MIDDLE, YES, READY, PRESS, OKAY, NOTHING, UHHH, BLANK, LEFT, FIRST, WHAT, NO, WAIT"),
        ("YOU", "SURE, YOU ARE, YOUR, YOU'RE, NEXT, UH HUH, UR, HOLD, WHAT?, YOU, UH UH, LIKE, DONE, U"),
        ("YOU ARE", "YOUR, NEXT, LIKE, UH HUH, WHAT?, DONE, UH UH, HOLD, YOU, U, YOU'RE, SURE, UR, YOU ARE"),
        ("YOUR", "UH UH, YOU ARE, UH HUH, YOUR, NEXT, UR, SURE, U, YOU'RE, YOU, WHAT?, HOLD, LIKE, DONE"),
        ("YOU'RE", "YOU, YOU'RE, UR, NEXT, UH UH, YOU ARE, U, YOUR, WHAT?, UH HUH, SURE, DONE, LIKE, HOLD"),
        ("UR", "DONE, U, UR, UH HUH, WHAT?, SURE, YOUR, HOLD, YOU'RE, LIKE, NEXT, UH UH, YOU ARE, YOU"),
        ("U", "UH HUH, SURE, NEXT, WHAT?, YOU'RE, UR, UH UH, DONE, U, YOU, LIKE, HOLD, YOU ARE, YOUR"),
        ("UH HUH", "UH HUH, YOUR, YOU ARE, YOU, DONE, HOLD, UH UH, NEXT, SURE, LIKE, YOU'RE, UR, U, WHAT?"),
        ("UH UH", "UR, U, YOU ARE, YOU'RE, NEXT, UH UH, DONE, YOU, UH HUH, LIKE, YOUR, SURE, HOLD, WHAT?"),
        ("WHAT?", "YOU, HOLD, YOU'RE, YOUR, U, DONE, UH UH, LIKE, YOU ARE, UH HUH, UR, NEXT, WHAT?, SURE"),
        ("DONE", "SURE, UH HUH, NEXT, WHAT?, YOUR, UR, YOU'RE, HOLD, LIKE, YOU, U, YOU ARE, UH UH, DONE"),
        ("NEXT", "WHAT?, UH HUH, UH UH, YOUR, HOLD, SURE, NEXT, LIKE, DONE, YOU ARE, UR, YOU'RE, U, YOU"),
        ("HOLD", "YOU ARE, U, DONE, UH UH, YOU, UR, SURE, WHAT?, YOU'RE, NEXT, HOLD, UH HUH, YOUR, LIKE"),
        ("SURE", "YOU ARE, DONE, LIKE, YOU'RE, YOU, HOLD, UH HUH, UR, SURE, U, WHAT?, NEXT, YOUR, UH UH"),
        ("LIKE", "YOU'RE, NEXT, U, UR, HOLD, DONE, UH UH, WHAT?, UH HUH, YOU, LIKE, SURE, YOU ARE, YOUR")
    ]);
    let mut display_txt = String::new();
    let mut button_txt = String::new();

    println!("What's on the display?");
    io::stdin().read_line(&mut display_txt).unwrap();

    // Middle left 
    if display_txt.trim().to_lowercase() == "yes" || display_txt.trim().to_lowercase() == "nothing" || display_txt.trim().to_lowercase() == "led" || display_txt.trim().to_lowercase() == "they are"
    {
        println!("What's on the middle left?");
        io::stdin().read_line(&mut button_txt).unwrap();
        println!("{}", word_lists[button_txt.trim()]);
    }

    // Top right
    if display_txt.trim().to_lowercase() == "first" || display_txt.trim().to_lowercase() == "okay" || display_txt.trim().to_lowercase() == "c"
    {
        println!("What's on the top right?");
        io::stdin().read_line(&mut button_txt).unwrap();
        println!("{}", word_lists[button_txt.trim()]);
    }

    // Bottom right
    if display_txt.trim().to_lowercase() == "display" || display_txt.trim().to_lowercase() == "says" || display_txt.trim().to_lowercase() == "no" || display_txt.trim().to_lowercase() == "lead" || display_txt.trim().to_lowercase() == "hold on" || display_txt.trim().to_lowercase() == "you are" || display_txt.trim().to_lowercase() == "there" || display_txt.trim().to_lowercase() == "see" || display_txt.trim().to_lowercase() == "cee"
    {
        println!("What's on the bottom right?");
        io::stdin().read_line(&mut button_txt).unwrap();
        println!("{}", word_lists[button_txt.trim()]);
    }

    // Bottom left
    if display_txt.trim().to_lowercase() == "blank display" || display_txt.trim().to_lowercase() == "reed" || display_txt.trim().to_lowercase() == "they're" || display_txt.trim().to_lowercase() == "leed"
    {
        println!("What's on the bottom left?");
        io::stdin().read_line(&mut button_txt).unwrap();
        println!("{}", word_lists[button_txt.trim()]);
    }

    // Middle right
    if display_txt.trim().to_lowercase() == "red" || display_txt.trim().to_lowercase() == "read" || display_txt.trim().to_lowercase() == "your" || display_txt.trim().to_lowercase() == "you're" || display_txt.trim().to_lowercase() == "their" || display_txt.trim().to_lowercase() == "blank" || display_txt.trim().to_lowercase() == "you" {
        println!("What's on the middle right?");
        io::stdin().read_line(&mut button_txt).unwrap();
        println!("{}", word_lists[button_txt.trim()]);
    } 

    // Top left
    if display_txt.trim().to_lowercase() == "ur"
    {
        println!("What's on the top left?");
        io::stdin().read_line(&mut button_txt).unwrap();
        println!("{}", word_lists[button_txt.trim()]);
    }
}
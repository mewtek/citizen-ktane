use std::io::{self, Read};

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
    // I'm aware of how stupid this is but the process of getting a char itself to an integer is
    // even more stupidly complicated.
    let last_digit_string = bomb_info.serial.chars().last().unwrap().to_string();
    let last_digit = last_digit_string.parse::<i32>().unwrap();


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
        if wire_sequence.iter().filter(|&n| *n == "red").count() > 1 && last_digit % 2 != 0
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
        if wire_sequence[wire_sequence.len()-1] == "black" && last_digit % 2 != 0
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
        if !wire_sequence.contains(&"yellow") && last_digit % 2 != 0
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
        defuse_button_strip(bomb_info);
        return;
    }

    if button_text.trim() == "detonate" && bomb_info.battery_count > 1
    {
        println!("Press and immediately release the button.");
        return;
    }

    if button_color.trim() == "white" && bomb_info.car
    {
        defuse_button_strip(bomb_info);
        return;
    }

    if bomb_info.battery_count > 2 && bomb_info.frq
    {
        println!("Press and immediately release the button.");
        return;
    }

    if button_color.trim() == "yellow"
    {
        defuse_button_strip(bomb_info);
        return;
    }

    if button_color.trim() == "red" && button_text.trim() == "hold"
    {
        println!("Press and immediately release the button.");
        return;
    }

    defuse_button_strip(bomb_info);
    return;
}


pub fn defuse_button_strip(bomb_info: Bomb)
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
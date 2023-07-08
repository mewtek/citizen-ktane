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

pub fn defuse_button(button_color: String, button_text: String, bomb_info: Bomb)
{
    // TODO:
    return;
}

pub fn defuse_keypads(symbols: Vec<&str>)
{
    // TODO:
    return;
}
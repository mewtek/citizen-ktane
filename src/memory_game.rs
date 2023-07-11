use std::{io};
pub struct MemoryGameResult {
    button_pressed_label: i32,
    button_pressed_position: i32
}


pub fn stage_one()
{
    println!("# MEMORY GAME - STAGE 1 #");
    let mut sequence: Vec<i32> = Vec::new();    // First element of this vector should always be the number on the display
    let mut sequence_string = String::new();    // For splitting
    let mut button_pressed_label: i32 = 0;
    let mut button_pressed_position: i32 = 0;

    io::stdin().read_line(&mut sequence_string).unwrap();

    let sequence_split: Vec<&str> = sequence_string.split(',').collect();
    for i in sequence_split
    {
        let x = i.trim().parse::<i32>().unwrap();
        sequence.push(x);
    }

    match sequence[0]
    {
        1=>{println!("Press the button in the second position."); button_pressed_label = sequence[2]; button_pressed_position = 2;},
        2=>{println!("Press the button in the second position."); button_pressed_label = sequence[2]; button_pressed_position = 2;},
        3=>{println!("Press the button in the third position."); button_pressed_label = sequence[3]; button_pressed_position = 3;},
        4=>{println!("Press the button in the fourth position."); button_pressed_label = sequence[4]; button_pressed_position = 4;},
        _=>return   // This should never be hit.
    }

    let result = MemoryGameResult {button_pressed_label,button_pressed_position};
    stage_two(result);
}

pub fn stage_two(stage_one_result: MemoryGameResult)
{
    println!("# MEMORY GAME - STAGE 2 #");
    let mut sequence: Vec<i32> = Vec::new();
    let mut sequence_string = String::new();
    let mut button_pressed_label: i32 = 0;
    let mut button_pressed_position: i32 = 0;

    io::stdin().read_line(&mut sequence_string).unwrap();

    let sequence_split: Vec<&str> = sequence_string.split(',').collect();
    for i in sequence_split
    {
        let x = i.trim().parse::<i32>().unwrap();
        sequence.push(x);
    }

    match sequence[0]
    {
        1=> {
            println!("Press the button labeled 4.");
            button_pressed_label = 4;
            button_pressed_position = sequence.iter().position(|&n| n == 4).unwrap() as i32;    // 4 isn't in a fixed position
        }

        2 | 4=> {
            println!("Press {}", sequence[stage_one_result.button_pressed_position as usize]);
            button_pressed_label = sequence[stage_one_result.button_pressed_position as usize];
            button_pressed_position = stage_one_result.button_pressed_position;
        }

        3=> {
            println!("Press {}", sequence[1]);
            button_pressed_label = sequence[1];
            button_pressed_position = 1;
        }

        _=>return
    }

    let result = MemoryGameResult{button_pressed_label, button_pressed_position};
    stage_three(stage_one_result, result)
}

pub fn stage_three(stage_one_result: MemoryGameResult, stage_two_result: MemoryGameResult)
{
    println!("# MEMORY GAME - STAGE 3 #");
    let mut sequence: Vec<i32> = Vec::new();
    let mut sequence_string = String::new();
    let mut button_pressed_label = 0;
    let mut button_pressed_position = 0;
    
    io::stdin().read_line(&mut sequence_string).unwrap();

    let sequence_split: Vec<&str> = sequence_string.split(',').collect();
    for i in sequence_split
    {
        let x = i.trim().parse::<i32>().unwrap();
        sequence.push(x);
    }

    match sequence[0]
    {
        1 => {
            println!("Press {}", stage_two_result.button_pressed_label);
            button_pressed_label = stage_two_result.button_pressed_label;
            button_pressed_position = sequence.iter().position(|&n| n == stage_two_result.button_pressed_label).unwrap() as i32;
        }

        2 => {
            println!("Press {}", stage_one_result.button_pressed_label);
            button_pressed_label = stage_one_result.button_pressed_label;
            button_pressed_position = sequence.iter().position(|&n| n == stage_one_result.button_pressed_label).unwrap() as i32;
        }

        3 => {
            println!("Press {}", sequence[3]);
            button_pressed_label = sequence[3];
            button_pressed_position = 3;
        }

        4 => {
            println!("Press the button labeled 4.");
            button_pressed_label = 4;
            button_pressed_position = sequence.iter().position(|&n| n == 4).unwrap() as i32;
        }

        _=>return
    }

    let result = MemoryGameResult{button_pressed_label, button_pressed_position};
    stage_four(stage_one_result, stage_two_result, result)
}

pub fn stage_four(stage_one_result: MemoryGameResult, stage_two_result: MemoryGameResult, stage_three_result: MemoryGameResult)
{
    println!("# MEMORY GAME - STAGE 4 #");
    let mut sequence: Vec<i32> = Vec::new();
    let mut sequence_string = String::new();
    let mut button_pressed_label = 0;
    let mut button_pressed_position = 0;

    io::stdin().read_line(&mut sequence_string).unwrap();

    let sequence_split: Vec<&str> = sequence_string.split(',').collect();
    for i in sequence_split
    {
        let x = i.trim().parse::<i32>().unwrap();
        sequence.push(x);
    }

    match sequence[0]
    {
        1 => {
            println!("Press {}", sequence[stage_one_result.button_pressed_position as usize]);
            button_pressed_label = sequence[stage_one_result.button_pressed_position as usize];
            button_pressed_position = stage_one_result.button_pressed_position;
        }

        2 => {
            println!("Press {}", sequence[1]);
            button_pressed_label = sequence[1];
            button_pressed_position = 1;
        }

        3 | 4 => {
            println!("Press {}", sequence[stage_two_result.button_pressed_position as usize]);
            button_pressed_label = sequence[stage_two_result.button_pressed_position as usize];
            button_pressed_position = stage_two_result.button_pressed_position;
        }

        _=>return
    }

    let result = MemoryGameResult{button_pressed_label, button_pressed_position};
    stage_five(stage_one_result, stage_two_result, stage_three_result, result);
}

pub fn stage_five(stage_one_result: MemoryGameResult, stage_two_result: MemoryGameResult, stage_three_result: MemoryGameResult, stage_four_result:MemoryGameResult)
{
    println!("# MEMORY GAME - STAGE 5 #");
    let mut sequence: Vec<i32> = Vec::new();
    let mut sequence_string = String::new();
    
    io::stdin().read_line(&mut sequence_string).unwrap();

    let sequence_split: Vec<&str> = sequence_string.split(',').collect();
    for i in sequence_split
    {
        let x = i.trim().parse::<i32>().unwrap();
        sequence.push(x);
    }

    match sequence[0]
    {
        1 => println!("Press {}", stage_one_result.button_pressed_label),
        2 => println!("Press {}", stage_two_result.button_pressed_label),
        3 => println!("Press {}", stage_four_result.button_pressed_label),
        4 => println!("Press {}", stage_three_result.button_pressed_label),
        _ => return
    }

    return;
}
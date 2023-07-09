use std::io;
mod solvers;

fn main() {
    // Bomb setup
    // TODO: This needs to use Google speech api or something, typing this all out is actually
    // completely disruptive to the actual game.

    let mut serial = String::new();
    let mut car = String::new();
    let mut frk = String::new();
    let mut batteries: String = String::new();

    println!("BOMB SETUP.");
    println!("Bomb serial number: ");
    io::stdin().read_line(&mut serial).unwrap();
    println!("Is there a label that says \"CAR\"?");
    io::stdin().read_line(&mut car).unwrap();
    println!("Is there a label that says \"FRQ\"?");
    io::stdin().read_line(&mut frk).unwrap();
    println!("How many batteries are on the bomb?");
    io::stdin().read_line(&mut batteries).unwrap();

    println!("SERIAL: {}\nCAR: {}\nFRQ: {}\nBATTERY COUNT: {}", serial, car, frk, batteries);

    let mut yn: String = String::new();
    println!("Does this look right?");
    io::stdin().read_line(&mut yn).unwrap();

    if yn.trim() == "y" || yn.trim() == "yes"
    {

        let mut car_bool = false;
        let mut frk_bool = false;
        let serial_trimmed = serial.trim().to_string();
        println!("{batteries}");
        let batteries_int: i32 = batteries.trim().parse().unwrap_or(0);
        println!("{batteries_int}");

        if car.trim().to_lowercase() == "y" || car.trim().to_lowercase() == "yes"
        {
            car_bool = true;
        }

        if frk.trim().to_lowercase() == "y" || frk.trim().to_lowercase() == "yes"
        {
            frk_bool = true;
        }
        

        let bomb = solvers::Bomb
        {
            serial: serial_trimmed,
            car: car_bool,
            frq: frk_bool,
            battery_count: batteries_int,
            strikes: 0
        };

        println!("Nice!");
        choose_defuse(bomb);
    }

    println!("Whoops, let's start again.");

}

fn choose_defuse(bomb_info: solvers::Bomb)
{
    let mut defuse_type = String::new();

    println!("What kind of module are we defusing?");
    io::stdin().read_line(&mut defuse_type).unwrap();

    if defuse_type.trim() == "wires"
    {
        let mut wire_sequence_string = String::new();
        println!("Sequence of wires seperated by commas:");
        io::stdin().read_line(&mut wire_sequence_string).unwrap();
        
        let wire_sequence = wire_sequence_string.split(',').collect::<Vec<&str>>();

        solvers::defuse_wires(wire_sequence, bomb_info);
        return;
    }

    if defuse_type.trim() == "button"
    {
        solvers::defuse_button(bomb_info);
        return;
    }

    if defuse_type.trim() == "simon" || defuse_type.trim() == "simon says"
    {
        solvers::defuse_simon_says(bomb_info);
        return;
    }
}

/// Checks to see if the last digit of the bomb's serial number is odd.
pub fn last_digit_odd(serial: String) -> bool
{
    let last_digit_string = serial.chars().last().unwrap().to_string();
    let last_digit: i32 = last_digit_string.parse::<i32>().unwrap_or(0);

    if last_digit % 2 == 0
    {
        return true;
    }

    return false;
}

/// Checks to see if the bomb's serial number has a vowel or more in it.
pub fn serial_has_vowels(serial: String) -> bool
{
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let serial_chars: Vec<char> = serial.chars().collect();

    // This can probably be done without looping over the vowels vector but
    // im just gonna go ahead and do it like this for now
    for i in 0..vowels.len()
    {
        if serial_chars.contains(&vowels[i])
        {
            return true;
        }
    }

    return false;
}


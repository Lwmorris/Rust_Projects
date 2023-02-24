use std::collections::HashMap;
use std::{convert, io};

fn main() -> io::Result<()> {
    //read from stdin
    let lines = io::stdin().lines();
    for line in lines {
        let line = line?;
        //transform each line
        let morse_line = convert_morse(line);
        //print each line
        println!("{}", morse_line);
    }
    Ok(())
}
fn convert_morse(input: String) -> String {
    let mut morse_line: String = String::new();
    let morse_map = HashMap::from([
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        ('0', "-----"),
        (' ', "/"),
        ('\n', "\n"),
    ]);
    let mut insert: String = String::from("");
    for c in input.chars() {
        if let Some(morse_string) = &morse_map.get(&c.to_lowercase().next().unwrap()) {
            morse_line.push_str(morse_string);
            morse_line.push(' ');
        }
    }
    //loop over characters in input
    //look up morse code for character (if it exists)
    //add morse code to line
    morse_line
}

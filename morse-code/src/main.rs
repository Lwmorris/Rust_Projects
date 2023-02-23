use std::{io, convert};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    
    //read from stdin 
    let lines = io::stdin().lines();
    for line in lines {
        let line = line?;
        //transform each line
        let morseLine = convert_morse(line);
        //print each line
        println!("{}", morseLine);
    }
    Ok(()) 
}
fn convert_morse(input: String) -> String {
    let mut morseLine:String = String::new();
    let morseMap = HashMap::from([
        ('a',".-"),
        ('b',"-..."),
        ('c',"-.-."),
        ('d',"-.."),
        ('e',"."),
        ('f',"..-."),
        ('g',"--."),
        ('h',"...."),
        ('i',".."),
        ('j',".---"),
        ('k',"-.-"),
        ('l',".-.."),
        ('m',"--"),
        ('n',"-."),
        ('o',"---"),
        ('p',".--."),
        ('q',"--.-"),
        ('r',".-."),
        ('s',"..."),
        ('t',"-"),
        ('u',"..-"),
        ('v',"...-"),
        ('w',".--"),
        ('x',"-..-"),
        ('y',"-.--"),
        ('z',"--.."),
        ('1',".----"),
        ('2',"..---"),
        ('3',"...--"),
        ('4',"....-"),
        ('5',"....."),
        ('6',"-...."),
        ('7',"--..."),
        ('8',"---.."),
        ('9',"----."),
        ('0',"-----"),
        (' ',"/"),
        ('\n',"\n")
        ]);
        let mut insert:String = String::from("");
        for c in input.chars() {
            if (c.is_digit(36) || c.is_whitespace()) {
                let morseString:String = insert + &morseMap.get(&c.to_lowercase().to_string().chars().nth(0).unwrap()).unwrap().to_string();
                morseLine.push_str(morseString.as_str());
                insert = String::from(" ");
     
            }
        }
    //loop over characters in input 
    //look up morse code for character (if it exists)
    //add morse code to line
    morseLine
} 
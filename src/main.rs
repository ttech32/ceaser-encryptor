/* This program is used to encrypt a give file.
Author: Darkcrypt
Note: I was a learner when doing this so if there is any upgrade to be made please do*/

use std::env; //This is to collect the file name as an argument when running the code
use std::fs; //This library is used to handle file operations
use std::fs::File; //This also handles file operations
use std::io::Write;//This to to write to a file being created

//The Main Function
fn main() {
    //we will collect every argument passed as the program runs
    let args: Vec<String> = env::args().collect();

    //Let's pick the index [1] and store it
    let filename = &args[1];

    //We are reading the file and storing it in the contents variable
    let contents = fs::read_to_string(filename).expect("Unable to read text");

    //I need to convert all the letters in contents to capital letter cause ciphers are all capital
    let contents = contents.to_uppercase();
    
    //Calling the function implementing the ceaser's cipher
    ceaser_cipher(&contents);
}

//Kindly Write your functions below

//Now we need to write a function to encrypt the text implementing ceaser's cipher
fn ceaser_cipher(content: &str) {

    //Create an array of alphabets from A-Z
    let alphabet = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

    //An empty string will be created to store the cipher_text once converted  
    let mut cipher_text = String::new();

    //A loop will be created to access each letter and word in the content to begin Operation
    for line in content.lines(){
        for word in line.chars(){
            //An if statment to make sure its in alphabet
            if alphabet.contains(&word){
                //we need to get the array index of the letters found
                let arr_index = match word {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    'E' => 4,
                    'F' => 5,
                    'G' => 6,
                    'H' => 7,
                    'I' => 8,
                    'J' => 9,
                    'K' => 10,
                    'L' => 11,
                    'M' => 12,
                    'N' => 13,
                    'O' => 14,
                    'P' => 15,
                    'Q' => 16,
                    'R' => 17,
                    'S' => 18,
                    'T' => 19,
                    'U' => 20,
                    'V' => 21,
                    'W' => 22,
                    'X' => 23,
                    'Y' => 24,
                    'Z' => 25,
                    //For infinity matching
                    '\0'..='@' | '['..='\u{d7ff}' | '\u{e000}'..='\u{10ffff}' => todo!(),
                };

                //Using the formula we encrypt the letters we have gotten from the loop
                let enc_index = (arr_index + 3) % 26;

                //The encrypted words will be gotten from their various index after calculation
                let enc_word = alphabet[enc_index];

                //Now we need to push the encrypted words to the cipher text created above
                cipher_text.push(enc_word);

                
            }
        }
    }//End of loop

    
    // Create a new file to store the cipher_text
    let file = File::create("cipher_message.txt");

    // Write content into the file
    let _ = file.expect("can't write into file").write_all(cipher_text.as_bytes());
}

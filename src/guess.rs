use rand::Rng;
use std::io;

pub fn ask_num() {
   let mut correct_ans = rand::thread_rng();
   
   //loop

   let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            println!("This integer");
            println!("is {}", i);
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}
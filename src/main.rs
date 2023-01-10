use std::io::stdin;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {    
    println!("Wellcom to Todo app built with Rust language!", );

    let file_path: String = String::from("./task_list.txt");
    // let file = File::open(file_path).unwrap();
    // let buffer = BufReader::new(file);

    // fn tasks_counter(file: &File) -> i32 {
    //     let buffer = BufReader::new(file);
    //     let mut counter: i32 = 0;
    //     for _ in buffer.lines() {
    //         counter +=1;
    //     }
    //     counter
    // }

    let mut user_input: String = String::new(); 

    loop {
        user_input.clear();
        stdin().read_line(&mut user_input).expect("Nothing was inserted");
        println!("{:?}", user_input);

        if user_input == "exit\n" {
            break;
        }

        if user_input == "list\n" {
            let file = File::open("./task_list.txt").unwrap();
            let buffer = BufReader::new(&file);
            for line in buffer.lines() {
                println!("{:?}", line.unwrap());
            }

            // println!("{:?}", buffer);
        }
    }     

    // let mut screen_pointer: i32 = 0;

    // for line in buffer.lines() {
    //     let task = line.unwrap();
    //     let _checked = task.as_bytes()[1];
    //     println!("> {:?}", task);
    //     // println!("{:?}", checked);
    // }

    // while screen_pointer < 

}

// scratch

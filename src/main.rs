use rand::Rng;
use square::*;
use std::io::{stdin, stdout, Write};

/*fn rand_vec(x: i32) -> Array2D<i64>{
  let mut rand = Array2D::filled_with(0, x, 2);
  let mut rng = rand::thread_rng();
  let mut vec = Vec::new();
  let mut vec2 = Vec::new();
  for y in 0..x{
    vec.push(rng.gen_range(0,4));
    vec2.push(rng.gen_range(0,4));
  }


}*/

fn prompt(x: usize, vec: &Vec<usize>, vec2: &Vec<usize>, y: i64) {
    let g = x - 1;
    let mut corr = false;
    while !corr {
        let mut inp = String::new();
        println!("What is the missing value of row {}?", x);
        let _ = stdin().read_line(&mut inp).expect("Invalid input");
        let inp: i64 = inp.trim().parse().expect("invalid input");

        if y == inp {
            println!("Correct!");
            corr = true;
        } else {
            println!("Sorry try again!");
        }
    }
}

fn five_ez() {
    let square = fill_five();
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    let mut vec2 = Vec::new();
    for x in 0..5 {
        vec.push(x);
        vec2.push(rng.gen_range(0, 4));
    }
    for x in 0..5 {
        for y in 0..5 {
            if square[(x, y)] == square[(vec[x], vec2[x])] {
                print!("{} ", "x");
            } else {
                print!("{} ", square[(x, y)]);
            }
        }
        println!();
    }
    println!("**********");

    prompt(1, &vec, &vec2, square[(vec[0], vec2[0])]);
    prompt(2, &vec, &vec2, square[(vec[1], vec2[1])]);
    prompt(3, &vec, &vec2, square[(vec[2], vec2[2])]);
    prompt(4, &vec, &vec2, square[(vec[3], vec2[3])]);
    prompt(5, &vec, &vec2, square[(vec[4], vec2[4])]);
    println!("**********");
    println!("Here is the complete square!");
    for x in 0..5 {
        for y in 0..5 {
            print!("{} ", square[(x, y)]);
        }
        println!();
    }
    println!("**********");
}

fn menu_loop() {
    let mut cont = true;
    while cont {
        let mut inp = String::new();
        println!("Select game mode");
        println!("5x5 - ez 1:");
        println!("5x5 - med 2:");
        println!("5x5 - hard 3:");
        println!("quit - 4:");
        println!();
        let _ = stdout().flush();
        stdin().read_line(&mut inp).expect("Invalid input");
        if let Some('\n') = inp.chars().next_back() {
            inp.pop();
        }
        if let Some('\r') = inp.chars().next_back() {
            inp.pop();
        }
        println!("**********");

        if inp == "1" {
            five_ez();
        }

        if inp == "4" {
            println!("goodbye!");
            cont = false;
        }
    }
}

fn main() {
    println!("Welcome to the magic square game");
    menu_loop();
}

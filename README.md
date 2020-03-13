# Magic Square console game
This is a console version of a simple fill in the missing number magic square game. Created by Cole Schwarz

## Build instructions
clone into desired folder
build with cargo build

## Run instructions
cargo run

## Testing 
a series of tests exists for the library crate square that can be run via cargo test

## Expectations

My initial plan for this project was to impliment the magic square game using the GKT-Rust gui bindings.
After struggling with the GTK bindings dependancies and documentation my efforts seems to be leading in 
the wrong direction. I then reverted to making a simple command line game with a menu loop to save the gui
idea for another time. I would have liked to make the gui work but and intend to continue to work on that
portion of the project in a future date. Instead I focused on the square crate and the ability to produce several 
variations of magic squares.

Two seperate algorithms for both 5x5 and the 7x7 squares were used. The "hard" difficulty
is a randomized version of the magic square based on the start location of the seeding of the square values. The "ez"
version used a fix starting location. I was also able to randomize the "missing" portion of the square for the 
game to make sure that each generated game is slightly different even on the "ez" mode.

I am satisfied with the square lib crate and the functionaliy of it but as far as the console app it leaves much
to be desired and future itterations will correct formatting issues and do better checks for invalid input.

use std::io;
mod game;

fn main() {
    println!("-- Welcome to Tic_Tac_Toe -- ");

    println!(": Enter 'Help' for commands");

    loop {
        
        let mut command = String::new();

        io::stdin()
        .read_line(&mut command)
        .expect("Failed to read lin");

        let command = command.trim().to_lowercase();

        match command.as_str() {
            "help" => println!("
            start: Starts game
            q:     Quits game
            help:  lists commands
            "),
            "start" => {
                game::game();
                println!(": Good Game!")
            },
            "q" => { println!(": Good Bye");  break; },
            _ => println!(":'{}' is not a command. \n: Pleas enter 'Help' to see the list of commands", command)

        };

    }
}

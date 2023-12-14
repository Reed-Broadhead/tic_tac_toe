use std::io;
use std::collections::HashSet;

pub fn game() {

    let mut board: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut _res: &str = "";

    println!("lets start");

    loop {
        let mut command = String::new(); 

        display(&mut board);

        println!("-enter number:");

        io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

        let num: i32 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("(input err): Please enter a valid number!");
                return;
            },
        };
        
        let position: usize =  board.iter().position(|&x| x == num.to_string()).unwrap(); 

        board[position] = "x";

        // Draw
        if !board.iter().any(|i| i != &"x" && i != &"y"){
            println!("Draw"); display(&mut board); break
        }
        // Player wins
        if check_complete(board) == -1 {
            println!("You Win"); display(&mut board); break
        }

        // ai turn
        ai(&mut board);
    
        if check_complete(board) == 1{
            println!("You Lose!");
            display(&mut board);
            break
        };   
    
    }    


}

pub fn display (board: &mut [&str; 9]) {
    let mut output = String::new(); 
    for el in 0..board.len() {
        if (el + 1) % 3 == 0{
            output = output + " | " + board[el] + " | "+ "\n"
        } else{
            output = output +  " | " + board[el] 
        }

    };
    println!("{}", output);
}

pub fn ai(board: &mut [&str; 9]) {

    let mut c: [&str; 9] = board.clone();

    let best: (i32, i32) = minimax( &mut c, true);

    if best.1 > -1 {
    board[best.1 as usize] = "y";
    } else {
        board[board.iter().position(|&i| i == "").unwrap()] = "y";
    }
    // println!("{:?}", best);


    }

pub fn minimax(board: &mut [&str; 9], is_maximizing: bool) -> (i32, i32) {

    let score = check_complete(*board);
    
    if score != 0 {
        return (score, -1);
    }
    
    let turn: &str = if is_maximizing{"y"} else {"x"}; 
    
    let mut best_move: i32 = -1;
    let mut best_score: i32 = if is_maximizing { i32::MIN } else { i32::MAX };
    
    for (i,  &el) in board.iter().enumerate() {

        if el != "x" && el != "y"{

            let mut new_board: [&str; 9] = board.clone();

            new_board[i] = turn;

            let (score, _) = minimax(&mut new_board, !is_maximizing);
         

            if is_maximizing && score > best_score || !is_maximizing && score < best_score{
                best_score = score;
                best_move = i as i32;
            }

            if best_score == 1 && is_maximizing || best_score == -1 && !is_maximizing{
                return(best_score, best_move)
            }
        }

    };

    if best_move == -1 {
        return (0, -1);
    }

    (best_score, best_move)

}

fn check_complete(board: [&str; 9]) -> i32 {
    let mut res: i32 = 0;
    
    for el  in 0..3{
        let horizontal: HashSet<_> = board[el*3..el*3+3].iter().cloned().collect();

        let vertical: HashSet<_> = HashSet::from([board[el], board[el + 3], board[el + 6]]);

        let diagonal: HashSet<_> =  if el == 0{
            HashSet::from([board[el], board[el + 4], board[el + 8]])
        } else if el == 2 {
            HashSet::from([board[6], board[4], board[2]])
        } else {
            HashSet::from(["", "0"])
        };

        for el in [horizontal, vertical, diagonal] {
            if el.len() == 1{
                
                // res = 
                match el.iter().next().expect("Set was empty"){
                    &"x" => {res = -1},
                    &"y" => {res = 1},
                    // &"" => {res = 0},
                    _ => {res=0}
                }
                break;
            }
        }
    };
    res
}
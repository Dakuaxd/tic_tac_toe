use std::io;

fn main() {
    let mut grid: [[usize; 3]; 3] = [[0; 3]; 3];
    let mut is_pX_turn: bool = true;
    let game_over_state: usize;
    let bot_difficulty: usize;
    // indexing like this: grid[1][2]

    let (is_two_player, bot_difficulty, is_go_first): (bool, usize, bool) = intro();

    if !is_two_player && !is_go_first {
        is_pX_turn = false;

        bot::bot_move(&mut grid, is_go_first, bot_difficulty);
    }

    loop {
        game::print_grid(&grid);

        game::player_move(is_pX_turn, &mut grid);

        if game::is_end(&grid).0 {
            game_over_state = game::is_end(&grid).1;
            break;
        }

        if is_two_player {
            is_pX_turn = !is_pX_turn;
        }
        else {
            bot::bot_move(&mut grid, is_go_first, bot_difficulty);
        }

        if game::is_end(&grid).0 {
            game_over_state = game::is_end(&grid).1;
            break;
        }
    }

    game::announce_result(game_over_state);
    game::print_grid(&grid);
}

fn intro() -> (bool, usize, bool) {
    println!("");
    println!("This is tic tac toe");
    println!("X always goes first");
    println!("");
    println!("Would you like to play local multiplayer or against a bot?");
    println!("");

    let mut yes_no: String;
    let is_go_first: bool;
    let mut difficulty = String::new();
    let diff_num: usize;

    println!("Type y for multiplayer and n for a bot");
    println!("");

    loop {
        yes_no = String::new();

        io::stdin().read_line(&mut yes_no)
            .expect("Didn't read anything");

        yes_no = game::fix_input(yes_no);
        let y_n: &str = &yes_no[..1];

        if y_n != "y" && y_n != "n" {
            println!("");
            println!("That wasn't what I asked for!");
            println!("Gimme a correct input.");
            println!("");
        }
        else if y_n == "y" {
            println!("");
            println!("ok");
            println!("");
            return (true, 0, false); // second one doesn't matter
        }
        else if y_n == "n" {
            println!("");
            println!("ok");
            println!("");
            break;
        }
        else { panic!("jlksjflkdsjlkfsdj"); }
    }

    println!("");
    println!("And what difficulty would you like the bot to be?");
    println!("0 (easy), 1 (medium), 2 (unbeatable)");

    loop {
        difficulty = String::new();

        io::stdin().read_line(&mut difficulty)
            .expect("Didn't read anything");

        difficulty = game::fix_input(difficulty);
        let d: &str = &difficulty[..1];

        if d != "0" && d != "1" && d != "2" {
            println!("");
            println!("That wasn't what I asked for!");
            println!("Gimme a correct input.");
            println!("");
        }
        else if d == "0" {
            println!("");
            println!("alright, easy set");
            println!("");
            diff_num = 0;
            break;
        }
        else if d == "1" {
            println!("");
            println!("alright, medium set");
            println!("");
            diff_num = 1;
            break;
        }
        else if d == "2" {
            println!("");
            println!("alright, hard set");
            println!("");
            diff_num = 2;
            break;
        }
        else { panic!("kkkkkkkkkkkfaskfsdj"); }
    }

    println!("");
    println!("Do you want to go first (y) or second (n)");

    loop {
        yes_no = String::new();

        io::stdin().read_line(&mut yes_no)
            .expect("Didn't read anything");

        yes_no = game::fix_input(yes_no);
        let y_n: &str = &yes_no[..1];

        if y_n != "y" && y_n != "n" {
            println!("");
            println!("That wasn't what I asked for!");
            println!("Gimme a correct input.");
            println!("");
        }
        else if y_n == "y" {
            println!("");
            println!("ok");
            println!("");
            is_go_first = true;
            break;
        }
        else if y_n == "n" {
            println!("");
            println!("ok");
            println!("");
            is_go_first = false;
            break;
        }
        else { panic!("asdfkljskdfjsdkljfld"); }
    }

    return (false, diff_num, is_go_first);
}

mod game {
    use std::io;

    pub fn print_grid(grid: &[[usize; 3]; 3]) -> () {
        let mut g: [[char; 3]; 3] = [[' '; 3]; 3];

        g[0][0] = 'T';

        for c in 0..3 { for r in 0..3 {
            g[c][r] = match grid[c][r] {
                0 => ' ',
                1 => 'O',
                2 => 'X',
                _ => panic!("aiughbaoiynaoihna"),
            }
        }}

        println!("");
        println!("    A   B   C", );
        println!("1   {} | {} | {} ", g[0][0], g[1][0], g[2][0]);
        println!("2   {} | {} | {} ", g[0][1], g[1][1], g[2][1]);
        println!("3   {} | {} | {} ", g[0][2], g[1][2], g[2][2]);
        println!("");
    }

    pub fn player_move(is_pX_turn: bool, grid: &mut [[usize; 3]; 3]) -> () {
        let player_letter: char;

        if is_pX_turn { player_letter = 'X'; }
        else { player_letter = 'O'; }

        println!("");
        println!("It's {}'s turn, please input the grid space you wish to place your letter.", player_letter);
        println!("Letter first, number second. (b2, a3 etc...)");
        println!("");

        let mut user_move = String::new();

        loop {
            user_move = String::new();

            io::stdin().read_line(&mut user_move)
            .expect("Didn't read anything");

            user_move = fix_input(user_move);

            if user_move.len() != 2 {
                println!("");
                println!("Hey... that's not the right type of input.");
                println!("Make sure it's like: b2, a3, etc...");
                println!("Try again >:(");
                println!("");

                user_move = String::new();

                continue;
            }
            else { // So far good
                // let input1 = &user_move[..1];
                // let input2 = &user_move[1..];

                let move1 = match &user_move[..1] {
                    "a" => 0,
                    "b" => 1,
                    "c" => 2,
                     _  => 3,
                };
                let move2 = match &user_move[1..] {
                    "1" => 0,
                    "2" => 1,
                    "3" => 2,
                     _  => 3,
                };

                if move1 == 3 || move2 == 3 {
                    println!("");
                    println!("Hey... that's not the right type of input.");
                    println!("Make sure it's like: b2, a3, etc...");
                    println!("Try again >:(");
                    println!("");
                    continue;
                }
                else if grid[move1][move2] == 0 { // yep, worked
                    break;
                }
                else if grid[move1][move2] != 0 {
                    println!("");
                    println!("That tile is already taken");
                    println!("Choose another position");
                    println!("");

                    continue;
                }
                else { panic!(":("); }
            }
        }

        // let user_move: String = "b2";

        let column: &str = &user_move[0..1]; // "b"
        let row = &user_move[1..];           // "2"

        let r = match row {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            _ => panic!("row"),
        };

        // let mut r; // THIS IS THE SAME THING
        //
        // if row == 1 {
        //     r = 0
        // }
        // else if row == 2 {println!("");
        //     r = 1
        // }
        // else if row == 3 {
        //     r = 2
        // }
        // else { panic!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") }

        let c = match column {
            "a" => 0,
            "b" => 1,
            "c" => 2,
             _  => panic!("column"),
        };

        if is_pX_turn {
            grid[c][r] = 2;
        }
        else if !is_pX_turn {
            grid[c][r] = 1;
        }
        else { panic!("aaaaaaaaaaaaaa"); }
    }

    pub fn is_end(grid: &[[usize; 3]; 3]) -> (bool, usize) {
        let g = grid;

        for c in 0..3 { // TODO:check if 2 or 3
            if g[c][0] == g[c][1] && g[c][1] == g[c][2] && g[c][1] != 0 {
                return (true, g[c][0]);
            }
        }
        for r in 0..3 {
            if g[0][r] == g[1][r] && g[1][r] == g[2][r] && g[2][r] != 0 {
                return (true, g[0][r]);
            }
        }

        if (grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[1][1] != 0)
        || (grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] && grid[2][0] != 0) {
            return (true, g[1][1]);
        }

        let mut grid_sum: usize = 0;
        let add = |sum, new| sum + new;
        // add(sum: usize, new: usize) -> usize {
        //     sum + new
        // }
        for c in 0..3 { for r in 0..3 {
            grid_sum = add(grid_sum, g[c][r]);
        }}

        if grid_sum == 14 {
            return (true, 3);
        }

        // if (grid[0][0] == grid[0][1] == grid[0][2] != 0)
        // || (grid[1][0] == grid[1][1] == grid[1][2] != 0)
        // || (grid[2][0] == grid[2][1] == grid[2][2] != 0)
        // || (grid[0][0] == grid[1][0] == grid[2][0] != 0)
        // || (grid[0][1] == grid[1][1] == grid[2][1] != 0)
        // || (grid[0][2] == grid[1][2] == grid[2][2] != 0)
        // || (grid[0][0] == grid[1][1] == grid[2][2] != 0)
        // || (grid[0][2] == grid[1][1] == grid[2][0] != 0) {
        //
        //     return (true, game_over_state);
        // }


        else {
            (false, 0)
        }
    }

    pub fn announce_result(game_over_state: usize) -> () { //don't need a mut or an "&"? I found it wont spit an error but also wont change anything
        if game_over_state == 3 {
            println!(""); //printing is busted
            println!("Yikes, what a tie!");
            println!("GGWP");
        }
        else if game_over_state == 1 {
            println!("");
            println!("O wins. Nice!");
        }
        else if game_over_state == 2 {
            println!("");
            println!("X wins. Nice!");
        }
        else { panic!("AAkljdlkajfdsajaskljflsdka")}

        println!("");
        println!("Here's the final board:");
    }

    pub fn fix_input(s: String) -> String {
        let new_s: &str = &s[..(s.len() - 2)];
        // "string  " will become "string"

        String::from(new_s)
    }
}

mod bot {
    extern crate rand;
    use rand::prelude::*;

    pub fn bot_move(g: &mut [[usize; 3]; 3], is_go_first: bool, bot_difficulty: usize) -> () {
        let bot_letter: usize;

        if is_go_first {
            bot_letter = 1;
        }
        else {
            bot_letter = 2;
        }

        let (c, r) = match bot_difficulty {
            0 => easy(&g),
            1 => medium(&g),
            2 => hard(&g, bot_letter),
            _ => panic!("opusaiufdusaulfs")
        };

        g[c][r] = bot_letter;
    }

    fn rand(min: usize, max_plus_one: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max_plus_one)
    }

    fn easy (g: &[[usize; 3]; 3]) -> (usize, usize) {
        loop {
            match rand(1,10) {
                1 => if g[0][0] == 0 { return (0,0); },
                2 => if g[0][1] == 0 { return (0,1); },
                3 => if g[0][2] == 0 { return (0,2); },
                4 => if g[1][0] == 0 { return (1,0); },
                5 => if g[1][1] == 0 { return (1,1); },
                6 => if g[1][2] == 0 { return (1,2); },
                7 => if g[2][0] == 0 { return (2,0); },
                8 => if g[2][1] == 0 { return (2,1); },
                9 => if g[2][2] == 0 { return (2,2); },
                _ => panic!("oogkanans")
            }
        }
    }

    fn medium(g: &[[usize; 3]; 3]) -> (usize, usize) {
        if g[1][1] == 0 {
            (1, 1)
        }
        else if g[0][0] == 0 || g[2][0] == 0 || g[0][2] == 0 || g[2][2] == 0 {
            loop {
                match rand(1, 5) {
                    1 => if g[0][0] == 0 { return (0,0); },
                    2 => if g[2][0] == 0 { return (2,0); },
                    3 => if g[0][2] == 0 { return (0,2); },
                    4 => if g[2][2] == 0 { return (2,2); },
                    _ => panic!("jgggggggggfj"),
                }
            }
        }
        else if g[1][0] == 0 || g[2][1] == 0 || g[0][1] == 0 || g[1][2] == 0 {
            loop {
                match rand(1, 5) {
                    1 => if g[1][0] == 0 { return (1,0); },
                    2 => if g[2][1] == 0 { return (2,1); },
                    3 => if g[0][1] == 0 { return (0,1); },
                    4 => if g[1][2] == 0 { return (1,2); },
                    _ => panic!("jsklfjdslkfj"),
                }
            }
        }
        else { panic!("pklfsjlfkjsdkjsdflk"); }
    }

    fn hard(g: &[[usize; 3]; 3], bot_letter: usize) -> (usize, usize) {
        let mut board_sum: usize = 0;
        let player_letter: usize = match bot_letter {
            1 => 2,
            2 => 1,
            _ => panic!("jflskjflksdjlfksdj")
        };

        for c in 0..3 { for r in 0..3 {
            board_sum += g[c][r];
        }}

        if board_sum < 3 {
            if g[1][1] == 0 { return (1, 1); }

            match rand(1, 5) {
                1 => if g[0][0] == 0 { return (0,0); },
                2 => if g[2][0] == 0 { return (2,0); },
                3 => if g[0][2] == 0 { return (0,2); },
                4 => if g[2][2] == 0 { return (2,2); },
                _ => panic!("panic on bot's first turn with middle tile already taken"),
            }
        }

        let is_near_win = is_two_unobstructed_tiles(&g, bot_letter);

        if is_near_win.0 {
            return (is_near_win.1, is_near_win.2);
        }

        let is_near_win = is_two_unobstructed_tiles(&g, player_letter);

        if is_near_win.0 {
            return (is_near_win.1, is_near_win.2);
        }

        medium(&g)
    }

    fn is_two_unobstructed_tiles(g: &[[usize; 3]; 3], player_letter: usize) -> (bool, usize, usize) {
        let (mut f, mut s, mut t): (usize, usize, usize);  // first, second, third

        'scan_columns: for c in 0..3 {
            f = g[c][0];
            s = g[c][1];
            t = g[c][2];

            if (f == s) && (f == player_letter) && (t == 0) { return (true, c, 2); }
            if (f == t) && (f == player_letter) && (s == 0) { return (true, c, 1); }
            if (s == t) && (s == player_letter) && (f == 0) { return (true, c, 0); }
        }

        'scan_row: for r in 0..3 {
            f = g[0][r];
            s = g[1][r];
            t = g[2][r];

            if (f == s) && (f == player_letter) && (t == 0) { return (true, 2, r); }
            if (f == t) && (f == player_letter) && (s == 0) { return (true, 1, r); }
            if (s == t) && (s == player_letter) && (f == 0) { return (true, 0, r); }
        }

        let mid_letter = g[1][1];

        if mid_letter == player_letter {
            if (g[0][0] == mid_letter) && (g[2][2] == 0) { return (true, 2, 2); }
            if (g[0][2] == mid_letter) && (g[2][0] == 0) { return (true, 2, 0); }
            if (g[2][2] == mid_letter) && (g[0][0] == 0) { return (true, 0, 0); }
            if (g[2][0] == mid_letter) && (g[0][2] == 0) { return (true, 0, 2); }
        }

        (false, 0, 0)
    }
}


// 0 == empty
// 1 == O
// 2 == X

// game_over_state
// 1 == O win
// 2 == X loss
// 3 == tie

// user_move = String::new();
//
// io::stdin().read_line(&mut user_move)
// .expect("Didn't read anything");


// Instant win
 //   | o |
 // o | x | o
 //   | o |

// tie
    // |   | o
    // | x |
    // |   |


//use std::io;

mod game;

fn game_loop(){

    let mut mcts_wins = 0;
    let mut draws = 0;
    let mut mcts_h_wins = 0;
    
    for _x in 0..100{
        let mut board = game::Board::default();
        
        while !board.is_game_over() {
            board.draw_board();
            if board.current_player == game::Player::Black{
                let list_of_moves = board.legal_moves(board.current_player);
                if list_of_moves.is_empty(){
                    board.change_player();
                    continue;
                }

                let coordinates = board.ai_move(board.current_player);
                let _ai_move = board.do_move(coordinates[0].x, coordinates[0].y);
            }else{
                let list_of_moves = board.legal_moves(board.current_player);
                if list_of_moves.is_empty(){
                    board.change_player();
                    continue;
                }

                let coordinates = board.ai_move_with_heuristics(board.current_player);
                let _ai_move_h = board.do_move(coordinates[0].x, coordinates[0].y);
            }
        }

        let final_score = board.get_score();
        
        if final_score[0] > final_score[1]{
            mcts_wins += 1;
        }
        if final_score[0] == final_score[1]{
            draws += 1;
        }
        if final_score[1] > final_score[0]{
            mcts_h_wins += 1;
        }
    }

    println!("MCTS wins: {}", mcts_wins);
    println!("Draws: {}", draws);
    println!("MCTS with heuristics wins: {}", mcts_h_wins);
    
}

fn main() {
    game_loop();
}
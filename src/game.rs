use rand::Rng;
//#[macro_use]
extern crate rand;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Black,
    White
}

type Position = Option<Player>;

#[derive(Debug)]
pub enum InvalidMove {
    Invalid,
    Filled
}

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

impl Coordinate {
    pub fn forward(self: &Self, dir: &Direction) -> Option<Coordinate> {
        match *dir {
            Direction::Down | Direction::Bottomleft | Direction::Bottomright if self.y == 0 => {
                return None;
                },
            Direction::Left | Direction::Topleft | Direction::Bottomleft if self.x == 0 => {
                return None;
                },
            _ => {}
        }

        let coordinates = match *dir {
            Direction::Up => {
                Coordinate {
                    x: self.x, y: self.y + 1
                    }
                },
            Direction::Down => {
                Coordinate {
                    x: self.x, y: self.y - 1
                    }
                },
            Direction::Right => {
                Coordinate {
                     x: self.x + 1, y: self.y 
                     } 
                },
            Direction::Left => {
                 Coordinate {
                     x: self.x - 1, y: self.y 
                     } 
                },
            Direction::Topleft => {
                 Coordinate {
                     x: self.x - 1, y: self.y + 1 
                     } 
                },
            Direction::Topright => {
                 Coordinate {
                     x: self.x + 1, y: self.y + 1 
                     } 
                },
            Direction::Bottomleft => {
                 Coordinate {
                     x: self.x - 1, y: self.y - 1 
                     } 
                },
            Direction::Bottomright => {
                 Coordinate {
                     x: self.x + 1, y: self.y - 1 
                     } 
                }
        };
        Some(coordinates)
    }
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    Topleft,
    Topright,
    Bottomleft,
    Bottomright
}

static ALL_DIRECTIONS: [Direction; 8] = [Direction::Up, Direction::Down, Direction::Right, Direction::Left,
         Direction::Topleft, Direction::Topright, Direction::Bottomleft, Direction::Bottomright];

#[derive(Clone, Debug)]
pub struct Board{
    pub current_player: Player,
    pub next_player: Player,
    board: [[Position; 8]; 8]
}


impl Board{
    pub fn default() -> Self {
        let mut board = Board{
            current_player: Player::Black,
            next_player: Player::White,
            board: [[None; 8]; 8]
        };
        //Set positions on the board
        board.set_coordinates(&Coordinate {x: 3, y: 4}, Player::Black);
        board.set_coordinates(&Coordinate {x: 4, y: 4}, Player::White);
        board.set_coordinates(&Coordinate {x: 3, y: 3}, Player::White);
        board.set_coordinates(&Coordinate {x: 4, y: 3}, Player::Black);
        board
    }
    
    //Draws the board after each turn
    pub fn draw_board(&self){
        let end = "";
        println!("  1 2 3 4 5 6 7 8");
        for (i, row) in self.board.iter().enumerate(){
            print!("{} {}", i+1, end);
            for cell in row{
                match *cell{
                    Some(Player::Black) => print!("● "),
                    Some(Player::White) => print!("○ "),
                    None => print!(". "),
                };
            }
            println!();
        }
    }
    
    //Switch players
    fn next_player(self: &Self, p : Player) -> Player {
        match p {
            Player::Black => Player::White,
            Player::White => Player::Black
        }
    }

    //Change current player
    pub fn change_player(self: &mut Self) {
        self.current_player = self.next_player(self.current_player);
    }

    //Gets a list of legal moves
    pub fn legal_moves(self: &Self, player: Player) -> Vec<Coordinate> {
        let mut move_list: Vec<Coordinate> = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                let coord = Coordinate{x, y};
                if self.is_move_legal(&coord, player).is_ok() {
                    move_list.push(coord)
                }
            }
        }
        move_list
    }

    //Checks if a move is legal or not
    fn is_move_legal(self: &Self, coord: &Coordinate, player: Player) -> Result<Vec<Coordinate>,InvalidMove> {
        if !self.check_bounds(coord) {
            println!("Sorry, this move is out of bounds!\n");
            return Err(InvalidMove::Invalid)
        }
        match self.board[coord.x][coord.y] {
            None => {
                let map: Vec<Coordinate> = ALL_DIRECTIONS.iter()
                    .flat_map(|ref d| self.trace_map(coord, &d, player))
                    .collect();
                if map.is_empty() {
                    Err(InvalidMove::Invalid)
                } 
                else {
                    Ok(map)
                }
            },
            _ => Err(InvalidMove::Filled)
        }
    }

    //Checks the bounds of the 8x8 Reversi board
    fn check_bounds(self: &Self, coordinates: &Coordinate) -> bool {
        coordinates.x < 8 && coordinates.y < 8
    }

    //Utility function for trace_map
    fn check_position_against_player(self: &Self, c: Option<Coordinate>, player: Player) -> bool {
        c.map(|c| self.check_bounds(&c) && self.get_coordinates(&c).map(|p| p == player).unwrap_or(false)).unwrap_or(false)
    }

    //Acquires all coordinates that are legally allowed to be played given the current state
    fn trace_map(self: &Self, coord: &Coordinate, dir: &Direction, player: Player) -> Vec<Coordinate> {
        let mut found = 0;
        let mut coord = coord.forward(&dir);
        let mut csgo: Vec<Coordinate> = Vec::new();
        while self.check_position_against_player(coord, self.next_player(player)) {
            csgo.push(coord.unwrap());
            found += 1;
            coord = coord.unwrap().forward(&dir);
        }
        if found > 0 && self.check_position_against_player(coord, player) {
            csgo
        } else {
            vec![]
        }
    }

    //Check if game is over
    pub fn is_game_over(self: &Self) -> bool {
        self.legal_moves(Player::Black).is_empty() &&
          self.legal_moves(Player::White).is_empty()
    }

    //ai_move with heuristics found from online strategic guides
    pub fn ai_move_with_heuristics(self: &mut Self, p: Player) -> Vec<Coordinate>{
        let mut move_list = self.legal_moves(p);
        let mut rng = rand::thread_rng();

        let mut len = move_list.len();
        while len > 1{
            let x = rng.gen_range(0, len);
            if !(move_list[x].x != 1 && move_list[x].x != 0 && move_list[x].x != 6 && move_list[x].x != 7 || move_list[x].x != 1 && move_list[x].y != 1 && move_list[x].x != 6 && move_list[x].y != 6 || move_list[x].y != 0 && move_list[x].y != 1 && move_list[x].y != 6 && move_list[x].y != 7){
                move_list.remove(x);
                len -= 1;
                continue;
            }
            if !(move_list[x].y != 0 && move_list[x].y != 7 || move_list[x].x != 0 && move_list[x].x != 7) {
                len -= 1;
                continue;
            }
            move_list.remove(x);
            len -= 1;
        }
        move_list
    }

    //Pure ai_move without heuristics
    pub fn ai_move(self: &mut Self, p: Player) -> Vec<Coordinate> {
        let mut move_list = self.legal_moves(p);
        let mut len = move_list.len();

        let mut rng = rand::thread_rng();
        while len > 1{
            let rand_num = rng.gen_range(0, len);
            move_list.remove(rand_num);
            len -= 1;
        }
        move_list
        
    } 

    //Function to complete the move given the x and y coordinates
    pub fn do_move(self: &mut Self, x: usize, y: usize) -> Result<Player,InvalidMove> {
        let coord = Coordinate {
            x, y
        };
        let legal_move = self.is_move_legal(&coord, self.current_player);
        match legal_move  {
          Ok(map) => {
            let player = self.current_player;
            self.set_coordinates(&coord, player);
            self.change_player();
            for coord in map {
                self.set_coordinates(&coord, player);
            }
            Ok(self.current_player)
          },
          Err(x) => Err(x)
        }
    }

    //Get position on board at given coordinates
    fn get_coordinates(self: &Self, coord: &Coordinate) -> Position {
        self.board[coord.x][coord.y]
    }

    //Set player on board at given coordinates
    fn set_coordinates(self: &mut Self, coord: &Coordinate, p: Player) {
        self.board[coord.x][coord.y] = Some(p);
    }
    
    //Gets the score for both players
    pub fn get_score(self: &Self) -> [i32; 2] {
        let mut score: [i32; 2] = [0, 0];
        for row in self.board.iter() {
            for cell in row.iter() {
                match *cell {
                    None => {},
                    Some(Player::Black) => {
                        score[0] += 1
                        },
                    Some(Player::White)   => {
                        score[1] += 1
                        }
                }
            }
        }
        if score[0] == 0 {
            score[1] = 64; 
            }
        if score[1] == 0 {
            score[0] = 64; 
            }
        score
    }
}
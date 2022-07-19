extern crate pancurses;
use pancurses::{endwin, initscr, noecho, Input};
// extern crate fps_clock;

use rand::Rng;
// use std::time::Instant;

const GAME_AREA_X: isize = 40;
const GAME_AREA_Y: isize = 40;
const STARTING_TAIL_LENGTH: isize = 10;
const TIMEOUT_TIME: i32 = 100;
const SNAKE_NODE_CHAR: char = 'o';
const SNAKE_HEAD_CHAR: char = 'X';
const BLANK_CHAR: char = ' ';
const WALL_CHAR: char = '#';
const APPLE_CHAR: char = 'a';

fn main() {
    // let mut time = Instant::now();

    let mut apple = Apple::new(
        rand::thread_rng().gen_range(2..=GAME_AREA_X - 2),
        rand::thread_rng().gen_range(2..=GAME_AREA_Y - 2),
    );

    let mut game_manager = Game {
        state: GameState::PLAYING,
        score: 0,
        input: InputStruct {
            vertical: 0,
            horizontal: 1,
        },
        apple: apple,
    };

    let mut tail_nodes = Vec::new();
    for index in 1..STARTING_TAIL_LENGTH {
        tail_nodes.push(SnakeNode::new(GAME_AREA_X / 2, (GAME_AREA_Y / 2) - index));
    }

    let mut snake: Snake = Snake::new(SnakeNode::new(GAME_AREA_X / 2, GAME_AREA_Y / 2), tail_nodes);

    'game_loop: loop {
        match game_manager.state {
            GameState::PLAYING => {
                // input
                game_input(&mut game_manager.input);
                // update
                game_update(&mut game_manager, &mut snake);
                // render
                game_render(&snake, &game_manager);

                // fps.tick();
            }
            GameState::PAUSED => {}
            GameState::OVER => {
                println!("GAME OVER! Final Score: {}", &game_manager.score);
                break 'game_loop;
            }
        }
    }
}

fn get_random_apple_location() {}

struct Apple {
    x: isize,
    y: isize,
}
impl Apple {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

struct SnakeNode {
    x: isize,
    y: isize,
}
impl SnakeNode {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

struct Snake {
    head: SnakeNode,
    nodes: Vec<SnakeNode>,
}
impl Snake {
    pub fn new(head: SnakeNode, nodes: Vec<SnakeNode>) -> Self {
        Self { head, nodes }
    }
}

struct Game {
    state: GameState,
    score: i32,
    input: InputStruct,
    apple: Apple,
}

enum GameState {
    PLAYING,
    PAUSED,
    OVER,
}

struct InputStruct {
    vertical: isize,
    horizontal: isize,
}

fn game_update(game_manager: &mut Game, snake: &mut Snake) {
    let input = &game_manager.input;
    for index in (0..snake.nodes.len()).rev() {
        if index == 0 {
            snake.nodes[index].x = snake.head.x;
            snake.nodes[index].y = snake.head.y;
        } else {
            snake.nodes[index].x = snake.nodes[index - 1].x;
            snake.nodes[index].y = snake.nodes[index - 1].y;
        }
    }

    snake.head.x -= input.vertical;
    snake.head.y += input.horizontal;

    for node in &snake.nodes {
        if (snake.head.x, snake.head.y) == (node.x, node.y) {
            // WE ATE OURSELVES!
            game_manager.state = GameState::OVER;
        }
    }

    match snake.head.x {
        0 | GAME_AREA_X => {
            game_manager.state = GameState::OVER;
        }
        _ => {}
    }
    match snake.head.y {
        0 | GAME_AREA_Y => {
            game_manager.state = GameState::OVER;
        }
        _ => {}
    }

    if (snake.head.x, snake.head.y) == (game_manager.apple.x, game_manager.apple.y) {
        eat_apple(snake, game_manager); // TODO: Has to be a better way of handling this, need to refactor so that I'm not passing these everywhere
    }
}

fn eat_apple(snake: &mut Snake, game_manager: &mut Game) {
    let tail_node = &snake.nodes[snake.nodes.len() - 1];
    snake.nodes.push(SnakeNode::new(tail_node.x, tail_node.y));
    (game_manager.apple.x, game_manager.apple.y) = (
        rand::thread_rng().gen_range(2..=GAME_AREA_X - 2),
        rand::thread_rng().gen_range(2..=GAME_AREA_Y - 2),
    );
    game_manager.score += 1;
}

fn game_render(snake: &Snake, game_manager: &Game) {
    // let window = initscr();
    // window.refresh();
    // Clear the screen/board
    // print!("\x1B[2J\x1B[1;1H");

    let window = initscr();
    window.refresh();
    window.clear();
    // noecho();
    // window.nodelay(true);
    // Construct 2d int array where:
    //   0 = blank space
    //   1 = edge space
    //   2 = snake body
    //   3 = snake head

    let mut game_board = [[0 as usize; GAME_AREA_X as usize]; GAME_AREA_Y as usize]; //[[usize; GAME_AREA_X]; GAME_AREA_Y];
    game_board[snake.head.x as usize][snake.head.y as usize] = 3;
    game_board[game_manager.apple.x as usize][game_manager.apple.y as usize] = 4;

    for node in &snake.nodes {
        game_board[node.x as usize][node.y as usize] = 2;
    }

    let (edge_pos_x, edge_pos_y) = (GAME_AREA_X as usize, GAME_AREA_Y as usize);
    for (i, row) in game_board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let mut val: usize = *col;
            if ([0, (&GAME_AREA_X - 1) as usize].contains(&i)
                || [0, (&GAME_AREA_Y - 1) as usize].contains(&j))
            {
                val = 1;
            }
            // match (i, j) {
            //     // edge
            //     (0, _) | (20, _) | (_, 0) | (_, 20) => {
            //         val = 1;
            //     }
            //     (_, _) => {}
            // }

            match val {
                0 => {
                    window.addch(BLANK_CHAR);
                }
                1 => {
                    window.addch(WALL_CHAR);
                }
                2 => {
                    window.addch(SNAKE_NODE_CHAR);
                }
                3 => {
                    window.addch(SNAKE_HEAD_CHAR);
                }
                4 => {
                    window.addch(APPLE_CHAR);
                }
                _ => {}
            }
        }
        window.addstr("\n");
    }
    endwin();
}

fn game_input(input: &mut InputStruct) {
    // input.horizontal = 0;
    // input.vertical = 0;
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();
    window.timeout(TIMEOUT_TIME);
    // window.nodelay(true);
    // loop {
    match window.getch() {
        Some(Input::Character(c)) => {
            match c {
                'w' | 'W' => {
                    input.vertical = 1;
                    input.horizontal = 0;
                }
                's' | 'S' => {
                    input.vertical = -1;
                    input.horizontal = 0;
                }
                'a' | 'A' => {
                    input.horizontal = -1;
                    input.vertical = 0;
                }
                'd' | 'D' => {
                    input.horizontal = 1;
                    input.vertical = 0;
                }
                // Other keys are ignored
                _ => {}
            }
            // window.addstr(&format!("{:?}", input.vertical.to_string()));
        }
        // Some(Input::KeyDC) => break,
        Some(input) => (),
        None => (),
        // }
    }
    endwin();
}

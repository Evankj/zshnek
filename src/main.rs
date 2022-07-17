extern crate pancurses;
use pancurses::{endwin, initscr, noecho, Input};

// use std::time::Instant;

const GAME_AREA_X: isize = 40;
const GAME_AREA_Y: isize = 40;

fn main() {
    // let mut time = Instant::now();

    let mut game_manager = Game {
        state: GameState::PLAYING,
        score: 0,
        input: InputStruct {
            vertical: 0,
            horizontal: 0,
        },
    };

    let mut tail_nodes = Vec::new();
    for index in 1..10 {
        tail_nodes.push(SnakeNode::new(GAME_AREA_X / 2, (GAME_AREA_Y / 2) - index));
    }

    let mut snake: Snake = Snake::new(SnakeNode::new(GAME_AREA_X / 2, GAME_AREA_Y / 2), tail_nodes);

    'game_loop: loop {
        match game_manager.state {
            GameState::PLAYING => {
                // input
                game_input(&mut game_manager.input);
                // update
                game_update(&game_manager.input, &mut snake);
                // render
                game_render(&snake);
            }
            GameState::PAUSED => {}
            GameState::OVER => {
                println!("GAME OVER! Final Score: {}", &game_manager.score);
                break 'game_loop;
            }
        }
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

fn game_update(input: &InputStruct, snake: &mut Snake) {
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
}

fn game_render(snake: &Snake) {
    // let window = initscr();
    // window.refresh();
    // Clear the screen/board
    // print!("\x1B[2J\x1B[1;1H");

    let window = initscr();
    window.refresh();
    window.clear();
    // Construct 2d int array where:
    //   0 = blank space
    //   1 = edge space
    //   2 = snake body
    //   3 = snake head

    let mut game_board = [[0 as usize; GAME_AREA_X as usize]; GAME_AREA_Y as usize]; //[[usize; GAME_AREA_X]; GAME_AREA_Y];
    let (head_pos_x, head_pos_y) = (snake.head.x, snake.head.y);
    game_board[head_pos_x as usize][head_pos_y as usize] = 3;

    for node in &snake.nodes {
        game_board[node.x as usize][node.y as usize] = 2;
    }

    for (i, row) in game_board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let mut val: usize = *col;
            if ([0, (GAME_AREA_X - 1) as usize].contains(&i)
                || [0, (GAME_AREA_Y - 1) as usize].contains(&j))
            {
                val = 1;
            }
            match val {
                0 => {
                    window.addch(' ');
                }
                1 => {
                    window.addch('X');
                }
                2 => {
                    window.addch('o');
                }
                3 => {
                    window.addch('O');
                }
                _ => {}
            }
        }
        window.addstr("\n");
    }
    endwin();
}

fn game_input(input: &mut InputStruct) {
    input.horizontal = 0;
    input.vertical = 0;
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();
    // loop {
    match window.getch() {
        Some(Input::Character(c)) => {
            match c {
                'w' | 'W' => {
                    input.vertical += 1;
                }
                's' | 'S' => {
                    input.vertical -= 1;
                }
                'a' | 'A' => {
                    input.horizontal -= 1;
                }
                'd' | 'D' => {
                    input.horizontal += 1;
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

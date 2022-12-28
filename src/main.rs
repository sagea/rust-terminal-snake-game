mod vector;
mod utils;
mod term;
mod ui;

#[macro_use]
extern crate impl_ops;

use term::{collect_events, key_press};

use termion::{raw::{IntoRawMode}, input::{MouseTerminal}, clear, cursor::{Goto, self}, event::{Event, Key}, terminal_size};
use tokio::sync::Mutex;
use ui::{text, button};
use utils::random_between;
use vector::Vector;
use std::{io::{Write, stdout}, thread::{sleep, self}, time::Duration, sync::{mpsc::{self, Receiver}, Arc}, rc::Rc, borrow::{BorrowMut, Borrow}, option::Iter};

#[derive(Debug, PartialEq)]
enum SnakeGameTickOutcome {
    Nothing,
    GameOver,
}

struct SnakeGame {
    snake: Vec<Vector>,
    food_pos: Option<Vector>,
    score: i32,
    size: Vector,
    snake_direction: Vector,
}

impl SnakeGame {
    fn new(size: Vector) -> Self {
        SnakeGame {
            size,
            snake: vec![v!(0, 0)],
            score: 0,
            food_pos: None,
            snake_direction: v!(0, 1),
        }
    }
    fn get_food_pos(&mut self) -> Vector {
        if self.food_pos == None {
            self.food_pos = Some(self.calculate_food_position());
        }
        self.food_pos.unwrap()
    }
    fn run_game_tick(&mut self, key_events: &Vec<Event>) -> SnakeGameTickOutcome {
        if key_press(key_events, &Key::Up) && self.snake_direction.y == 0 {
            self.snake_direction = v!(0, -1);
        } else if key_press(key_events, &Key::Down) && self.snake_direction.y == 0 {
            self.snake_direction = v!(0, 1);
        } else if key_press(key_events, &Key::Left) && self.snake_direction.x == 0 {
            self.snake_direction = v!(-1, 0);
        } else if key_press(key_events, &Key::Right) && self.snake_direction.x == 0 {
            self.snake_direction = v!(1, 0);
        }

        let food_pos = self.get_food_pos();
        let head_pos = self.snake.first().expect("Should always have at least one");
        let next_pos = head_pos + self.snake_direction;

        if next_pos.x < 0 || next_pos.y < 0 || next_pos.x > self.size.x || next_pos.y > self.size.y {
            return SnakeGameTickOutcome::GameOver;
        }

        if self.snake.contains(&next_pos) {
            return SnakeGameTickOutcome::GameOver;
        }

        let food_touched = next_pos == food_pos;
        if food_touched {
            self.score += 1;
            self.food_pos = None;

        }
        if food_touched == false {
            self.snake.pop();
        }
        self.snake.insert(0, next_pos);
        return SnakeGameTickOutcome::Nothing;
    }
    fn calculate_food_position(&mut self) -> Vector {
        loop {
            let pos = v!(random_between(0, self.size.x), random_between(0, self.size.y));
            let found = self.snake.iter().find(|item| item == &&pos);
            if found.is_none() {
                return pos;
            }
        }
    }
}


fn calculate_border_around(size: &Vector) -> Vec<(Vector, String)> {
    // todo: Fix this. I think it's incorrectly adding a border with one extra space based on a visual bug
    let inner_width = size.x;
    let inner_height = size.y;
    let outer_width = size.x + 2;
    let border_char = "*".to_string();
    let horizontal_border = std::iter::repeat("*").take(outer_width as usize + 1).collect::<String>();
    let mut deets = vec![];
    deets.push((v!(0, 0), horizontal_border.clone()));
    for i in 1..inner_height + 2 {
        deets.push((v!(0, i), border_char.clone()));
        deets.push((v!(inner_width + 2, i), border_char.clone()));
    }
    deets.push((v!(0, inner_height + 2), horizontal_border));
    deets
}


#[derive(Debug, PartialEq)]
enum GameScreen {
    Start,
    GameOver,
    Game,
}

#[tokio::main]
async fn main() {
    let stdin = term::offthread_stdin();
    let stdout_og = Arc::new(Mutex::new(MouseTerminal::from(stdout().into_raw_mode().unwrap())));
    let stdout = Arc::clone(&stdout_og);
    let game_size = v!(20, 10);
    let mut snake_game = SnakeGame::new(game_size);
    let mut screen = GameScreen::Start;

    loop {
        let mut stdout = stdout.lock().await;
        let term_size = Vector::from(terminal_size().unwrap());
        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", Goto(1, 1)).unwrap();
        let events = collect_events(&stdin);

        if key_press(&events, &Key::Ctrl('c')) {
            break;
        }

        if screen == GameScreen::Start {
            let center_pos = term_size / v!(2, 2);
            text(center_pos.set_y(5), "Rust Snake!".to_string(), |lines| write_many_at!(stdout, lines.iter()));
            button(
                v!(term_size.x / 2, 8),
                "Play".to_owned(),
                &events,
                || {
                    screen = GameScreen::Game;
                    snake_game = SnakeGame::new(game_size);
                },
                |lines| write_many_at!(stdout, lines.iter())
            );
            let mut should_cancel = false;
            button(
                v!(term_size.x / 2, 9),
                "Cancel".to_owned(),
                &events,
                || {
                    should_cancel = true;
                },
                |lines| write_many_at!(stdout, lines.iter())
            );
            if should_cancel {
                break;
            }
        } else if screen == GameScreen::Game {
            let game_offset = ((term_size - game_size) / v!(2, 2)).set_y(3);
            write_many_at!(
                stdout,
                game_offset - v!(1, 1),
                calculate_border_around(&snake_game.size).iter()
            );

            let result = snake_game.run_game_tick(&events);

            if result == SnakeGameTickOutcome::GameOver {
                screen = GameScreen::GameOver;
            } else {
                write_at!(stdout, v!(1, 1) + game_offset.set_y(0), "Score");
                write_at!(stdout, snake_game.score);
                write_at!(stdout, snake_game.get_food_pos() + game_offset, "§");
                write_many_at!(
                    stdout,
                    game_offset,
                    snake_game.snake
                        .iter()
                        .map(|pos| (pos, "▢".to_owned()))
                );
            }
        } else if screen == GameScreen::GameOver {
            let center_pos = term_size / v!(2, 2);
            text(center_pos.set_y(5), "Game Over".to_string(), |lines| write_many_at!(stdout, lines.iter()));
            text(center_pos.set_y(6), format!("Final Score: {}", snake_game.score), |lines| write_many_at!(stdout, lines.iter()));
            button(
                v!(term_size.x / 2, 8),
                "Play Again".to_owned(),
                &events,
                || {
                    screen = GameScreen::Game;
                    snake_game = SnakeGame::new(game_size);
                },
                |lines| write_many_at!(stdout, lines.iter())
            );
        }
        write!(stdout, "{}", cursor::Hide).unwrap();
        stdout.flush().unwrap();
        sleep(Duration::from_millis(200));
    }

    let mut stdout = stdout.lock().await;
    write!(stdout, "{}", cursor::Show).unwrap();
    stdout.flush().unwrap();
}

use crossterm::cursor::{MoveDown, MoveTo, Show};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{terminal::SetSize, ExecutableCommand};
use rand::Rng;
use std::io::stdout;
use std::io::Stdout;

pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
    snake: Snake,
    food: Option<Point>,
    /* snake: Snake,
    speed: u16,
    score: u16, */
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn transform(&self, direction: Direction, times: u16) -> Self {
        let times = times as i16;

        let transformation = match direction {
            Direction::Down => (0, times),
            Direction::Up => (0, -times),
            Direction::Left => (-times, 0),
            Direction::Right => (times, 0),
        };

        Self::new(
            Self::transform_value(self.x, transformation.0),
            Self::transform_value(self.y, transformation.1),
        )
    }

    fn transform_value(value: u16, by: i16) -> u16 {
        if by.is_negative() && by.abs() as u16 > value {
            panic!("Should not going negative: {} ,{}", value, by)
        }
        (value as i16 + by) as u16
    }
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
        }
    }
}

pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        /*     let opposite = direction.opposite();*/
        let body: Vec<Point> = (0..length)
            .into_iter()
            .map(|i| start.transform(direction.opposite(), i))
            .collect();

        Self {
            body,
            direction,
            digesting: false,
        }
    }
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size = size().unwrap();
        Self {
            stdout,
            original_terminal_size,
            width,
            height,
            snake: Snake::new(
                Point::new(width / 2, height / 2),
                5,
                match rand::thread_rng().gen_range(0..=4) {
                    0 => Direction::Up,
                    1 => Direction::Right,
                    2 => Direction::Down,
                    _ => Direction::Left,
                },
            ),
            food: None,
            /*  snake: Snake::new(
                Point::new(width / 2, height / 2),
                3,
                match rand::thread_rng().gen_range(0, 4) {
                    0 => Direction::Up,
                    1 => Direction::Right,
                    2 => Direction::Down,
                    _ => Direction::Left
                },
            ),
            speed: 0,
            score: 0, */
        }
    }

    fn restore_ui(&mut self) {
        let (cols, rows) = self.original_terminal_size;
        self.stdout
            .execute(SetSize(cols, rows))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Show)
            .unwrap()
            .execute(ResetColor)
            .unwrap();

        disable_raw_mode().unwrap();
    }

    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        execute!(
            stdout(),
            SetSize(self.width, self.height),
            Clear(ClearType::All),
        )
        .unwrap();
    }

    pub fn run(&mut self) {
        /* self.restore_ui(); */
        self.prepare_ui();
        self.render();
    }

    fn draw_borders(&mut self) {
        execute!(stdout(), SetBackgroundColor(Color::Red)).unwrap();

        for x in 0..self.width + 1 {
            execute!(
                stdout(),
                MoveTo(x, 0),
                Print('#'),
                MoveTo(x, self.height - 1),
                Print('#')
            )
            .unwrap();
        }

        for y in 0..self.height {
            execute!(
                stdout(),
                MoveTo(0, y),
                Print('-'),
                MoveTo(self.width, y),
                Print('-')
            )
            .unwrap();
        }
        /* self.stdout
        .execute(MoveTo(0, 0)).unwrap()
        .execute(Print("#")).unwrap()
        .execute(MoveTo(self.width + 1, self.height + 1)).unwrap()
        .execute(Print("#")).unwrap()
        .execute(MoveTo(self.width + 1, 0)).unwrap()
        .execute(Print("#")).unwrap()
        .execute(MoveTo(0, self.height + 1)).unwrap()
        .execute(Print("#")).unwrap(); */
    }

    fn draw_snake(&mut self) {
        /* let fg = SetForegroundColor(match self.speed % 3 {
            0 => Color::Green,
            1 => Color::Cyan,
            _ => Color::Yellow
        }); */
        execute!(
            stdout(),
            SetForegroundColor(Color::White),
            MoveTo(self.width / 2, self.height / 2),
        )
        .unwrap();

        let body_points = self.snake.body.clone();

        /* self.stdout
        .execute(MoveTo(7, 6)).unwrap(); */
    }

    fn draw_background(&mut self) {
        for x in 1..self.width {
            for y in 1..self.height {
                execute!(
                    stdout(),
                    MoveTo(x, y),
                    SetBackgroundColor(Color::Magenta),
                    Print(' ')
                )
                .unwrap();
            }
        }
    }

    fn draw_food(&mut self) {
        execute!(
            stdout(),
            SetForegroundColor(Color::White),
            MoveTo(4 + 1, 3 + 1),
            Print('?')
        )
        .unwrap();

        /* for food in self.food.iter() {
            execute!(
                stdout(),
                MoveTo(food.x + 1, food.y + 1),
                Print('?')
            ).unwrap();
        } */
    }

    fn render(&mut self) {
        self.draw_background();
        self.draw_borders();
        /*  self.draw_food(); */
        /* self.draw_snake(); */
    }
}

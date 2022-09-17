use crate::minesweeper;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::view::CannotFocus;
use cursive::views::Dialog;
use cursive::{Printer, Vec2, XY};
use std::time::Instant;

const BOMB: usize = 9;

pub struct BoardView {
    board: minesweeper::MinesweeperBoard,
    overlay: Vec<Vec<Cell>>,

    focused: XY<usize>,
    remaining_mines: isize,
    revealed: usize,

    start_time: Instant,
}

impl BoardView {
    pub fn new(board: &minesweeper::MinesweeperBoard) -> Self {
        Self {
            board: board.clone(),
            overlay: vec![vec![Cell::Unrevealed; board.size.x]; board.size.y],
            focused: XY::new(board.size.x / 2, board.size.y / 2),
            remaining_mines: board.remaining_mines as isize,
            revealed: 0,
            start_time: Instant::now(),
        }
    }

    fn get_cell(&self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {
        mouse_pos
            .checked_sub(offset)
            .map(|pos| pos.map_x(|x| x / 2))
            .and_then(|pos| {
                if pos.fits_in(self.board.size) {
                    Some(pos)
                } else {
                    None
                }
            })
    }

    fn flag(&mut self, pos: Vec2) -> EventResult {
        if pos.fits_in(self.board.size) {
            let new_cell = match self.overlay[pos.y][pos.x] {
                Cell::Unrevealed => {
                    if self.board.cells[pos.y][pos.x].is_mine {
                        self.board.remaining_mines -= 1;
                    }
                    self.remaining_mines -= 1;
                    Cell::Flagged
                }
                Cell::Flagged => {
                    if self.board.cells[pos.y][pos.x].is_mine {
                        self.board.remaining_mines += 1;
                    }
                    self.remaining_mines += 1;
                    Cell::Unrevealed
                }
                other => other,
            };
            self.overlay[pos.y][pos.x] = new_cell;
        }

        EventResult::Consumed(None)
    }

    fn reveal(&mut self, pos: Vec2) -> EventResult {
        if pos.fits_in(self.board.size) {
            if self.overlay[pos.y][pos.x] != Cell::Unrevealed {
                return EventResult::Consumed(None);
            }

            match self.board.cells[pos.y][pos.x] {
                minesweeper::Cell { is_mine: true, .. } => {
                    self.overlay[pos.y][pos.x] = Cell::Revealed(BOMB);

                    let mines_left = self.board.remaining_mines;

                    return EventResult::with_cb(move |s| {
                        s.add_layer(
                            Dialog::text(format!(
                                "YOU LOST! You had {} mine{} left to clear",
                                mines_left,
                                if mines_left == 1 { "" } else { "s" }
                            ))
                            .button("Ok", |s| {
                                s.pop_layer();
                                s.pop_layer();
                            }),
                        );
                    });
                }
                minesweeper::Cell {
                    is_mine: false,
                    surrounding_mines: n,
                } => {
                    self.overlay[pos.y][pos.x] = Cell::Revealed(n);
                    if self.revealed == 0 {
                        self.start_time = Instant::now();
                    }
                    self.revealed += 1;
                    if n == 0 {
                        for p in self.board.neighbours(pos) {
                            self.reveal(p);
                        }
                    }
                }
            }
        }

        if self.revealed == self.board.size.x * self.board.size.y - self.board.mines_count {
            let final_time = Instant::now().duration_since(self.start_time);

            return EventResult::with_cb(move |s| {
                s.add_layer(
                    Dialog::text(format!("YOU WON! It took you {:.3?}", final_time)).button(
                        "Ok",
                        |s| {
                            s.pop_layer();
                            s.pop_layer();
                        },
                    ),
                );
            });
        }

        EventResult::Consumed(None)
    }

    fn auto_reveal(&mut self, pos: Vec2) -> EventResult {
        if pos.fits_in(self.board.size) {
            if let Cell::Revealed(n) = self.overlay[pos.y][pos.x] {
                let neighbours = self.board.neighbours(pos);

                let tagged = neighbours
                    .iter()
                    .map(|p| self.overlay[p.y][p.x])
                    .filter(|&cell| cell == Cell::Flagged)
                    .count();

                if tagged != n {
                    return EventResult::Consumed(None);
                }

                for p in neighbours {
                    let result = self.reveal(p);
                    if result.has_callback() {
                        return result;
                    }
                }
            }
        }

        EventResult::Consumed(None)
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        for (y, rows) in self.overlay.iter().enumerate() {
            for (x, cell) in rows.iter().enumerate() {
                let text = match *cell {
                    Cell::Revealed(n) => {
                        ["  ", " 1", " 2", " 3", " 4", " 5", " 6", " 7", " 8", " X"][n]
                    }
                    Cell::Unrevealed => "[]",
                    Cell::Flagged => "##",
                };

                let (front_colour, back_colour) = if self.focused == XY::new(x, y) {
                    (
                        Color::Dark(BaseColor::Blue),
                        match cell {
                            Cell::Revealed(BOMB) => Color::Dark(BaseColor::Red),
                            _ => Color::Dark(BaseColor::Black),
                        },
                    )
                } else {
                    (
                        Color::Dark(BaseColor::Black),
                        match cell {
                            Cell::Revealed(n) => match n {
                                1 => Color::RgbLowRes(3, 5, 3),
                                2 => Color::RgbLowRes(5, 5, 3),
                                3 => Color::RgbLowRes(5, 4, 3),
                                4 => Color::RgbLowRes(5, 3, 3),
                                5 => Color::RgbLowRes(5, 2, 2),
                                6 => Color::RgbLowRes(5, 0, 1),
                                7 => Color::RgbLowRes(5, 0, 2),
                                8 => Color::RgbLowRes(5, 0, 3),
                                &BOMB => Color::Dark(BaseColor::Red),
                                _ => Color::Dark(BaseColor::White),
                            },
                            Cell::Unrevealed => Color::RgbLowRes(3, 3, 3),
                            Cell::Flagged => Color::RgbLowRes(4, 4, 2),
                        },
                    )
                };

                printer.with_color(ColorStyle::new(front_colour, back_colour), |printer| {
                    printer.print((x * 2, y), text)
                });
            }
        }

        printer.print(
            XY::new(0, self.board.size.y + 1),
            format!("Mines left: {}", self.remaining_mines).as_str(),
        );
    }

    fn required_size(&mut self, _: XY<usize>) -> XY<usize> {
        XY::new(self.board.size.x * 2, self.board.size.y + 2)
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Press(_),
            } => {
                if let Some(pos) = self.get_cell(position, offset) {
                    self.focused = pos;
                    return EventResult::Consumed(None);
                }
            }

            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Release(button),
            } => {
                if let Some(pos) = self.get_cell(position, offset) {
                    if self.focused == pos {
                        match button {
                            MouseButton::Left => return self.reveal(pos),
                            MouseButton::Right => {
                                return self.flag(pos);
                            }
                            MouseButton::Middle => {
                                return self.auto_reveal(pos);
                            }
                            _ => (),
                        }
                    }
                }
            }

            Event::Key(key) => match key {
                Key::Up => {
                    if self.focused.y > 0 {
                        self.focused.y -= 1;
                    }
                    return EventResult::Consumed(None);
                }
                Key::Down => {
                    if self.focused.y < self.board.size.y - 1 {
                        self.focused.y += 1;
                    }
                    return EventResult::Consumed(None);
                }
                Key::Left => {
                    if self.focused.x > 0 {
                        self.focused.x -= 1;
                    }
                    return EventResult::Consumed(None);
                }
                Key::Right => {
                    if self.focused.x < self.board.size.x - 1 {
                        self.focused.x += 1;
                    }
                    return EventResult::Consumed(None);
                }
                _ => (),
            },

            Event::Char(key) => match key {
                'c' => return self.reveal(self.focused),
                'x' => return self.flag(self.focused),
                'z' => return self.auto_reveal(self.focused),
                _ => (),
            },

            _ => (),
        }

        EventResult::Ignored
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Revealed(usize),
    Unrevealed,
    Flagged,
}

extern crate rand;

mod boardview;
mod minesweeper;

use crate::boardview::BoardView;
use crate::minesweeper::{Difficulty, MinesweeperBoard, DIFFICULTIES};
use cursive::direction::Orientation;
use cursive::views::{Button, Dialog, LinearLayout, Panel, SelectView};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    let quit_cb = |s: &mut Cursive| {
        s.add_layer(
            Dialog::text("Do you want to quit?")
                .button("Yes", |s| s.quit())
                .button("No", |s| {
                    s.pop_layer();
                }),
        );
    };

    siv.clear_global_callbacks(cursive::event::Event::CtrlChar('c'));

    siv.set_on_pre_event(cursive::event::Event::CtrlChar('c'), quit_cb);

    siv.add_global_callback('q', quit_cb);

    siv.add_layer(
        Dialog::new()
            .title("Minesweeper")
            .padding_lrtb(2, 2, 1, 1)
            .content(
                LinearLayout::vertical()
                    .child(Button::new_raw("  New game   ", show_options))
                    .child(Button::new_raw("    Exit     ", |s| s.quit())),
            ),
    );

    siv.run();
}

fn show_options(siv: &mut Cursive) {
    siv.add_layer({
        let dialog = Dialog::new().title("Select difficulty");

        let mut menu = SelectView::new();

        for difficulty in DIFFICULTIES {
            menu.add_item(difficulty.name, difficulty);
        }

        dialog
            .content(menu.on_submit(|s, difficulty| {
                s.pop_layer();
                start_game(s, difficulty);
            }))
            .dismiss_button("Back")
    });
}

fn start_game(siv: &mut Cursive, difficulty: &Difficulty) {
    let mut rng = rand::thread_rng();

    let board = MinesweeperBoard::new(
        difficulty.width,
        difficulty.height,
        difficulty.mines,
        &mut rng,
    );

    siv.add_layer(
        LinearLayout::new(Orientation::Vertical).child(
            Dialog::new()
                .title("Minesweeper")
                .content(Panel::new(BoardView::new(&board)))
                .button("Quit game", |s| {
                    s.pop_layer();
                }),
        ),
    );

    siv.add_layer(Dialog::info(
        "Controls:
Reveal cell:                  left click   / c
Mark as mine:                 right-click  / x
Reveal nearby unmarked cells: middle-click / z
Move around:                  arrow keys",
    ));
}

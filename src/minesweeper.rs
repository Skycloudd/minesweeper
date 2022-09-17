use cursive::Vec2;
use rand::seq::index::sample;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct MinesweeperBoard {
    pub size: Vec2,
    pub cells: Vec<Vec<Cell>>,

    pub mines_count: usize,

    pub remaining_mines: usize,
}

impl MinesweeperBoard {
    pub fn new<R>(width: usize, height: usize, mines: usize, rng: &mut R) -> Self
    where
        R: Rng,
    {
        let mut board = Self {
            size: Vec2::new(width, height),
            cells: vec![
                vec![
                    Cell {
                        is_mine: false,
                        surrounding_mines: 0,
                    };
                    width
                ];
                height
            ],
            mines_count: mines,
            remaining_mines: mines,
        };

        let mine_indices = sample(rng, width * height, mines);

        for idx in mine_indices {
            let pos = Vec2::new(idx % width, idx / width);

            board.cells[pos.y][pos.x].is_mine = true;

            for p in board.neighbours(pos) {
                if let Some(&mut Cell {
                    is_mine: false,
                    surrounding_mines: ref mut n,
                }) = board.get_mut(p)
                {
                    *n += 1;
                }
            }
        }

        board
    }

    pub fn neighbours(&self, pos: Vec2) -> Vec<Vec2> {
        let pos_min = pos.saturating_sub((1, 1));
        let pos_max = (pos + (2, 2)).or_min(self.size);
        (pos_min.x..pos_max.x)
            .flat_map(|x| (pos_min.y..pos_max.y).map(move |y| Vec2::new(x, y)))
            .filter(|&p| p != pos)
            .collect()
    }

    fn get_mut(&mut self, pos: Vec2) -> Option<&mut Cell> {
        if pos.fits_in(self.size) {
            Some(&mut self.cells[pos.y][pos.x])
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub is_mine: bool,
    pub surrounding_mines: usize,
}

#[derive(Clone, Debug)]
pub struct Difficulty {
    pub name: &'static str,
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

pub const EASY_DIFFICULTY: Difficulty = Difficulty {
    name: "Easy",
    width: 9,
    height: 9,
    mines: 10,
};

pub const INTERMEDIATE_DIFFICULTY: Difficulty = Difficulty {
    name: "Intermediate",
    width: 16,
    height: 16,
    mines: 40,
};

pub const EXPERT_DIFFICULTY: Difficulty = Difficulty {
    name: "Expert",
    width: 30,
    height: 16,
    mines: 99,
};

pub const DIFFICULTIES: [Difficulty; 3] =
    [EASY_DIFFICULTY, INTERMEDIATE_DIFFICULTY, EXPERT_DIFFICULTY];

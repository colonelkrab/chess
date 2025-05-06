use crate::grid::{CellId, Grid};
use crate::path::{Direction, Magnitude, Path};
use crate::pieces::{Piece, PieceType, Side};
use crate::textures::PieceTxts;

pub struct BoardStatus {
    pinned_pieces: Vec<(CellId, Path)>,
    checks: Vec<Path>,
}

#[derive(Debug)]
pub struct Check {
    pub absolute: bool,
    pub path: Path,
}
#[derive(Debug)]
pub struct Game {
    pub white_stack: Vec<Piece>,
    pub black_stack: Vec<Piece>,
    pub turn: Side,
    pub cell_cache: Vec<CellId>,
    pub white_king: CellId,
    pub black_king: CellId,
    pub checked: Option<Check>,
    pub move_count: u32,
}

impl Game {
    pub fn move_to_stack(&mut self, piece: Piece) {
        match piece.side {
            Side::Black => self.black_stack.push(piece),
            Side::White => self.white_stack.push(piece),
        }
    }
    pub fn king_now(&self) -> &CellId {
        match self.turn {
            Side::White => &self.white_king,
            Side::Black => &self.black_king,
        }
    }
    pub fn new(grid: &mut Grid, piecetxts: &'static PieceTxts) -> Game {
        let starting_pos =
            "wra1 wnb1 wbc1 wqd1 wke1 wbf1 wng1 wrh1 wpa2 wpb2 wpc2 wpd2 wpe2 wpf2 wpg2 wph2 bra8 bnb8 bbc8 bqd8 bke8 bbf8 bng8 brh8 bpa7 bpb7 bpc7 bpd7 bpe7 bpf7 bpg7 bph7";

        for code in starting_pos.split_whitespace() {
            let alg = AlgebraicNotation::new(code);
            grid.get_cell_mut(&alg.cell)
                .add_item(Piece::new(alg.piece, piecetxts, alg.side));
        }
        Game {
            white_stack: Vec::new(),
            black_stack: Vec::new(),
            turn: Side::White,
            cell_cache: Vec::new(),
            white_king: CellId(4, 7),
            black_king: CellId(4, 0),
            checked: None,
            move_count: 0,
        }
    }

    pub fn switch_turns(&mut self, grid: &mut Grid) {
        self.turn = self.turn.switch();
        self.move_count += 1;
        for id in &self.cell_cache {
            let cell = grid.get_cell_mut(id);
            cell.valid_moves = None;
            cell.pin = None;
        }
        self.cell_cache.clear();

        let status = self.get_board_status(grid).unwrap();
        if !status.checks.is_empty() {
            self.checked = Some(Check {
                absolute: status.checks.len() > 1,
                path: *status.checks.first().unwrap(),
            });
        } else {
            self.checked = None;
        }
        for (id, path) in status.pinned_pieces {
            let cell = grid.get_cell_mut(&id);
            cell.pin = Some(path);
            self.cell_cache.push(id);
        }
    }
    fn get_board_status(&self, grid: &mut Grid) -> Option<BoardStatus> {
        let king_cell: CellId = match self.turn {
            Side::White => self.white_king,
            Side::Black => self.black_king,
        };
        let Some(king) = &grid.get_cell(&king_cell).item else {
            println!("{},{}", king_cell.0, king_cell.1);
            println!("no king");
            return None;
        };
        let mut pinned_pieces: Vec<(CellId, Path)> = Vec::new();
        let mut checks: Vec<Path> = Vec::new();
        let side = &king.side;
        for direction in Direction::iterator() {
            let mut temp: Option<(CellId, Path)> = None;
            let mut current_cell = king_cell;
            let mut n = 0;

            while n < 10 {
                n += 1;
                let Some(id) = &current_cell.try_next_cellid(*direction) else {
                    break;
                };

                current_cell = *id;
                let cell = grid.get_cell(id);
                let Some(piece) = &cell.item else {
                    continue;
                };
                let path = Path {
                    magnitude: Magnitude::Fixed(n),
                    direction: *direction,
                };

                if piece.side == *side {
                    match temp {
                        Some(_) => break,
                        None => {
                            temp = Some((current_cell, path));
                            continue;
                        }
                    }
                } else {
                    match temp {
                        Some(_) => {
                            for line_of_sight in piece.line_of_sight {
                                if line_of_sight.is_equal_to(&path.flip()) {
                                    let Some(path_from_pinned) =
                                        &path.same_direction_subtract(&temp.unwrap().1)
                                    else {
                                        println!("path direction not same ");
                                        break;
                                    };
                                    pinned_pieces.push((temp.unwrap().0, *path_from_pinned));
                                }
                            }
                            break;
                        }
                        None => {
                            for line_of_sight in piece.line_of_sight {
                                if line_of_sight.is_equal_to(&path.flip()) {
                                    checks.push(path);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(BoardStatus {
            pinned_pieces,
            checks,
        })
    }
}
pub struct AlgebraicNotation {
    pub piece: PieceType,
    pub cell: CellId,
    pub side: Side,
}

impl AlgebraicNotation {
    pub fn new(code: &str) -> Self {
        let str_array: Vec<char> = code.chars().collect();
        let (side_, piece_, horiz, vert) = (str_array[0], str_array[1], str_array[2], str_array[3]);
        let side = match side_ {
            'b' => Side::Black,
            'w' => Side::White,
            _ => Side::Black,
        };
        let piece = match piece_ {
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'p' => PieceType::Pawn,
            _ => {
                println!("wrong code for piece");
                PieceType::Pawn
            }
        };
        let x: u32 = match horiz {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => 0,
        };

        let y: u32 = 8 - vert.to_digit(10).unwrap();
        let cell = CellId(x, y);

        Self { side, piece, cell }
    }
}

use std::vec;

use macroquad::prelude::*;
pub struct Cell {
    pub id: CellId,
    center: (f32, f32),
    origin: (f32, f32),
    color: Color,
    size: f32,
    pub item: Option<Piece>,
    pub valid_moves: Option<Vec<CellId>>,
}

impl Cell {
    pub fn draw(&self) {
        let (x, y) = self.origin;
        draw_rectangle(x, y, self.size, self.size, self.color);
        if let Some(piece) = &self.item {
            piece.draw(self.origin, self.size);
        }
    }
    pub fn highlight(&self) {
        let (x, y) = self.origin;
        draw_rectangle(x, y, self.size, self.size, Color::new(0.2, 1.0, 1.0, 0.25));
    }

    pub fn add_item(&mut self, piece: Piece) {
        self.item = Some(piece);
    }

    pub fn add_valid_moves(&mut self, valid_moves: Vec<CellId>) {
        self.valid_moves = Some(valid_moves);
    }

    pub fn move_item_to(&mut self, dest: &mut Cell, game: &mut Game) {
        let Some(valid_moves) = &self.valid_moves else {
            return;
        };

        let Some(piece) = &self.item else {
            return;
        };
        if !valid_moves.contains(&dest.id) {
            return;
        };
        if let Some(dest_item) = &dest.item {
            if dest_item.side == piece.side {
                return;
            } else {
                game.move_to_stack(dest.item.take().unwrap());
            }
        }
        self.valid_moves = None;
        let piece = self.item.take();
        dest.add_item(piece.unwrap());
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    cell_size: f32,
}
impl Grid {
    pub fn new64(cell_width: f32) -> Grid {
        let w = cell_width;
        let mut cells: Vec<Cell> = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let origin = (j as f32 * w, i as f32 * w);
                cells.push(Cell {
                    origin,
                    id: CellId(j, i),
                    size: w,
                    center: (origin.0 + (w / 2.0), origin.1 + (w / 2.0)),
                    color: if ((j % 2 == 0) & (i % 2 == 0)) | ((i + j) % 2 == 0) {
                        WHITE
                    } else {
                        GRAY
                    },
                    item: None,
                    valid_moves: None,
                });
            }
        }
        Grid {
            cells,
            cell_size: w,
        }
    }

    pub fn resize(&mut self, cell_width: f32) {
        let w = cell_width;
        self.cell_size = w;
        for cell in self.cells.iter_mut() {
            let CellId(x, y) = &cell.id;

            let origin = (*x as f32 * w, *y as f32 * w);
            let center = (origin.0 + (w / 2.0), origin.1 + (w / 2.0));
            cell.origin = origin;
            cell.center = center;
            cell.size = w;
        }
    }

    pub fn get_cell(&self, id: &CellId) -> &Cell {
        self.cells.get(id.to_vec_idx()).unwrap()
    }
    pub fn get_cell_mut(&mut self, coords: &CellId) -> &mut Cell {
        self.cells.get_mut(coords.to_vec_idx()).unwrap()
    }
    pub fn get_cell_mut_pair(&mut self, cell1: &CellId, cell2: &CellId) -> (&mut Cell, &mut Cell) {
        let cell1_idx: usize = cell1.to_vec_idx();
        let cell2_idx: usize = cell2.to_vec_idx();
        let mid: usize;
        if cell1_idx < cell2_idx {
            mid = cell1_idx + 1;
            let (g1, g2) = self.cells.split_at_mut(mid);
            (
                g1.get_mut(mid - 1).unwrap(),
                g2.get_mut(cell2_idx - mid).unwrap(),
            )
        } else {
            mid = cell2_idx + 1;
            let (g1, g2) = self.cells.split_at_mut(mid);
            (
                g2.get_mut(cell1_idx - mid).unwrap(),
                g1.get_mut(mid - 1).unwrap(),
            )
        }
    }

    pub fn draw(&self) {
        for cell in self.cells.iter() {
            cell.draw();
        }
    }
    pub fn coord_to_cell_id(&self, (xm, ym): (f32, f32)) -> Option<CellId> {
        let w = self.cell_size;
        let id: CellId = CellId((xm / w).floor() as u32, (ym / w).floor() as u32);
        if id.is_valid() {
            Some(id)
        } else {
            None
        }
    }
}

pub fn get_cell_width() -> f32 {
    let h = screen_height();
    let w = screen_width();
    if h > w {
        w / 8.0
    } else {
        h / 8.0
    }
}

pub struct PieceTxts {
    pub pawn_w: Texture2D,
    pub pawn_b: Texture2D,
    pub king_w: Texture2D,
    pub king_b: Texture2D,
    pub bishop_w: Texture2D,
    pub bishop_b: Texture2D,
}
impl PieceTxts {
    pub async fn default() -> PieceTxts {
        let pawn_w: Texture2D = load_texture("pieces_basic/white-pawn.png").await.unwrap();

        let pawn_b: Texture2D = load_texture("pieces_basic/black-pawn.png").await.unwrap();
        let king_w: Texture2D = load_texture("pieces_basic/white-king.png").await.unwrap();

        let king_b: Texture2D = load_texture("pieces_basic/black-king.png").await.unwrap();

        let bishop_w: Texture2D = load_texture("pieces_basic/white-bishop.png").await.unwrap();

        let bishop_b: Texture2D = load_texture("pieces_basic/black-bishop.png").await.unwrap();
        build_textures_atlas();

        PieceTxts {
            pawn_w,
            pawn_b,
            king_b,
            king_w,
            bishop_b,
            bishop_w,
        }
    }
}

#[derive(PartialEq)]
pub enum Side {
    Black,
    White,
}

pub enum PieceType {
    Pawn,
    King,
    Bishop,
}
pub struct Piece {
    pub name: String,
    pub side: Side,
    pub piece_type: PieceType,
    pub txt: Texture2D,
    pub valid_moves: Option<Vec<(u32, u32)>>,
}

impl Piece {
    pub fn new(piece_type: PieceType, txts: PieceTxts, side: Side) -> Piece {
        match piece_type {
            PieceType::Pawn => {
                let txt = if side == Side::White {
                    txts.pawn_w
                } else {
                    txts.pawn_b
                };
                Piece {
                    name: "Pawn".to_string(),
                    side,
                    piece_type,
                    txt,
                    valid_moves: None,
                }
            }
            PieceType::King => {
                let txt = if side == Side::White {
                    txts.king_w
                } else {
                    txts.king_b
                };
                Piece {
                    name: "King".to_string(),
                    side,
                    piece_type,
                    txt,
                    valid_moves: None,
                }
            }
            PieceType::Bishop => {
                let txt = if side == Side::White {
                    txts.bishop_w
                } else {
                    txts.bishop_b
                };
                Piece {
                    name: "Bishop".to_string(),
                    side,
                    piece_type,
                    txt,
                    valid_moves: None,
                }
            }
        }
    }
    pub fn draw(&self, origin: (f32, f32), size: f32) {
        let (x, y) = origin;
        draw_texture_ex(
            &self.txt,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }

    pub fn calc_valid_moves(&self, cell: &Cell, grid: &Grid) -> Vec<CellId> {
        let CellId(x, y) = cell.id;
        let mut valid_moves: Vec<CellId> = Vec::new();
        match self.piece_type {
            PieceType::Pawn => match self.side {
                Side::Black => {
                    let move_id = CellId(x, y + 1);
                    if move_id.is_valid() {
                        valid_moves.push(move_id);
                    }
                }
                Side::White => {
                    if y == 0 {
                        return valid_moves;
                    }

                    let move_id = CellId(x, y - 1);
                    if move_id.is_valid() {
                        valid_moves.push(move_id);
                    }
                }
            },
            PieceType::King => {}
            PieceType::Bishop => {}
        }

        valid_moves
    }
}

#[derive(PartialEq)]
pub struct CellId(pub u32, pub u32);
impl CellId {
    pub fn to_vec_idx(&self) -> usize {
        (self.0 + self.1 * 8) as usize
    }

    pub fn is_valid(&self) -> bool {
        (self.0 < 8) & (self.1 < 8)
    }
}

pub struct Game {
    pub white_stack: Vec<Piece>,
    pub black_stack: Vec<Piece>,
}

impl Game {
    pub fn move_to_stack(&mut self, piece: Piece) {
        match piece.side {
            Side::Black => self.black_stack.push(piece),
            Side::White => self.white_stack.push(piece),
        }
    }

    pub fn new() -> Game {
        Game {
            white_stack: Vec::new(),
            black_stack: Vec::new(),
        }
    }
}

pub enum Directions {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Directions {
    pub fn value(&self) -> (i8, i8) {
        match self {
            Directions::Right => (1, 0),
            Directions::Left => (-1, 0),
            Directions::Up => (0, -1),
            Directions::Down => (0, 1),
            Directions::UpRight => (1, -1),
            Directions::UpLeft => (-1, -1),
            Directions::DownRight => (1, 1),
            Directions::DownLeft => (-1, 1),
        }
    }
}

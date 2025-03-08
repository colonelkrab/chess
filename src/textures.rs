use macroquad::prelude::*;
pub struct PieceTxts {
    pub pawn_w: Texture2D,
    pub pawn_b: Texture2D,
    pub king_w: Texture2D,
    pub king_b: Texture2D,
    pub bishop_w: Texture2D,
    pub bishop_b: Texture2D,
    pub rook_b: Texture2D,
    pub rook_w: Texture2D,
}
impl PieceTxts {
    pub async fn default() -> PieceTxts {
        let pawn_w: Texture2D = load_texture("tatiana/pw.png").await.unwrap();
        let pawn_b: Texture2D = load_texture("tatiana/pb.png").await.unwrap();

        let king_w: Texture2D = load_texture("tatiana/kw.png").await.unwrap();
        let king_b: Texture2D = load_texture("tatiana/kb.png").await.unwrap();

        let bishop_w: Texture2D = load_texture("tatiana/bw.png").await.unwrap();
        let bishop_b: Texture2D = load_texture("tatiana/bb.png").await.unwrap();

        let rook_w: Texture2D = load_texture("tatiana/rw.png").await.unwrap();
        let rook_b: Texture2D = load_texture("tatiana/rb.png").await.unwrap();

        PieceTxts {
            pawn_w,
            pawn_b,
            king_b,
            king_w,
            bishop_b,
            bishop_w,
            rook_w,
            rook_b,
        }
    }
}

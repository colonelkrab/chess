use macroquad::prelude::*;
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
        let pawn_w: Texture2D = load_texture("tatiana/pw.png").await.unwrap();
        pawn_w.set_filter(FilterMode::Linear);
        let pawn_b: Texture2D = load_texture("tatiana/pb.png").await.unwrap();
        let king_w: Texture2D = load_texture("tatiana/kw.png").await.unwrap();

        let king_b: Texture2D = load_texture("tatiana/kb.png").await.unwrap();
        king_b.set_filter(FilterMode::Linear);
        let bishop_w: Texture2D = load_texture("tatiana/bw.png").await.unwrap();

        let bishop_b: Texture2D = load_texture("tatiana/bb.png").await.unwrap();

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

rust
pub fn get(board: &[u8; 8], x: usize, y: usize) -> u8 {
    if x > 7 {
        0
    } else if y > 7 {
        0
    } else {
        board[y]
    }
}

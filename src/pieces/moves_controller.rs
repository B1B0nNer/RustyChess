pub fn moves_controller(
    row: i8,
    col: i8,
    color: char,
    directions: &[(i8, i8)],
    board: &Vec<Vec<&'static str>>,
) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();

    for &(dr, dc) in directions {
        let mut r = row + dr;
        let mut c = col + dc;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            let piece = board[r as usize][c as usize];
            if piece.is_empty() {
                moves.push((r, c));
            } else {
                // Check if it's an opponent's piece
                if piece.chars().next().unwrap() != color {
                    moves.push((r, c));
                }
                break; // Cannot jump over pieces
            }
            r += dr;
            c += dc;
        }
    }

    moves
}

pub fn single_step_controller(
    row: i8,
    col: i8,
    color: char,
    directions: &[(i8, i8)],
    board: &Vec<Vec<&'static str>>,
) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();

    for &(dr, dc) in directions {
        let r = row + dr;
        let c = col + dc;

        if r >= 0 && r < 8 && c >= 0 && c < 8 {
            let piece = board[r as usize][c as usize];
            if piece.is_empty() || piece.chars().next().unwrap() != color {
                moves.push((r, c));
            }
        }
    }

    moves
}
type Cell = u8;
type Row = [u8; 3];
type Box = [Row; 3];
type Sudoku = [Box; 9];

fn available_in_box(b: Box) -> Vec<u8> {
    let mut available = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for &r in b.into_iter() {
        for &v in r.into_iter() {
            available.retain(|&x| x != v);
        }
    }
    return available;
}

fn solve(sudoku: Sudoku) -> Sudoku {
    for &b in sudoku.into_iter() {
        for &r in b.into_iter() {
            for &c in r.into_iter() {
                // Get available values for box
                // Get available values for column
                // Get available values for row
                // If combination == 1 use that
            }
        }
    }
    return sudoku;
}

fn main() {
    let puzzle: Sudoku = [
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
    ];
}

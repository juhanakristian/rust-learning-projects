type Row = [u8; 9];
type Sudoku = [Row; 9];
type Position = (usize, usize);

fn print_sudoku(sudoku: Sudoku) {
    println!();
    for row in sudoku.iter() {
        println!("{:?}", row);
    }
}

fn allowed_for_box(value: u8, row: usize, column: usize, sudoku: Sudoku) -> bool {
    let box_row = row / 3;
    let box_row_origin = box_row * 3;
    let box_col = column / 3;
    let box_col_origin = box_col * 3;

    for r in box_row_origin..(box_row_origin + 3) {
        let row_values = sudoku.get(r).unwrap();
        for c in box_col_origin..(box_col_origin + 3) {
            let v = *row_values.get(c).unwrap();
            if v == value {
                return false;
            }
        }
    }

    return true;
}

fn allowed_for_row(value: u8, row: usize, sudoku: Sudoku) -> bool {
    let row_values = sudoku.get(row).unwrap();
    for c in 0..9 {
        let v = *row_values.get(c).unwrap();
        if v == value {
            return false;
        }
    }

    return true;
}

fn allowed_for_column(value: u8, column: usize, sudoku: Sudoku) -> bool {
    for r in 0..9 {
        let row_values = sudoku.get(r).unwrap();
        let v = *row_values.get(column).unwrap();
        if v == value {
            return false;
        }
    }

    return true;
}

fn next(pos: Position) -> Position {
    let r = pos.0;
    let c = pos.1;

    let mut nc = c + 1;
    let mut nr = r;
    if nc > 8 {
        nc = 0;
        nr = r + 1;
    }

    return (nr, nc);
}

fn prev(pos: Position) -> Position {
    if pos.1 == 0 {
        if pos.0 > 0 {
            return (pos.0 - 1, 8);
        } else {
            return (0, 0);
        }
    } else {
        return (pos.0, pos.1 - 1);
    }
}

fn solve(sudoku: Sudoku) -> Sudoku {
    let mut fixed: Vec<Position> = vec![];
    let mut solved_sudoku: Sudoku = sudoku.clone();
    for r in 0..9 {
        for c in 0..9 {
            let value = *sudoku.get(r).unwrap().get(c).unwrap();
            if value != 0 {
                fixed.push((r, c));
            }
        }
    }

    let mut pos = (0, 0);
    loop {
        while fixed.iter().any(|x| *x == pos) {
            pos = next(pos);
            if pos.0 > 8 {
                return solved_sudoku;
            }
        }

        let mut value = *solved_sudoku.get(pos.0).unwrap().get(pos.1).unwrap();

        let mut allowed = false;
        while !allowed && value <= 9 {
            value += 1;
            allowed = allowed_for_box(value, pos.0, pos.1, solved_sudoku)
                && allowed_for_row(value, pos.0, solved_sudoku)
                && allowed_for_column(value, pos.1, solved_sudoku);
        }

        if value > 9 {
            solved_sudoku[pos.0][pos.1] = 0;
            pos = prev(pos);
            while fixed.iter().any(|x| *x == pos) {
                pos = prev(pos);
            }
        } else {
            solved_sudoku[pos.0][pos.1] = value;
            if pos == (8, 8) {
                return solved_sudoku;
            }
            pos = next(pos);
        }
    }
}

#[test]
fn test_puzzle_allowed_for_box() {
    let puzzle: Sudoku = [
        [0, 0, 0, 2, 6, 0, 7, 0, 1],
        [6, 8, 0, 0, 7, 0, 0, 9, 0],
        [1, 9, 0, 0, 0, 4, 5, 0, 0],
        [8, 2, 0, 1, 0, 0, 0, 4, 0],
        [0, 0, 4, 6, 0, 2, 9, 0, 0],
        [0, 5, 0, 0, 0, 3, 0, 2, 8],
        [0, 0, 9, 3, 0, 0, 0, 7, 4],
        [0, 4, 0, 0, 5, 0, 0, 3, 6],
        [7, 0, 3, 0, 1, 8, 0, 0, 0],
    ];

    assert!(allowed_for_box(2, 0, 0, puzzle));
    assert!(allowed_for_box(3, 0, 0, puzzle));
    assert!(allowed_for_box(4, 0, 0, puzzle));
    assert!(allowed_for_box(5, 0, 0, puzzle));
    assert!(allowed_for_box(7, 0, 0, puzzle));
    assert!(!allowed_for_box(6, 0, 0, puzzle));
    assert!(!allowed_for_box(1, 0, 0, puzzle));
    assert!(!allowed_for_box(8, 0, 0, puzzle));
    assert!(!allowed_for_box(9, 0, 0, puzzle));
}

#[test]
fn test_puzzle_allowed_for_row() {
    let puzzle: Sudoku = [
        [0, 0, 0, 2, 6, 0, 7, 0, 1],
        [6, 8, 0, 0, 7, 0, 0, 9, 0],
        [1, 9, 0, 0, 0, 4, 5, 0, 0],
        [8, 2, 0, 1, 0, 0, 0, 4, 0],
        [0, 0, 4, 6, 0, 2, 9, 0, 0],
        [0, 5, 0, 0, 0, 3, 0, 2, 8],
        [0, 0, 9, 3, 0, 0, 0, 7, 4],
        [0, 4, 0, 0, 5, 0, 0, 3, 6],
        [7, 0, 3, 0, 1, 8, 0, 0, 0],
    ];

    assert!(allowed_for_row(3, 0, puzzle));
    assert!(allowed_for_row(4, 0, puzzle));
    assert!(allowed_for_row(5, 0, puzzle));
    assert!(allowed_for_row(8, 0, puzzle));
    assert!(allowed_for_row(9, 0, puzzle));
    assert!(!allowed_for_row(1, 0, puzzle));
    assert!(!allowed_for_row(2, 0, puzzle));
    assert!(!allowed_for_row(6, 0, puzzle));
    assert!(!allowed_for_row(7, 0, puzzle));
}

#[test]
fn test_puzzle_allowed_for_col() {
    let puzzle: Sudoku = [
        [0, 0, 0, 2, 6, 0, 7, 0, 1],
        [6, 8, 0, 0, 7, 0, 0, 9, 0],
        [1, 9, 0, 0, 0, 4, 5, 0, 0],
        [8, 2, 0, 1, 0, 0, 0, 4, 0],
        [0, 0, 4, 6, 0, 2, 9, 0, 0],
        [0, 5, 0, 0, 0, 3, 0, 2, 8],
        [0, 0, 9, 3, 0, 0, 0, 7, 4],
        [0, 4, 0, 0, 5, 0, 0, 3, 6],
        [7, 0, 3, 0, 1, 8, 0, 0, 0],
    ];

    assert!(allowed_for_column(3, 0, puzzle));
    assert!(allowed_for_column(4, 0, puzzle));
    assert!(allowed_for_column(5, 0, puzzle));
    assert!(allowed_for_column(2, 0, puzzle));
    assert!(allowed_for_column(9, 0, puzzle));
    assert!(!allowed_for_column(1, 0, puzzle));
    assert!(!allowed_for_column(6, 0, puzzle));
    assert!(!allowed_for_column(7, 0, puzzle));
    assert!(!allowed_for_column(8, 0, puzzle));
}

#[test]
fn test_next_position() {
    assert!(next((0, 0)) == (0, 1));
    assert!(next((0, 8)) == (1, 0));
}

#[test]
fn test_prev_position() {
    assert!(prev((1, 0)) == (0, 8));
    assert!(prev((0, 1)) == (0, 0));
}

fn main() {
    let puzzle: Sudoku = [
        [0, 0, 0, 2, 6, 0, 7, 0, 1],
        [6, 8, 0, 0, 7, 0, 0, 9, 0],
        [1, 9, 0, 0, 0, 4, 5, 0, 0],
        [8, 2, 0, 1, 0, 0, 0, 4, 0],
        [0, 0, 4, 6, 0, 2, 9, 0, 0],
        [0, 5, 0, 0, 0, 3, 0, 2, 8],
        [0, 0, 9, 3, 0, 0, 0, 7, 4],
        [0, 4, 0, 0, 5, 0, 0, 3, 6],
        [7, 0, 3, 0, 1, 8, 0, 0, 0],
    ];

    let solved = solve(puzzle);
    print_sudoku(solved);
}

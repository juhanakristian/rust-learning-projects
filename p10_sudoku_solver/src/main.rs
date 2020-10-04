type Row = [u8; 9];
type Sudoku = [Row; 9];
type Position = (usize, usize);

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
    let r = pos.0;
    let c = pos.1;

    let mut nc = c - 1;
    let mut nr = r;
    if nc <= 0 {
        nc = 8;
        nr = r - 1;
    }

    return (nr, nc);
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
        if fixed.into_iter().any(|x| x == pos) {
            pos = next(pos);
            if pos.0 > 8 {
                break;
            }

            continue;
        }

        let mut value = *sudoku.get(pos.0).unwrap().get(pos.1).unwrap();

        let mut allowed = false;
        while !allowed && value < 9 {
            value += 1;
            allowed = allowed_for_box(value, pos.0, pos.1, sudoku)
                && allowed_for_row(value, pos.0, sudoku)
                && allowed_for_column(value, pos.1, sudoku);
        }

        if value > 9 {
            solved_sudoku[pos.0][pos.1] = 0;
            pos = prev(pos);
        } else {
            solved_sudoku[pos.0][pos.1] = value;
        }
    }

    return sudoku;
}

fn main() {
    let puzzle: Sudoku = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
}

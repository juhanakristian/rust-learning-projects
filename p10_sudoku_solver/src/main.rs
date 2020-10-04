type Row = [u8; 9];
type Sudoku = [Row; 9];
type Position = [usize; 2];

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

fn solve(sudoku: Sudoku) -> Sudoku {
    let mut fixed: Vec<Position> = vec![];
    for r in 0..9 {
        for c in 0..9 {
            let value = *sudoku.get(r).unwrap().get(c).unwrap();
            if value != 0 {
                fixed.push([r, c]);
            }
        }
    }

    let mut r = 0;
    let mut c = 0;
    while {
        let value = *sudoku.get(r).unwrap().get(c).unwrap();
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

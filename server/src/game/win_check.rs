use shared::protocol::Cell;

pub fn check_winner(board: &[Vec<Cell>]) -> Option<Cell> {
    let n = board.len();
    let dirs = [(1,0), (0,1), (1,1), (1,-1)];

    for y in 0..n {
        for x in 0..n {
            let cell = board[y][x];
            if cell == Cell::Empty {
                continue;
            }

            for (dx, dy) in dirs {
                let mut count = 1;

                for i in 1..n {
                    let nx = x as isize + dx * i as isize;
                    let ny = y as isize + dy * i as isize;

                    if nx < 0 || ny < 0 || nx >= n as isize || ny >= n as isize {
                        break;
                    }

                    if board[ny as usize][nx as usize] == cell {
                        count += 1;
                    } else {
                        break;
                    }
                }

                if count == n {
                    return Some(cell);
                }
            }
        }
    }

    None
}

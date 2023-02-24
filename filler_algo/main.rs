use rand::Rng;
use std::vec;

fn main() {
    let mut four_indexes: Vec<usize> = vec![28, 17];
    let mut three_indexes: Vec<usize> = vec![18];
    let mut two_indexes: Vec<usize> = vec![61, 72];
    let mut one_indexes: Vec<usize> = vec![71];
    let mut board: [u8; 90] = generate_board();
    let mut ownership: [u8; 90] = [
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 4, 3, 5, 5, 0, 0, 0, 0, 0, 0, 0, 4, 5,
        5, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 5,
        5, 2, 0, 0, 0, 0, 0, 0, 0, 5, 5, 1, 2, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    ];
    let mut right_score: u8 = 1;
    let mut left_score: u8 = 1;
    let mut right_captured: Vec<usize> = vec![];
    let mut left_captured: Vec<usize> = vec![];
    loop {
        let mut num: u8 = 7;
        print_board(board);
        while num == 7 {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line);
            match line.trim().parse::<u8>() {
                Ok(i) => {
                    if i > 6 || i < 1 {
                        println!("Pick a number between 1 and 6");
                    } else if board[18] == i {
                        println!("You can't pick the same color as the opponent");
                    } else if board[71] == i {
                        println!("You can't pick the same color twice in a row");
                    } else {
                        num = i;
                    }
                }
                Err(..) => println!("Invalid Input"),
            }
        }
        (
            board,
            ownership,
            two_indexes,
            one_indexes,
            right_score,
            right_captured,
        ) = make_move(
            board,
            ownership,
            num,
            two_indexes,
            one_indexes,
            4,
            1,
            2,
            right_score,
            right_captured,
        );
        print_board(board);
        (
            board,
            ownership,
            four_indexes,
            three_indexes,
            left_score,
            left_captured,
        ) = algo_move(
            board,
            ownership,
            four_indexes.clone(),
            three_indexes.clone(),
            left_score,
            left_captured,
        );
        println!("{:?} {:?}", right_captured, left_captured);
    }
}

fn algo_move(
    board: [u8; 90],
    ownership: [u8; 90],
    touch_indexes: Vec<usize>,
    owner_indexes: Vec<usize>,
    left_score: u8,
    left_captured: Vec<usize>,
) -> ([u8; 90], [u8; 90], Vec<usize>, Vec<usize>, u8, Vec<usize>) {
    let mut lines: Vec<([u8; 90], [u8; 90], Vec<usize>, Vec<usize>, u8, Vec<usize>)> = vec![(
        board,
        ownership,
        touch_indexes.clone(),
        owner_indexes.clone(),
        left_score,
        left_captured.clone(),
    )];
    let mut color = 0;
    let mut paths: Vec<[u8; 8]> = vec![[0, 0, 0, 0, 0, 0, 0, 0]];
    let here_touch_colors = generate_touch_colors(board, touch_indexes.clone());
    for l in 0..8 {
        let mut temp_lines: Vec<([u8; 90], [u8; 90], Vec<usize>, Vec<usize>, u8, Vec<usize>)> =
            vec![];
        let mut temp_paths: Vec<[u8; 8]> = vec![];
        let mut h = 0;
        if lines.len() == 0 {
            if here_touch_colors.clone().len() == 0 {
                for i in 1..7 {
                    if i != board[71] && i != board[18] {
                        color = i;
                    }
                }
            } else {
                for i in here_touch_colors.clone() {
                    if i != board[71] {
                        color = i;
                    }
                }
            }
        }
        for i in &lines {
            let possible_moves = generate_touch_colors(i.0, i.2.clone());
            if possible_moves.len() == 0 {
                for o in 1..7 {
                    if o != board[71] && o != board[18] {
                        color = o;
                        temp_lines.push(make_move(
                            i.0,
                            i.1,
                            o,
                            i.2.clone(),
                            i.3.clone(),
                            2,
                            3,
                            4,
                            i.4,
                            i.5.clone(),
                        ));
                        let mut new_path: [u8; 8] = paths[h];
                        new_path[l] = o;
                        temp_paths.push(new_path);
                        break;
                    }
                }
            }
            for j in possible_moves {
                temp_lines.push(make_move(
                    i.0,
                    i.1,
                    j,
                    i.2.clone(),
                    i.3.clone(),
                    2,
                    3,
                    4,
                    i.4,
                    i.5.clone(),
                ));
                let mut new_path: [u8; 8] = paths[h];
                new_path[l] = j;
                temp_paths.push(new_path);
            }
            h += 1;
        }
        lines = temp_lines;
        paths = temp_paths;
    }
    let mut max = 0;
    for i in 0..lines.len() {
        if lines[i].4 > max && paths[i][0] != board[71] {
            max = lines[i].4;
            color = paths[i][0];
        }
    }
    println!("max {}", max);
    if color == 0 {
        for i in here_touch_colors {
            if i != board[71] {
                color = i;
            }
        }
    }
    println!("{}", color);
    make_move(
        board,
        ownership,
        color,
        touch_indexes,
        owner_indexes,
        2,
        3,
        4,
        left_score,
        left_captured,
    )
}

fn generate_touch_colors(board: [u8; 90], touch_indexes: Vec<usize>) -> Vec<u8> {
    let mut touch_colors: Vec<u8> = vec![];
    for i in &touch_indexes {
        if !touch_colors.contains(&board[*i]) {
            touch_colors.push(board[*i]);
        }
    }
    touch_colors
}

fn make_move(
    mut board: [u8; 90],
    mut ownership: [u8; 90],
    color: u8,
    mut touch_indexes: Vec<usize>,
    mut owner_indexes: Vec<usize>,
    opp_touch: u8,
    you_own: u8,
    you_touch: u8,
    mut score: u8,
    mut captured: Vec<usize>,
) -> ([u8; 90], [u8; 90], Vec<usize>, Vec<usize>, u8, Vec<usize>) {
    let mut touch_remove: Vec<usize> = vec![];
    for i in 0..touch_indexes.len() {
        if board[touch_indexes[i]] == color {
            touch_remove.push(touch_indexes[i]);
            ownership[touch_indexes[i]] = you_own;
            owner_indexes.push(touch_indexes[i]);
            if !captured.contains(&touch_indexes[i]) {
                score += 1;
            } else {
                captured.remove(
                    captured
                        .iter()
                        .position(|x| *x == touch_indexes[i]).unwrap()
                );
            }
            if touch_indexes[i] <= 18
                && (ownership[touch_indexes[i] + 1] != you_own
                    && ownership[touch_indexes[i] + 1] != 5
                    && ownership[touch_indexes[i] - 1] != you_own
                    && ownership[touch_indexes[i] - 1] != 5)
            {
                let mut bfs_res = bfs(ownership, touch_indexes[i] + 1, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
                bfs_res = bfs(ownership, touch_indexes[i] - 1, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
            } else if touch_indexes[i] >= 71
                && touch_indexes[i] <= 78
                && (ownership[touch_indexes[i] + 1] != you_own
                    && ownership[touch_indexes[i] + 1] != 5
                    && ownership[touch_indexes[i] - 1] != you_own
                    && ownership[touch_indexes[i] - 1] != 5)
            {
                let mut bfs_res = bfs(ownership, touch_indexes[i] + 1, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
                bfs_res = bfs(ownership, touch_indexes[i] - 1, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
            } else if touch_indexes[i] % 10 == 1
                && (ownership[touch_indexes[i] + 10] != you_own
                    && ownership[touch_indexes[i] + 10] != 5
                    && ownership[touch_indexes[i] - 10] != you_own
                    && ownership[touch_indexes[i] - 10] != 5)
            {
                let mut bfs_res = bfs(ownership, touch_indexes[i] + 10, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
                bfs_res = bfs(ownership, touch_indexes[i] - 10, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
            } else if touch_indexes[i] % 10 == 8
                && (ownership[touch_indexes[i] + 10] != you_own
                    && ownership[touch_indexes[i] + 10] != 5
                    && ownership[touch_indexes[i] - 10] != you_own
                    && ownership[touch_indexes[i] - 10] != 5)
            {
                let mut bfs_res = bfs(ownership, touch_indexes[i] + 10, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
                bfs_res = bfs(ownership, touch_indexes[i] - 10, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
            }
            if !touch_indexes.contains(&(touch_indexes[i] - 1))
                && (ownership[touch_indexes[i] - 1] == 0
                    || ownership[touch_indexes[i] - 1] == opp_touch)
            {
                ownership[touch_indexes[i] - 1] = you_touch;
                touch_indexes.push(touch_indexes[i] - 1);
            }
            if !touch_indexes.contains(&(touch_indexes[i] + 1))
                && (ownership[touch_indexes[i] + 1] == 0
                    || ownership[touch_indexes[i] + 1] == opp_touch)
            {
                ownership[touch_indexes[i] + 1] = you_touch;
                touch_indexes.push(touch_indexes[i] + 1);
            }
            if !touch_indexes.contains(&(touch_indexes[i] + 10))
                && (ownership[touch_indexes[i] + 10] == 0
                    || ownership[touch_indexes[i] + 10] == opp_touch)
            {
                ownership[touch_indexes[i] + 10] = you_touch;
                touch_indexes.push(touch_indexes[i] + 10);
            }
            if !touch_indexes.contains(&(touch_indexes[i] - 10))
                && (ownership[touch_indexes[i] - 10] == 0
                    || ownership[touch_indexes[i] - 10] == opp_touch)
            {
                ownership[touch_indexes[i] - 10] = you_touch;
                touch_indexes.push(touch_indexes[i] - 10);
            }
        }
    }
    for i in touch_remove {
        touch_indexes.remove(touch_indexes.iter().position(|x| *x == i).unwrap());
    }
    for j in 0..owner_indexes.len() {
        board[owner_indexes[j]] = color;
    }
    (
        board,
        ownership,
        touch_indexes,
        owner_indexes,
        score,
        captured,
    )
}

fn bfs(ownership: [u8; 90], index: usize, you_touch: u8, they_touch: u8) -> (bool, Vec<usize>) {
    let mut stack: Vec<usize> = vec![index];
    let mut visited: Vec<usize> = vec![];
    if ownership[index] == they_touch {
        return (false, vec![]);
    }
    while stack.len() > 0 {
        let current: usize = stack[stack.len() - 1];
        if (ownership[current + 1] == 0 || ownership[current + 1] == you_touch)
            && !visited.contains(&(current + 1))
        {
            stack.push(current + 1);
        } else if ownership[current + 1] == they_touch {
            return (false, vec![]);
        }
        if (ownership[current - 1] == 0 || ownership[current - 1] == you_touch)
            && !visited.contains(&(current - 1))
        {
            stack.push(current - 1);
        } else if ownership[current - 1] == they_touch {
            return (false, vec![]);
        }
        if (ownership[current + 10] == 0 || ownership[current + 10] == you_touch)
            && !visited.contains(&(current + 10))
        {
            stack.push(current + 10);
        } else if ownership[current + 10] == they_touch {
            return (false, vec![]);
        }
        if (ownership[current - 10] == 0 || ownership[current - 10] == you_touch)
            && !visited.contains(&(current - 10))
        {
            stack.push(current - 10);
        } else if ownership[current - 10] == they_touch {
            return (false, vec![]);
        }
        if !visited.contains(&current) {
            visited.push(current);
        }
        stack.remove(stack.iter().position(|&r| r == current).unwrap());
    }
    
    (true, visited)
}

fn generate_board() -> [u8; 90] {
    let mut board: [u8; 90] = [0; 90];
    let mut rng = rand::thread_rng();
    for i in 9..79 {
        let mut num: u8 = (rng.gen::<u8>() % 6) + 1;
        if i % 10 == 9 || i % 10 == 0 {
            board[i] = 0;
        } else {
            board[i] = num;
            num = (rng.gen::<u8>() % 6) + 1;
            while board[i - 10] == board[i]
                || board[i + 10] == board[i]
                || board[i - 1] == board[i]
                || board[i + 1] == board[i]
            {
                board[i] = num;
                num = (rng.gen::<u8>() % 6) + 1;
            }
        }
    }
    board
}

fn print_board(board: [u8; 90]) {
    let mut string_board = String::new();
    for i in 0..70 {
        if board[i + 11] != 0 {
            match board[i + 11] {
                1 => string_board += "\u{001b}[41m  \u{001b}[0m",
                2 => string_board += "\u{001b}[43m  \u{001b}[0m",
                3 => string_board += "\u{001b}[42m  \u{001b}[0m",
                4 => string_board += "\u{001b}[44m  \u{001b}[0m",
                5 => string_board += "\u{001b}[45m  \u{001b}[0m",
                _ => string_board += "\u{001b}[40m  \u{001b}[0m",
            };
        } else if board[i + 11] == 0 && board[i - 1 + 11] == 0 {
            string_board += "\n";
        }
    }
    string_board += "\n\u{001b}[41m 1 \u{001b}[43m 2 \u{001b}[42m 3 \u{001b}[44m 4 \u{001b}[45m 5 \u{001b}[40m 6 \u{001b}[0m\n\n";
    print!("\r{}", string_board);
}

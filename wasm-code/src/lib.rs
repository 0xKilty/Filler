use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use js_sys::Array;
use js_sys::ArrayIter;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn algo_move(b: Array, o: Array, t: Array, ow: Array, score: u8, c: Array) -> u8 {
    let mut board: [u8; 90] = [0; 90];
    let mut ownership: [u8; 90] = [0; 90];
    let mut touch_indexes: Vec<usize> = vec![];
    let mut owner_indexes: Vec<usize> = vec![];
    let mut captured: Vec<usize> = vec![];
    for i in 0..b.length() {
        board[i as usize] = b.at(i as i32).as_f64().unwrap() as u8;
        ownership[i as usize] = o.at(i as i32).as_f64().unwrap() as u8;
    }
    for i in t.iter() {    
        let y = i.as_f64();
        match y {
            Some(x) => touch_indexes.push(x as usize),
            None => {},
        }
    }
    for i in ow.iter() {
        let y = i.as_f64();
        match y {
            Some(x) => owner_indexes.push(x as usize),
            None => {},
        }
    }
    for i in 0..c.length() {
        captured.push(c.at(i as i32).as_f64().unwrap() as usize);
    }
    let mut lines: Vec<([u8; 90], [u8; 90], Vec<usize>, Vec<usize>, u8, Vec<usize>)> = vec![(
        board,
        ownership,
        touch_indexes.clone(),
        owner_indexes.clone(),
        score,
        captured.clone(),
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
    if color == 0 {
        for i in here_touch_colors {
            if i != board[71] {
                color = i;
            }
        }
    }
    color - 1
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
                let mut bfs_res = bfsR(ownership, touch_indexes[i] + 1, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
                bfs_res = bfsR(ownership, touch_indexes[i] - 1, you_touch, opp_touch);
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
                let mut bfs_res = bfsR(ownership, touch_indexes[i] + 1, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
                bfs_res = bfsR(ownership, touch_indexes[i] - 1, you_touch, opp_touch);
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
                let mut bfs_res = bfsR(ownership, touch_indexes[i] + 10, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
                bfs_res = bfsR(ownership, touch_indexes[i] - 10, you_touch, opp_touch);
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
                let mut bfs_res = bfsR(ownership, touch_indexes[i] + 10, you_touch, opp_touch);
                if bfs_res.0 {
                    for t in bfs_res.1 {
                        if !captured.contains(&t) {
                            score += 1;
                            captured.push(t);
                        }
                    }
                }
                bfs_res = bfsR(ownership, touch_indexes[i] - 10, you_touch, opp_touch);
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

fn generate_touch_colors(board: [u8; 90], touch_indexes: Vec<usize>) -> Vec<u8> {
    let mut touch_colors: Vec<u8> = vec![];
    for i in &touch_indexes {
        if !touch_colors.contains(&board[*i]) {
            touch_colors.push(board[*i]);
        }
    }
    touch_colors
}

fn bfsR(ownership: [u8; 90], index: usize, you_touch: u8, they_touch: u8) -> (bool, Vec<usize>) {
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

#[wasm_bindgen]
pub fn bfs(ownership: Array, index: i32) -> Array {
    let mut stack: Vec<i32> = vec![index];
    let mut visited: Vec<i32> = vec![];
    let mut res: Array = Array::new();
    if ownership.at(index) == 2 {
        return res;
    }
    while stack.len() > 0 {
        let current: i32 = stack[stack.len() - 1];
        if (ownership.at(current + 1) == 0 || ownership.at(current + 1) == 4)
            && !visited.contains(&(current + 1))
        {
            stack.push(current + 1);
        } else if ownership.at(current + 1) == 2 {
            return res;
        }
        if (ownership.at(current - 1) == 0 || ownership.at(current - 1) == 4)
            && !visited.contains(&(current - 1))
        {
            stack.push(current - 1);
        } else if ownership.at(current - 1) == 2 {
            return res;
        }
        if (ownership.at(current + 10) == 0 || ownership.at(current + 10) == 4)
            && !visited.contains(&(current + 10))
        {
            stack.push(current + 10);
        } else if ownership.at(current + 10) == 2 {
            return res;
        }
        if (ownership.at(current - 10) == 0 || ownership.at(current - 10) == 4)
            && !visited.contains(&(current - 10))
        {
            stack.push(current - 10);
        } else if ownership.at(current - 10) == 2 {
            return res;
        }
        if !visited.contains(&current) {
            visited.push(current);
        }
        stack.remove(stack.iter().position(|&r| r == current).unwrap());
    }
    for i in visited {
        res.push(&JsValue::from_f64(f64::from(i)));
    }
    res
}
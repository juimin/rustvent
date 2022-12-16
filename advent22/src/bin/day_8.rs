use advent22::get_input_contents;


fn init_visibility_map(trees: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let mut mask = Vec::new();

    for row in trees {
        let mut mask_row = Vec::new();
        for _ in row {
            mask_row.push(false);
        }
        mask.push(mask_row);
    }

    return mask;
}

fn find_visible_trees(trees: &Vec<Vec<i32>>, mut v_map: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // From left
    for row in 0..trees.len() {
        let mut lowest_height = -1;
        for col in 0..trees[row].len() {
            if trees[row][col] > lowest_height {
                v_map[row][col] = true;
                lowest_height = trees[row][col];
            }
        }
    }

    // From right
    for row in 0..trees.len() {
        let mut lowest_height = -1;
        let r = trees.len() - 1 - row;
        for col in 0..trees[row].len() {
            let c = trees[row].len() -1 - col;
            if trees[r][c] > lowest_height {
                v_map[r][c] = true;
                lowest_height = trees[r][c];
            }
        }
    }

    let columns = trees[0].len();

    for col in 0..columns {
        let mut lowest_height = -1;
        for row in 0..trees.len() {
            if trees[row][col] > lowest_height {
                v_map[row][col] = true;
                lowest_height = trees[row][col];
            }
        }
    }

    for col in 0..columns {
        let c = columns - 1 - col;
        let mut lowest_height = -1;
        for row in 0..trees.len() {
            let r = trees.len() - 1 - row;
            if trees[r][c] > lowest_height {
                v_map[r][c] = true;
                lowest_height = trees[r][c];
            }
        }
    }

    v_map
}


fn scenic_helper(trees: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let mut total = 1;

    let mut score = 0;

    let max_height = trees[row][col];

    // check left
    for offset in 0..col {
        let c = col - offset - 1;
        score += 1;
        if trees[row][c] >= max_height {
            break;
        }
    }
    total *= score;
    score = 0;

    // check right
    for c in (col + 1)..trees[row].len() {
        score += 1;
        if trees[row][c] >= max_height {
            break;
        }
    }
    total *= score;
    score = 0;

    // check up
    for offset in 0..row {
        let r = row - offset - 1;
        score += 1;
        if trees[r][col] >= max_height {
            break;
        }
    }
    total *= score;
    score = 0;

    // check down
    for r in (row + 1)..trees.len() {
        score += 1;
        if trees[r][col] >= max_height {
            break;
        }
    }
    total *= score;


    return total;
}

fn calc_scenic_score(trees: &Vec<Vec<i32>>) -> i32 {
    let mut best_scenic_score = 0;

    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let scenic = scenic_helper(trees, row, col);
            if scenic > best_scenic_score {
                best_scenic_score = scenic;
            }
        }
    }

    return best_scenic_score;
}


fn main() {
    let file_contents = get_input_contents();

    let mut trees: Vec<Vec<i32>> = Vec::new();

    for line in file_contents.trim().split("\n") {
        let mut v: Vec<i32> = Vec::new();
        for c in line.chars() {
            v.push(String::from(c).parse::<i32>().expect("Should be int"));
        }
        trees.push(v);

    }

    let mut v_map = init_visibility_map(&trees);
    v_map = find_visible_trees(&trees, v_map);

    let mut total = 0;
    for line in v_map {
        for tree in line {
            if tree {
                total += 1;
            }
        }
    }

    println!("visible trees: {}", total);

    let best_scenic_score = calc_scenic_score(&trees);

    println!("best scenic score: {}", best_scenic_score);


}

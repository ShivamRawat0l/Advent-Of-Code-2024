use std::fs;

fn convert_to_matrix(string: String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];

    string.lines().for_each(|line| {
        let line_vec = line.chars().collect::<Vec<char>>();
        result.push(line_vec);
    });
    result
}
/*
fn part1() {
    if tree[i][j] == 'X' {
        if (j + 3) < tree[i].len() {
            if (i + 3) < tree.len()
                && tree[i + 1][j + 1] == 'M'
                && tree[i + 2][j + 2] == 'A'
                && tree[i + 3][j + 3] == 'S'
            {
                total += 1;
            }
            if i >= 3
                && tree[i - 1][j + 1] == 'M'
                && tree[i - 2][j + 2] == 'A'
                && tree[i - 3][j + 3] == 'S'
            {
                total += 1;
            }
            if tree[i][j + 1] == 'M' && tree[i][j + 2] == 'A' && tree[i][j + 3] == 'S' {
                total += 1;
            }
        }
        if j >= 3 {
            if (i + 3) < tree.len()
                && tree[i + 1][j - 1] == 'M'
                && tree[i + 2][j - 2] == 'A'
                && tree[i + 3][j - 3] == 'S'
            {
                total += 1;
            }
            if i >= 3
                && tree[i - 1][j - 1] == 'M'
                && tree[i - 2][j - 2] == 'A'
                && tree[i - 3][j - 3] == 'S'
            {
                total += 1;
            }
            if tree[i][j - 1] == 'M' && tree[i][j - 2] == 'A' && tree[i][j - 3] == 'S' {
                total += 1;
            }
        }
        if (i + 3) < tree.len()
            && tree[i + 1][j] == 'M'
            && tree[i + 2][j] == 'A'
            && tree[i + 3][j] == 'S'
        {
            total += 1;
        }
        if i >= 3 && tree[i - 1][j] == 'M' && tree[i - 2][j] == 'A' && tree[i - 3][j] == 'S' {
            total += 1;
        }
    }
}
*/

pub fn day4() {
    let file = fs::read_to_string("./src/day4/day4.input.txt").unwrap();
    let mut total = 0;
    let tree = convert_to_matrix(file);
    for i in 0..tree.len() {
        for j in 0..tree[i].len() {
            if tree[i][j] == 'A' {
                if (i >= 1) && ((i + 1) < tree.len()) && (j >= 1) && ((j + 1) < tree[i].len()) {
                    let d1 = ((tree[i + 1][j + 1] == 'S' && tree[i - 1][j - 1] == 'M')
                        || (tree[i + 1][j + 1] == 'M' && tree[i - 1][j - 1] == 'S'));
                    let d2c1 = tree[i - 1][j + 1] == 'S' && tree[i + 1][j - 1] == 'M';
                    let d2c2 = tree[i - 1][j + 1] == 'M' && tree[i + 1][j - 1] == 'S';
                    let d2 = d2c1 || d2c2;
                    if (j == 6 && i == 2) {
                        println!("{:?}{:?} {:?}{:?}", d1, d2, i, j);
                    }
                    if d1 && d2 {
                        total += 1;
                    }
                }
            }
        }
    }
    println!("{:?}", total);
}

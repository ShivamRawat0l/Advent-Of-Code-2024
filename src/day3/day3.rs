use std::fs;

pub fn day3() {
    let file = fs::read_to_string("./src/day3/day3.input.txt").unwrap();
    let mut total = 0usize;

    let mut is_enabled = true;
    let mut string = String::from("");
    file.lines().for_each(|line| {
        line.chars().for_each(|word| {
            if word == 'd' || word == 'm' {
                string = "".to_string();
            }
            string.push(word);
            if string == "do()" {
                is_enabled = true;
                string = "".to_string();
            } else if string == "don't()" {
                is_enabled = false;
                string = "".to_string();
            } else if is_enabled && string.starts_with("mul(") && string.ends_with(")") {
                let numbers = string.split("mul(").collect::<Vec<_>>()[1]
                    .split(")")
                    .collect::<Vec<_>>()[0]
                    .split(",")
                    .collect::<Vec<_>>();
                if numbers.len() == 2 {
                    let number1 = numbers[0].parse::<usize>().unwrap_or(0);
                    let number2 = numbers[1].parse::<usize>().unwrap_or(0);
                    total = total + (number1 * number2)
                }
                println!("{:?}", total);
                string = "".to_string();
            }
        });
    });
    println!("{:?}", total);
}

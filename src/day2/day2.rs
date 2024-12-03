use std::fs;

pub fn day2() {
    let content = fs::read_to_string("./src/day2/day2.input.txt").unwrap();
    let mut ok = 0;
    let lines = content.split("\n");
    for line in lines {
        let mut isIncreasing: bool = false;
        let mut lastValue = -1;
        let mut isSecondDigit = false;
        let mut isValid = true;
        line.split(" ").for_each(|k| {
            let w = k.parse().unwrap();
            if lastValue == -1 {
                lastValue = w;
                isSecondDigit = true;
            } else {
                if ((lastValue - w) as i32).abs() > 3 {
                    isValid = false;
                } else if ((lastValue - w) as i32).abs() < 1 {
                    isValid = false;
                }
                if isSecondDigit {
                    isIncreasing = if lastValue > w { true } else { false };
                    isSecondDigit = false;
                } else {
                    if lastValue < w && isIncreasing {
                        isValid = false;
                    } else if lastValue > w && !isIncreasing {
                        isValid = false;
                    }
                }
                lastValue = w;
            }
        });
        if isValid {
            ok += 1;
        }
    }
    println!("{:?}", ok)
}

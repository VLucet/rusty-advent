use std::fs;

fn main() {
    let dec_1_1_input_path: String = "inputs/dec_1_1.txt".to_string();

    let dec_1_1_solution: u32 = solve_dec_1_1(&dec_1_1_input_path);
    println!("Maximum calories: {}", dec_1_1_solution);

    let dec_1_2_solution: u32 = solve_dec_1_2(&dec_1_1_input_path);
    println!("Maximum 3: {}", dec_1_2_solution);
}

fn solve_dec_1_1(file_path: &String) -> u32 {
    let file_contents = read_file(file_path);

    let mut tmp_sum: u32 = 0;
    let mut sum_max: u32 = 0;

    for line in file_contents.lines() {
        if line.eq("") {
            if tmp_sum > sum_max {
                sum_max = tmp_sum.clone();
            }

            tmp_sum = 0;
        } else {
            tmp_sum += line.parse::<u32>().unwrap();
        }
    }

    return sum_max;
}

fn solve_dec_1_2(file_path: &String) -> u32 {
    let file_contents = read_file(file_path);

    let mut tmp_sum: u32 = 0;
    let mut calories: Vec<u32> = vec![];

    for line in file_contents.lines() {
        if line.eq("") {
            calories.push(tmp_sum);
            tmp_sum = 0;
        } else {
            tmp_sum += line.parse::<u32>().unwrap();
        }
    }

    calories.sort_by(|a, b| b.cmp(a));

    let max_three = calories[0..3].iter().sum();

    return max_three;
}

fn read_file(file_path: &String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

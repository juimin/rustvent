use advent22::get_input_contents;

const TOP_N: usize = 3;

fn main() {
    let file_contents = get_input_contents();
    let lines = file_contents.split("\n");

    let mut current_calories: i32 = 0;
    let mut max_calories: i32 = 0;

    let mut top_values: Vec<i32> = Vec::new();

    for line in lines {
        if line.len() == 0 {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            top_values.push(current_calories);
            current_calories = 0;
        } else {
            let calories = line.to_string().parse::<i32>().expect("non integer input");
            current_calories += calories;
        }
    }

    top_values.sort();
    top_values.reverse();

    println!("Maximum Calories: {}", max_calories);

    let mut sum = 0;
    for index in 0..TOP_N {
        sum += top_values[index];
    }

    println!("Sum of top calories: {}", sum);
}

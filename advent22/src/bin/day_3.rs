use advent22::get_input_contents;

fn main() {
    let file_contents = get_input_contents();

    let mut priority_sum = 0;

    for line in file_contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let line_bytes = line.as_bytes();
        let mut compartment_one: [i32; 52] = [0; 52];
        let mut compartment_two: [i32; 52] = [0; 52];

        for i in 0..line.len() / 2 {
            let c = line_bytes[i] as char as usize;
            if c >= 65 && c <= 90 {
                compartment_one[c - 65 + 26] += 1;
            } else if c >= 97 && c <= 122 {
                compartment_one[c - 97] += 1;
            }
        }

        for i in line.len() / 2..line.len() {
            let c = line_bytes[i] as char as usize;
            if c >= 65 && c <= 90 {
                compartment_two[c - 65 + 26] += 1;
            } else if c >= 97 && c <= 122 {
                compartment_two[c - 97] += 1;
            }
        }

        // println!("{:?}", compartment_one);
        // println!("{:?}", compartment_two);

        for i in 0..52 {
            if compartment_one[i] > 0 && compartment_two[i] > 0 {
                priority_sum += i + 1;
            }
        }
    }

    println!("Sum of priorities for part 1: {}", priority_sum);
}

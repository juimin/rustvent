use advent22::get_input_contents;

fn populate_compartment_mapping(line: &[u8], lower: usize, upper: usize, compartment: &mut [i32]) {
    for i in lower..upper {
        let c = line[i] as char as usize;
        if c >= 65 && c <= 90 {
            compartment[c - 65 + 26] += 1;
        } else if c >= 97 && c <= 122 {
            compartment[c - 97] += 1;
        }
    }
}

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

        populate_compartment_mapping(line_bytes, 0, line_bytes.len() / 2, &mut compartment_one);
        populate_compartment_mapping(
            line_bytes,
            line_bytes.len() / 2,
            line_bytes.len(),
            &mut compartment_two,
        );

        for i in 0..52 {
            if compartment_one[i] > 0 && compartment_two[i] > 0 {
                priority_sum += i + 1;
            }
        }
    }

    println!("Sum of priorities for part 1: {}", priority_sum);
}

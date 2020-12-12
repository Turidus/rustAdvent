pub mod puzzle_one {

    use std::fs;
    use std::path::Path;

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");
        let mut lines = input.lines();

        let mut input: Vec<Vec<char>> = Vec::new();

        for line in lines {
            let chars = line.chars();
            let mut letters: Vec<char> = Vec::new();
            for letter in chars {
                letters.push(letter)
            }
            input.push(letters)
        }

        let mut index = 0;
        let index_max = input.first().unwrap().len();
        let mut count: u32 = 0;

        for line in input {
            let letter = line[index];
            if letter.to_string() == "#" {
                count += 1;
            }
            index = (index + 3) % index_max;
        }
        println!("Answer: {}", count);
    }
}

pub mod puzzle_two {

    use std::fs;
    use std::path::Path;

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");
        let mut lines = input.lines();

        let mut input: Vec<Vec<char>> = Vec::new();

        for line in lines {
            let chars = line.chars();
            let mut letters: Vec<char> = Vec::new();
            for letter in chars {
                letters.push(letter)
            }
            input.push(letters)
        }
        let input = input;
        let index_max = input.first().unwrap().len();
        let mut indices = [0, 0, 0, 0];
        let mut counts = [0, 0, 0, 0];

        for line in &input {
            for i in 0..4 {
                if line[indices[i]].to_string() == "#" {
                    counts[i] += 1;
                }
            }
            indices = [(indices[0] + 1) % index_max, (indices[1] + 3) % index_max, (indices[2] + 5) % index_max, (indices[3] + 7) % index_max];
        }

        let mut index = 0;
        let mut count = 0;

        for i in (0..input.len()).step_by(2) {
            let line = &input[i];
            if line[index].to_string() == "#" {
                count += 1;
            }
            index = (index + 1) % index_max;
        }
        println!("Counts: {}, {}, {}, {}, {}", counts[0], counts[1], counts[2], counts[3], count);
        for i in 0..4 {
            count = count * counts[i];
        }
        println!("Answer: {}", count);
    }
}
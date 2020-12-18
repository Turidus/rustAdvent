pub mod puzzle_one {
    use std::fs;
    use std::path::Path;
    use std::time::Instant;

    pub fn run(path: &Path){
        let now = Instant::now();
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let mut answer: u32 = 0;
        let mut main_list: Vec<u32> = Vec::new();

        for line in input.lines() {

            let number: u32 = line.parse().unwrap();

            if main_list.len() < 25 {
                main_list.push(number);
                continue
            }

            if check_if_rule_broken(&number, &main_list) {
                answer = number;
                break;
            }

            main_list.push(number);
            main_list.remove(0);
        }

        println!("The Answer is {}.", answer);
        println!("It took {} ms to get the Answer.", now.elapsed().as_millis())
    }

    fn check_if_rule_broken(number: &u32, list: &Vec<u32>) -> bool {

        let mut temp_list = list.clone();
        temp_list.sort();

        for i in 0..temp_list.len() {
            for j in 0..temp_list.len() {
                if i == j { continue }

                if temp_list[i] + temp_list[j] == *number {
                    return false
                }
                else if temp_list[i] + temp_list[j] > *number {
                    break
                }
            }
        }

        true
    }
}

pub mod puzzle_two {
    use std::fs;
    use std::path::Path;
    use std::time::Instant;

    pub fn run(path: &Path){
        let now = Instant::now();
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let num: usize = 85848519;
        let answer: usize;
        let mut main_list: Vec<usize> = Vec::new();

        for line in input.lines() {
            main_list.push(line.parse().unwrap());
        }

        let mut slice_borders: (usize,usize) = (0,main_list.len());
        let mut found = false;

        for i in 0..main_list.len() {
            if found {break}
            let mut acc: usize = 0;
            for j in i..main_list.len() {
                acc += main_list[j];

                if acc > num {break}
                else if acc == num {
                    found = true;
                    slice_borders = (i,j);
                    break
                }
            }
        }

        let slice = &mut main_list[slice_borders.0 .. slice_borders.1 + 1];
        slice.sort();

        answer = slice.first().unwrap() + slice.last().unwrap();

        println!("The Answer is {}.", answer);
        println!("It took {} ms to get the Answer.", now.elapsed().as_millis())
    }
}
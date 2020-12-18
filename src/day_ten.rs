pub mod puzzle_one {
    use std::fs;
    use std::path::Path;
    use std::time::Instant;

    pub fn run(path: &Path){
        let now = Instant::now();
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let mut answer: usize = 0;
        let mut main_list: Vec<usize> = Vec::new();

        for line in input.lines() {
            main_list.push(line.parse().unwrap())
        }

        main_list.sort();

        main_list.push(3 + main_list.last().unwrap());

        let mut last_num: usize = 0;
        let mut singles: usize = 0;
        let mut tripples: usize = 0;

        for num in main_list {
            if num - last_num == 1 {singles += 1}
            else if num - last_num == 3 {tripples += 1}
            else if num - last_num != 2 {
                println!("Difference was not 1, 2, 3: num = {}, last_num = {}", num, last_num);
            }

            last_num = num;
        }

        answer = singles * tripples;

        println!("The Answer is {}.", answer);
        println!("It took {} ms to get the Answer.", now.elapsed().as_millis())
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

        let mut answer: usize = 0;

        println!("The Answer is {}.", answer);
        println!("It took {} ms to get the Answer.", now.elapsed().as_millis())
    }
}
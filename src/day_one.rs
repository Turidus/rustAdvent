pub mod puzzle_one {
    use std::fs;
    use std::path::Path;
    use std::cmp::Ordering;

    #[derive(Debug)]
    struct Result {
        smaller: u32,
        bigger: u32,
    }
    impl Result{
        fn get(&self) -> u32 {
            self.smaller * self.bigger
        }
    }

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");
        println!("{}", input);

        let mut result: Result = Result {smaller: 1, bigger: 1};
        let lines = input.lines();
        let mut smaller:Vec<u32> = Vec::new();
        let mut bigger:Vec<u32> = Vec::new();
        let mut equal:Vec<u32> = Vec::new();

        for line in lines{
            let num: u32 = line.trim().parse().expect("Line was not a number");

            match num.cmp(&1010) {
                Ordering::Less => {smaller.push(num)}
                Ordering::Equal => {equal.push(num)}
                Ordering::Greater => {bigger.push(num)}
            }
        }
        smaller.sort();
        bigger.sort();
        println!("len small {:?}",smaller);
        println!("len big {:?}", bigger);



        if(equal.len() == 2){
            result.smaller = 1010;
            result.bigger = 1010;
        }
        else {
            let mut found = false;
            for small in smaller.iter(){
                if found {break};
                for big in bigger.iter(){
                    let ans = small + big;
                    match ans.cmp(&2020) {
                        Ordering::Less => {}
                        Ordering::Equal => {
                            result.smaller = *small;
                            result.bigger = *big;
                            found = true;
                            break
                            }
                        Ordering::Greater => {break}
                    }
                }
            }
        }

        println!("The numbers were: {},{}", result.smaller,result.bigger);
        println!("The answer is: {}", result.get());
    }
}

pub mod puzzle_two {
    use std::fs;
    use std::path::Path;
    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");
    }
}

pub mod puzzle_ups {
    use std::fs;
    use std::path::Path;
    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");
    }
}
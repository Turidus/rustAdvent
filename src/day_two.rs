pub mod puzzle_one {

    use std::fs;
    use std::path::Path;
    use regex::Regex;

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let re = Regex::new(r"(?P<l>\d+)-(?P<h>\d+)\s(?P<c>.):\s(?P<p>.+)").unwrap();

        let mut count: u32 = 0;

        for cap in re.captures_iter(&input){
            let low = cap.name("l").unwrap().as_str();
            let low: usize = low.parse().unwrap();

            let high = cap.name("h").unwrap().as_str();
            let high: usize = high.parse().unwrap();

            let char = cap.name("c").unwrap().as_str();
            let pattern = format!("({})", char);

            let re_2 = Regex::new(&pattern).unwrap();
            let pw = cap.name("p").unwrap().as_str();
            let captures = re_2.captures_iter(&pw);
            let captures_2 = re_2.captures_iter(&pw);
            println!(".......");
            println!("PW = {}", pw);
            for c in captures {
                println!("{:?}", c);
            }


            let times_found = captures_2.count();
            println!("Count = {}", times_found);
            if times_found >= low && times_found <= high {
                count += 1;
            }
        }
        println!("The answer is {}", count)
    }
}

pub mod puzzle_two {

    use std::fs;
    use std::path::Path;
    use regex::Regex;

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let re = Regex::new(r"(?P<l>\d+)-(?P<h>\d+)\s(?P<c>.):\s(?P<p>.+)").unwrap();

        let mut count: u32 = 0;

        for cap in re.captures_iter(&input){
            let low = cap.name("l").unwrap().as_str();
            let low: usize = low.parse::<usize>().unwrap() - 1;

            let high = cap.name("h").unwrap().as_str();
            let high: usize = high.parse::<usize>().unwrap() - 1;

            let char:char = cap.name("c").unwrap().as_str().parse().expect("not what i expected");
            let pw = cap.name("p").unwrap().as_str();

            let char_iter = pw.chars();

            let mut char_list: Vec<char> = Vec::new();
            for letter in char_iter {
                char_list.push(letter);
            }

            let mut hit = 0;

            if low < char_list.len() && char_list[low] == char {
                hit += 1;
            }
            if high < char_list.len() && char_list[high] == char {
                hit += 1;
            }

            if hit == 1 {
                count += 1;
            }
        }
        println!("The answer is {}", count)
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
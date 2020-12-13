pub mod puzzle_one {
    use std::fs;
    use std::path::Path;
    use regex::Regex;
    use std::collections::HashSet;

    pub fn run(path: &Path) {
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let re_blank = Regex::new(r"^\s*$").unwrap();
        let mut count = 0;
        let mut group = String::from("");

        for line in input.lines() {
            if !re_blank.is_match(line) {
                group.push_str(line);
            }
            else {
                count += get_count(&group);
                group = String::from("");
            }
        }
        if !re_blank.is_match(&group){
            count += get_count(&group);
        }
        println!("Answer is {}", count)
    }

    fn get_count(group: &String) -> usize{
        let mut set: HashSet<char> = HashSet::new();
        for s in group.chars() {
            set.insert(s);
        }
        set.len()
    }
}

pub mod puzzle_two {
    use std::fs;
    use std::path::Path;
    use std::collections::HashMap;
    use regex::Regex;

    pub fn run(path: &Path) {
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let re_blank = Regex::new(r"^\s*$").unwrap();
        let mut count = 0;
        let mut group = String::from("");
        let mut group_size: u32 = 0;
        for line in input.lines() {
            if !re_blank.is_match(line) {
                group.push_str(line);
                group_size += 1;
            }
            else {
                count += get_count(&group, group_size);
                group = String::from("");
                group_size = 0;
            }
        }
        if !re_blank.is_match(&group){
            count += get_count(&group, group_size);
        }
        println!("Answer is {}", count)
    }

    fn get_count(group: &String, group_size: u32) -> usize{
        let mut map: HashMap<char, u32> = HashMap::new();
        for s in group.chars() {
            if(map.contains_key(&s)){
                //save to unwrap, existence of key is already certain.
                let value = *map.get(&s).unwrap();
                map.insert(s, value + 1);
            }
            else {
                map.insert(s, 1);
            }
        }
        map.retain(|_,v| v == &group_size);
        map.len()
    }
}
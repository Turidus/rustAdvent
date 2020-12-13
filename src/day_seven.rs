pub mod puzzle_one {
    use std::fs;
    use std::path::Path;
    use regex::Regex;
    use std::collections::HashMap;

    #[derive(Debug, Hash, Eq)]
    struct Color {
        name: String
    }
    impl PartialEq for Color {
        fn eq(&self, b: &Self) -> bool{
            self.name == b.name
        }
    }

    #[derive(Debug)]
    struct Rule {
        color: Color,
        quantity: usize
    }

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let re_1: Regex = Regex::new(r"([a-z]+\s[a-z]+) bags contain (.+)").unwrap();


        let mut map: HashMap<Color, Vec<Option<Rule>>> = HashMap::new();

        for line in input.lines() {
            //we can use unwrap here. A non match should be a panik!
            let caps = re_1.captures(line).unwrap();
            let color: Color = Color{name: String::from(caps.get(1).unwrap().as_str())};
            let second_part = caps.get(2).unwrap().as_str();

            let rules: Vec<Option<Rule>> = get_rules(second_part);

            map.insert(color, rules);
        }
        println!("MapLength: {}", map.len());
        let mut count = 0;
        for key in map.keys() {
            if check_rules(key, &map) {count += 1;}
        }
        println!("Answer is {}", count)

    }

    fn get_rules(second_part: &str) -> Vec<Option<Rule>> {

        lazy_static! {
            static ref re_2: Regex = Regex::new(r"(\d)\s([a-z]+\s[a-z]+)").unwrap();
            static ref re_3: Regex = Regex::new(r"no other bags").unwrap();
        }
        //let re_2: Regex = Regex::new(r"(\d)\s([a-z]+\s[a-z]+)").unwrap();
        //let re_3: Regex = Regex::new(r"no other bags").unwrap();
        if re_3.is_match(second_part) {
            return vec![None]
        }
        let mut vec: Vec<Option<Rule>> = Vec::new();
        for cap in re_2.captures_iter(second_part){
            //we can use unwrap here. A non match should be a panik!
            let quantity: usize = cap.get(1).unwrap().as_str().parse().unwrap();
            let name: String = String::from(cap.get(2).unwrap().as_str());

            let color = Color{name};
            let rule = Rule{quantity,color};
            vec.push(Some(rule));
        }

        vec
    }

    fn check_rules(color: &Color, map: &HashMap<Color, Vec<Option<Rule>>>) -> bool {

        if color.name.as_str() == "shiny gold" {
            println!("shiny gold");
            return true
        }

        let rules = map.get(color).unwrap();


        for rule in rules {
            match rule {
                None => {return false}
                Some(c) => {
                    if check_rules(&c.color, map) {
                        println!("Color: {:?}", c.color);
                        return true
                    };
                }
            }
        }
        false
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
pub mod puzzle_one {
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;
    use std::io::{Error, ErrorKind};
    use std::convert::TryFrom;

    #[derive(Debug)]
    struct Instruction {
        op: Operation,
        value: i32,
        visited: bool
    }

    #[derive(Debug)]
    enum Operation {
        Jmp,
        Acc,
        Nop
    }
    impl FromStr for Operation {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let op: Operation;
            let e = Error::new(ErrorKind::InvalidInput, "Could not parse this line");
            if s == "jmp" {
                op = Operation::Jmp;
            }
            else if s == "acc" {
                op = Operation::Acc;
            }
            else if s == "nop" {
                op = Operation::Nop;
            }
            else {
                return Err(e);
            }

            Ok(op)
        }
    }

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let mut instructions: Vec<Instruction> = Vec::new();

        for line in input.lines() {
            let mut split = line.split_ascii_whitespace();

            let op = Instruction {
                op: split.next().unwrap().parse().unwrap(),
                value: split.next().unwrap().parse().unwrap(),
                visited: false
            };
            instructions.push(op);
        }

        println!("Answer is {}", get_last_acc(instructions))
    }

    fn get_last_acc(mut instructions: Vec<Instruction>) -> i32 {

        let mut index: usize = 0;
        let mut acc = 0;

        loop {
            if instructions[index].visited {
                break
            }
            instructions[index].visited = true;

            match instructions[index].op {
                Operation::Jmp => {
                    let tempdex: i32 = instructions[index].value + index as i32;
                    if tempdex < 0  {
                        panic!("Index out of bounds: {}", tempdex)
                    }
                    else if tempdex > instructions.len() as i32 {
                        break //Terminated
                    }
                    index = usize::try_from(tempdex).unwrap();
                }
                Operation::Acc => {
                    acc += instructions[index].value;
                    index += 1;
                }
                Operation::Nop => {
                    index += 1;
                }
            }
        }
        acc
    }
}

pub mod puzzle_two {
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;
    use std::io::{Error, ErrorKind};
    use std::convert::TryFrom;
    use std::time::Instant;

    #[derive(Debug, Copy, Clone)]
    struct Instruction {
        op: Operation,
        value: i32,
        visited: bool
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Operation {
        Jmp,
        Acc,
        Nop
    }
    impl FromStr for Operation {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let op: Operation;
            let e = Error::new(ErrorKind::InvalidInput, "Could not parse this line");
            if s == "jmp" {
                op = Operation::Jmp;
            }
            else if s == "acc" {
                op = Operation::Acc;
            }
            else if s == "nop" {
                op = Operation::Nop;
            }
            else {
                return Err(e);
            }

            Ok(op)
        }
    }

    pub fn run(path: &Path){
        let now = Instant::now();
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let mut instructions: Vec<Instruction> = Vec::new();

        for line in input.lines() {
            let mut split = line.split_ascii_whitespace();

            let op = Instruction {
                op: split.next().unwrap().parse().unwrap(),
                value: split.next().unwrap().parse().unwrap(),
                visited: false
            };
            instructions.push(op);
        }

        for index in 0..instructions.len() {
            if instructions[index].op == Operation::Acc || instructions[index].value == 0 {
                continue
            }
            let mut temp_list = instructions.to_vec();

            if instructions[index].op == Operation::Jmp {
                temp_list[index].op = Operation::Nop;
            }
            else {
                temp_list[index].op = Operation::Jmp;
            }
            let (acc, term) = get_last_acc(temp_list);
            if term {
                println!("Answer is {} on index {}", acc, index);
                break
            }
        }
        println!("Getting the answer took {} ms", now.elapsed().as_millis())
    }

    fn get_last_acc(mut instructions: Vec<Instruction>) -> (i32,bool) {

        let mut index: usize = 0;
        let mut acc = 0;
        let mut term = false;

        loop {
            if instructions[index].visited {
                break
            }
            instructions[index].visited = true;

            match instructions[index].op {
                Operation::Jmp => {
                    let tempdex: i32 = instructions[index].value + index as i32;
                    if tempdex < 0 || tempdex > instructions.len() as i32 {
                        panic!("Index out of bounds: {}", tempdex)
                    }
                    else if tempdex == instructions.len() as i32 {
                        term = true;
                        break //Terminated
                    }
                    index = usize::try_from(tempdex).unwrap();
                }
                Operation::Acc => {
                    acc += instructions[index].value;
                    index += 1;
                }
                Operation::Nop => {
                    index += 1;
                }
            }
        }
        (acc, term)
    }

}

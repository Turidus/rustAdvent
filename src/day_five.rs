pub mod puzzle_one {
    use std::fs;
    use std::path::Path;

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let mut free_seats: Vec<i32> = Vec::new();

        for i in 0..128 {
            for j in 0..8{
                free_seats.push(i * 8 + j)
            }
        }
        let count = input.lines().count();
        println!("count {}", count);

        for line in input.lines(){

            let row = &line[..7];
            let column = &line[7..];

            let row = decode_row(row);
            let column = decode_column(column);

            let seat_id = row * 8 + column;

            free_seats[seat_id as usize] = -1;
        }

        println!("The Answer (-1 are full seats): {:?}", free_seats);


        for i in free_seats {
            if i != -1 {
                //println!("Answer is {}", i)
            }
        }

    }

    fn decode_row(code: &str) -> u32{
        let mut low: u32 = 0;
        let mut high: u32 = 127;
        let mut result: u32 = 128;

        for s in code.chars() {
            let half = (high - low) / 2;
            if half != 0 {
                if s == 'F' {
                    high = low + half;
                }
                else if s == 'B' {
                    low = high - half;
                }
                else {
                    panic!("Only F or B allowed, was {}", s) }
            }
            else {
                if s == 'F' {
                    result = low;
                }
                else if s == 'B' {
                    result = high;
                }
                else {
                    panic!("Only F or B allowed, was {}", s)
                }
            }
        }
        result
    }

    fn decode_column(code: &str) -> u32{
        let mut low: u32 = 0;
        let mut high: u32 = 7;
        let mut result: u32 = 8;

        for s in code.chars() {
            let half = (high - low) / 2;
            if half != 0 {
                if s == 'L' {
                    high = low + half;
                }
                else if s == 'R' {
                    low = high - half;
                }
                else { panic!("Only L or R allowed, was {}", s) }
            }
            else {
                if s == 'L' {
                    result = low;
                }
                else if s == 'R' {
                    result = high;
                }
                else { panic!("Only L or R allowed, was {}", s) }
                break;
            }
        }
        result
    }
}
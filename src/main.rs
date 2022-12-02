use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut bank: i32 = 0;
    let mut elves: Vec<i32> = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        println!("file opened");
        for line in lines {
            if let Ok(ip) = line {
                if ip == ""{
                    elves.push(bank);
                    bank = 0;
                }else{
                    println!("adding");
                    bank += ip.parse::<i32>().unwrap()
                }
            }
        }
    }
    let answer: i32 = *elves.iter().max().unwrap();
    println!("{}",answer);

    let mut topThree: i32 = answer;
    elves.retain(|&x| x != answer);
    let sto: i32 = *elves.iter().max().unwrap();
    topThree += sto;
    elves.retain(|&x| x != sto);
    let sto: i32 = *elves.iter().max().unwrap();
    topThree += sto;

    println!("second answer: {}",topThree);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
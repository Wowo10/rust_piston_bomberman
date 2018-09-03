use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_map(map_name: &'static str) -> Vec<Vec<char>> {
    let f = File::open(map_name).expect("map_file not found");

    let reader = BufReader::new(f);

    let mut return_vector: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let mut temp: Vec<char> = Vec::new();
        for character in line.expect("map read problem").chars(){
            if character != ';'{
                temp.push(character);
            }
        }
        return_vector.push(temp);
    }

    return_vector
}

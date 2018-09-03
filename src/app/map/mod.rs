use std::fs::File;
use std::io::{BufRead, BufReader};

// pub struct Map {
//     map: String,
//     cursor: usize,
// }

// impl Map {
//     pub fn create(map_name: &'static str) -> Self {
//         let mut f = File::open(map_name).expect("map_file not found");

//         let mut contents = String::new();
//         f.read_to_string(&mut contents)
//             .expect("something went wrong reading the map_file");

//         Map {
//             map: contents,
//             cursor: 0,
//         }
//     }

//     fn get_current(&self) -> &str {
//         let cursor = &self.cursor;

//         &self.map[*cursor..*cursor + 1]
//     }

//     pub fn read_next_tile(&mut self) -> &str {
//         let temp = self.get_current();

//         if temp == ";" {
//             self.get_current()
//         } else {
//             temp
//         }
//     }
// }

pub fn read_map(map_name: &'static str) -> Vec<Vec<char>> {
    let f = File::open(map_name).expect("map_file not found");

    let reader = BufReader::new(f);

    let mut return_vector: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let mut temp: Vec<char> = Vec::new();
        for character in line.expect("bug").chars(){
            if character != ';'{
                temp.push(character);
            }
        }
        return_vector.push(temp);
    }

    return_vector
}

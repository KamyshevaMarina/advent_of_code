use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("filesystem error");
    let mut s: String = String::new();
    // читаем файл в строку
    f.read_to_string(&mut s).expect("io error");
    // делим строку по \n и собираем в вектор
    let mut ids = s.split('\n').collect::<Vec<&str>>();
    ids.pop().unwrap();
    let mut map: HashMap<char, u8> = HashMap::new();
    let mut entry;
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;
    for i in &ids {
        for c in i.chars() {
            entry = map.entry(c).or_insert(0u8);
            *entry += 1;
        }
        match map.values().find(|&&i| i == 3u8) {
            Some(_) => threes += 1,
            None => (),
        }
        match map.values().find(|&&i| i == 2u8) {
            Some(_) => twos += 1,
            None => (),
        }
        map.clear();
    }

    println!("hashsum is: {}", twos * threes);
}

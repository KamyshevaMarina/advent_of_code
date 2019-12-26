use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("filesystem error");
    let mut s: String = String::new();
    // читаем файл в строку
    f.read_to_string(&mut s).expect("io error");
    // делим строку по \n и собираем в вектор
    let mut numbers = s
        .split('\n')
        .map(|i| i.parse().unwrap_or(0))
        .collect::<Vec<i32>>();
    numbers.pop().unwrap(); // pop пустую строку в конце
    println!(
        "frequency drift: {}",
        numbers.iter().fold(0, |acc, i| acc + i)
    );
}

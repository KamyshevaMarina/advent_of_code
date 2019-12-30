fn main() {
    let mut s = std::fs::read_to_string("data.txt").unwrap();
    s.pop();
    println!(
        "answer: {}",
        s.split('\n')
            .map(|i| i.parse::<i32>().unwrap())
            .fold(0, |acc, i| acc + i)
    );
}

mod d1;

fn main() {
    let count = d1b();
    println!("Day 1 b: {}", count);
}

fn d1a() -> u16 {
    let sonar = d1::Sonar::new("data/d1.txt");

    sonar.count_increases()
}

fn d1b() -> u16 {
    let sonar = d1::Sonar::new("data/d1.txt");

    sonar.count_window_increases()
}
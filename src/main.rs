mod d1;
mod d2;

fn main() {
    let result = d2b();
    println!("d2b: {}", result);
}

fn d1a() -> u16 {
    let sonar = d1::Sonar::new("data/d1.txt");

    sonar.count_increases()
}

fn d1b() -> u16 {
    let sonar = d1::Sonar::new("data/d1.txt");

    sonar.count_window_increases()
}

fn d2a() -> u32 {
    let instructions = d2::Instructions::new("data/d2.txt");

    let (x, z) = instructions.process();

    x as u32 * z as u32
}

fn d2b() -> u32 {
    let instructions = d2::Instructions::new("data/d2.txt");

    let (x, z) = instructions.adv_process();

    x as u32 * z as u32
}
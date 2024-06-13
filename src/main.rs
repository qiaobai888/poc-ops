use std::time::Instant;

fn main() {
    let mut count: u128 = 0;

    let stamp = Instant::now();
    while count <= 110_000_000 {
        count += 1;
    }
    let end = stamp.elapsed();

    println!("{:?}", end);
}

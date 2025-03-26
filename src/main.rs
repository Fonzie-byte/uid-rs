use chrono::Utc;
use rand::Rng;

fn main() {
    let timestamp = Utc::now().timestamp();
    let random: u32 = rand::rng().random_range(0..=0xFFFF_FFFF);

    println!("{:0>8x}{:0>8x}", timestamp, random);
}

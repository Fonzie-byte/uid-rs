use chrono::Utc;
use rand::Rng;

fn main() {
    // Timestamp, 4 bytes long (although an i64 in Rust)
    let timestamp = Utc::now().timestamp();
    // 32 bits / 4 bytes of cryptographically secure random data
    let random: u32 = rand::rng().random_range(0..=0xFFFF_FFFF);

    // Print each as up to 8 hexadecimal chars; 16 total
    println!("{:0>8x}{:0>8x}", timestamp, random);
}

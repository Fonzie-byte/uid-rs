use chrono::Utc;
use rand_core::{OsRng, TryRngCore};

fn main() {
    // Timestamp, 4 bytes long (although an i64 in Rust)
    let timestamp = Utc::now().timestamp();
    // 32 bits / 4 bytes of cryptographically secure random data
    let random = OsRng.try_next_u32().unwrap();

    // Print each as up to 8 hexadecimal chars; 16 total
    println!("{:0>8x}{:0>8x}", timestamp, random);
}

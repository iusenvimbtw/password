
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::io::{self, Write};
use std::process::Command;

// Type alias for convenience
type HmacSha256 = Hmac<Sha256>;

fn main() {
    let mut input = String::new();
    print!("Enter a string to hash: ");
    io::stdout().flush().unwrap(); // Make sure the prompt is displayed

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Remove any trailing newline characters

    // Secret key (make sure this is kept secret and secure)
    let secret_key = b"cant tell u my secret mate"; // Replace with your secret key

    // Create HMAC-SHA256 instance
    let mut mac = HmacSha256::new_from_slice(secret_key).expect("Invalid key length");
    mac.update(input.as_bytes());

    // Compute the hash
    let result = mac.finalize();
    let hash_bytes = result.into_bytes();
    let hash_string = format!("{:x}", hash_bytes);

    // Copy hash to clipboard using CopyQ
    let output = Command::new("copyq")
        .arg("copy")
        .arg(&hash_string)
        .output()
        .expect("Failed to execute copyq");

    if !output.status.success() {
        eprintln!("Failed to copy to clipboard: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("HMAC-SHA256 hash: {}", hash_string);
        println!("Hash copied to clipboard.");
    }
}

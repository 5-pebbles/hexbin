use clap::Parser;
use regex::Regex;

use num_bigint::BigUint;
use num_traits::Num;

/// A command-line utility to travel between the realms of hexadecimal and binary
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hexadecimal or binary value to transmute
    #[clap(index = 1, required = true)]
    value: String,
}

fn main() {
    let value = Args::parse().value;

    let re_bin = Regex::new(r"^[01]+$").unwrap();
    let re_hex = Regex::new(r"^(0x)?([0-9A-Fa-f]+)$").unwrap();

    let result = if re_bin.is_match(&value) {
        bin_to_hex(&value)
    } else if re_hex.is_match(&value) {
        hex_to_bin(&value)
    } else {
        "Please enter a valid hexadecimal or binary string.".to_string()
    };

    println!("{}", result);
}

/// Convert a hexadecimal string to a binary string
fn hex_to_bin(hex: &str) -> String {
    let value = BigUint::from_str_radix(&hex.strip_prefix("0x").unwrap_or(hex), 16).unwrap();
    format!("{:b}", value)
}

/// Convert a binary string to a hexadecimal string
fn bin_to_hex(bin: &str) -> String {
    let value = BigUint::from_str_radix(bin, 2).unwrap();
    format!("0x{:X}", value)
}

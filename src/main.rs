#![cfg_attr(feature = "cargo-clippy", allow(useless_format))]

#[macro_use]
mod tools;
use tools::rlog;

fn main() {
    log!("Hello, world!");
}

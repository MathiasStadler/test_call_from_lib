pub const GREETING: &'static str = "src > lib.rs > GREETINGS => Hallo, Rust library here!";

pub fn foo() {
    println!("src > lib.rs > Hallo, Rust library here!")
}

pub mod bar;
pub mod foo;
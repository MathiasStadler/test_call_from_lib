/*use test_call_from_lib::GREETING;

fn main() {
    println!("{}", GREETING);
}
*/

/*
use test_call_from_lib::bar::bar;

fn main() {
    bar();
}
*/
use utils::bar::bar;

mod utils;

fn main() {
    bar();
}
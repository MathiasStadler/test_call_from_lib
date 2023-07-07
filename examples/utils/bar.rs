// use crate::foo;
use super::foo;
// or use crate::utils::foo;
pub fn bar() {
    println!("examples > utils > bar(bar.rs)");
    foo::say_foo();
}

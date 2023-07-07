use crate::foo;

pub fn bar() {
    println!("src > bar(bar.rs)");
    foo::say_foo();
}

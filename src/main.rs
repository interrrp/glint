use crate::check::check_all;

mod check;

fn main() {
    let text = "Hello,\tworld!";
    for err in check_all(text) {
        println!("{}", err);
    }
}

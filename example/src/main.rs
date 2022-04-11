#[macro_use]
extern crate named_tup;

fn main() {
    // let count = 5;
    let mut item = tup!(count: "Count");

    item.count = "Yum";

    let item = item + tup!(banana: 42345);

    let test = item + tup!(lol: "Lol", count: "Hi");
    assert_eq!(test, tup!(count: "Yum", lol: "Lol", banana: 42345));
}

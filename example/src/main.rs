#[macro_use]
extern crate named_tup;

fn main() {
    // let count = 5;

    let mut item = tup!(count: "Count");
    // assert_eq!(item.count, count);
    // assert_eq!(item.banana, "Banana");

    item.count = "Yum";

    let item = tup!(
        count: item.count,
        banana: "42345"
    );
    println!("{}", item.banana);
    assert_eq!(item.count, "Yum");
}

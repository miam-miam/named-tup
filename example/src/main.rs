#[macro_use]
extern crate named_tup;

fn main() {
    let count = tup!(count: 5);
    let mut count = test(count);

    count.count = 42345;

    let banana = 2;

    let item = count + tup!(banana, count: 5);

    let test = item + tup!(lol: "Lol", count: 8);
    println!("{test:?}");
    println!("{:?}", tup!(lol: "Yap"));
    assert_eq!(test, tup!(count: 8, lol: "Lol", banana: 2));
}

fn test(arg: tup!(count: i32)) -> tup!(count: i64) {
    tup!(count: arg.count.into())
}

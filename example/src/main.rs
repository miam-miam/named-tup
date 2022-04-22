use named_tup::{tup, tup_default, TupInto};

fn main() {
    let count = tup!();
    let mut count = test(count.into_tup());

    println!("{count:?}");

    count.count = 42345;

    let banana = 2;

    let item = count + tup!(banana, count: 5);

    let test = item + tup!(lol: "Lol", count: 8);
    println!("{test:?}");
    println!("{:?}", tup!(lol: "Yap"));
    assert_eq!(test, tup!(count: 8, lol: "Lol", banana: 2));
}

#[tup_default]
fn test(arg: tup!(count: i32 = 4)) -> tup!(count: i64 = 3) {
    tup!(count: <i64>::from(arg.count)).into_tup()
}

use named_tup::tup;

mod test;

fn main() {
    // let count = 5;
    // // This will have the type of tup!(count: i32, ingredients: [&str; 3], eggs: bool)
    // let cakes = tup!(count, ingredients: ["milk", "flower", "sugar"], eggs: true);
    //
    // // We can just add a price afterwards
    // let mut cakes = cakes + tup!(price: 3);
    // // And now it has the type of tup!(eggs: bool, ingredients: [&str; 3], count: i32, price: i32)
    //
    // // Once the price is in the tup we can just update it!
    // cakes.price = 4;
    //
    // // Will print tup { count: 5, eggs: true, ingredients: ["milk", "flower", "sugar"], price: 4 }
    // println!("{cakes:?}");
    let test = tup!(ingredients: ["milk", "flower", "sugar"]);
    let test2 = tup!(count: 5);
}

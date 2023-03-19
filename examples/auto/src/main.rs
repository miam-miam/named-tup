use named_tup::tup;

fn main() {
    let count = 5;

    // This will have the type of Tup!(count: i32, ingredients: [&str; 3], eggs: bool)
    let cakes = tup!(count, ingredients: ["milk", "flower", "sugar"], eggs: true);

    // We can just add a price afterwards
    let mut cakes = cakes + tup!(price: 3);
    // And now it has the type of Tup!(eggs: bool, ingredients: [&str; 3], count: i32, price: i32)

    // Once the price is in the tup we can just update it!
    cakes.price = 4;

    // Will print tup { count: 5, eggs: true, ingredients: ["milk", "flower", "sugar"], price: 4 }
    println!("{cakes:?}");
}

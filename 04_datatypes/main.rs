fn main() {

    let mut x:u32 = 20;
    x = x + 30;
    // x = x - 30; at compile time will result in a panic 
    // because an unsigned integer cannot be a negative number, a signed iteger can be as shown below by y.
    let mut y:i32 = 20;
    y = y - 30;

    // Number Literals in Rust
    // Decimal = 98_222;
    // Hex = 0xff;
    // Octal = 0o77;
    // Binary = 0b111_0000;
    
    println!("This is the value of the unsigned integer: {}", x);
    println!("This is the value of the signed integer: {}", y);

    // Bool types work the same as in other languages
    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // Rust also supports character types
    // Character types are denoted with single quotes
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("These are character types: {} {} {}", c, z, heart_eyed_cat);

    // Compound Types in Rust
    // Tuples are fixed in length and group together a variety of types into one compound type.
    // Inorder to access the values a tup must be destructured.
    let tup = ("Fueztech is cool!", 100_000);
    let (website, visitors,) = tup;
    let visitors = tup.1;

    // Array Type is fixed in length, holds one datatype, and will not grow or shrink in size.
    let tree = ["Oak", "Acacia", "Birch", "Elm"];
    let tree2 = tree[1];
    //let tree3 = tree[5]; This throws an error because it is out of scope of the array.
    let tree3 = tree[2];
    println!("The value of the trees variables is: {} and {}", tree2, tree3);
    
    println!("The value of y is: {}", y);
}

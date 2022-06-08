fn main() {
    let mut x = 5;
    println!("The value of x is: {} and the address of {:p}", x, &x);
    x = 8;
    println!("The value of x is: {} and the address of {:p}", x, &x);
    
    // Constants are great for hardcoded values and are not mutable. 
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("This is what a constant looks like {}", THREE_HOURS_IN_SECONDS,);

    let y = 12;

    let y = y + 3;
    
    // Shadowing is not merely muting a variable. Inner scope displays thid below. { }
    // Shadowing allows you to use the same name of a variable and repeat the use of the let keyword
    
    {
        let y = y * 4;
        println!("The value of y in the inner scope is: {} address: {:p}", y, &y);
    }

println!("The value of y is: {} address: {:p}", y, &y);
}


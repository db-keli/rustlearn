const NUM: u32 = 100_000;

fn main() {
    // Undersctanding Immutability
    {
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x now is: {}", x);
        let z;
        z = x + NUM;
        println!("The constant + x is: {}", z);
    }

    //Shadowing
    {
        let x = 5;
        println!("The value of x is: {}", x);
        let x = 5 + 1;
        println!("The value of x here is: {}", x);
        let x = 5 + 2;
        println!("The value of x here is: {}", x);
    }

    //Tuples
    {
        let tup = (23, -32, 0.56);

        let (x, y, z) = tup;

        println!("The tuple values are: {}",tup);
    }
}

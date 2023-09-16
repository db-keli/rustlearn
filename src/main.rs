const NUM: u32 = 100_000;

fn main() {
    // Understanding Immutability
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
        let tup = (-23, -32, 0.56);

        let (x, y, z) = tup;

        println!("The first value in the tuple is: {}", x);
        println!("The second value in the tuple is: {}", y);
        println!("The third value in the tuple is: {}", z);

        let about = ("Michael", "Attakora", "Adusei");

        let first_name = about.0;
        let middle_name = about.1;
        let last_name = about.2; 

        println!("The first name is {}, second name is {} and last name is {}", first_name,
            middle_name, last_name);
    }

    //Arrays
    {
        let arr:[i32; 7] = [1, 2, 3, 4, 5, 6, 7];

        // let first = arr[0];
        // let second = arr[1];

        println!("the firs element of the array is: {}", arr[0]);
    }
    
    //Functions
    another_function(5);

    let add_result = add_numbers(6, 4);
    println!("The result of the addition is: {}", add_result);

    // Ownership
    {
        let x =5;
        let y = x; // x Copied into y 

        println!("{}", y);

        let s1 = String::from("Hello");
        let s2 = s1.clone(); // s1 moved onto s2

        println!("s1: {}", s1);
        println!("s2: {}", s2);


        let s = String::from("Hello");
        let d = 5;

        takes_ownership(s); // Ownership of s moves into the function
                                        // so s is no longer valid and calling it will throw 
                                        // an error
        
        makes_copy(d); // Copies d into some_integer so d is still valid and 
                                    // and can be called after
    }

}

fn another_function(x:i32){
    println!("The value you typed + 5 is {}", x+5);
}

fn add_numbers(x: i32, y: i32 ) -> i32 {
    return x + y;
}

fn takes_ownership (some_string: String) {
    println!("{}", some_string);
}

fn makes_copy (some_integer: i32 ) {
    println!("{}", some_integer);
}

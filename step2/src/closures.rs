pub fn test_closures(){
    let add  = |x, y|{
        println!("Returning some text: {0} {1}", x, y);
    };
    add(-3, 8);
}
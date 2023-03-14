pub fn run(){
   /* Scope */
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    let s1= String::from("hello");
    let s2=s1;  // Rust Considers S1 has no longer valid, not a shallow copy of s1 but a move: s1 was moved into s2.  uses HEAP.

    println!("{s1}");

    let x=5;
    let y=x; // x is still valid here, because it is a primitive type, not a pointer to a string uses STACK


    let str=String::from("New String");

    take_ownership(str);  // str's value moves into the function...
    // ... and so is no longer valid here

    let x1=32; // 
    makes_copy(x1);  // x1 would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x1 afterward



    /* Borrowing */
    let mut str1=String::from("Borrow");

    let mut r1=&str1; // Don't create a  
     

}

/* String -> Heap */
fn take_ownership(some_string: String){
    println!("This is {}",some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.


/* Int -> Stack */
fn makes_copy(some_integer:i32){
    println!("This is {some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn first_word(s: &String)-> usize{
    
}
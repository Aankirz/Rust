# Rust

## Using Crate


### Ownership Rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

<h6>To illustrate the rules of ownership, we need a data type that is more complex than those we covered
 - data that is stored on the heap and explore how Rust knows when to clean up that data, and the <b>String</b> type is a great example.
 - Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
 -  When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
 - In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.



</h6>
<h4>
 Thus to solve the problem of transferring ownership Rust has a feature of using value without transferring ownership, called references.
</h4>

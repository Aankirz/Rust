# Rust
- Rust doesn't allow null values, so there is no null pointer exception.
- Rust doen't allow  us to mark only certain fields as mutable. 
- Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute #[derive(Debug)] just before the struct definition, 
- Another way to print out a value using the Debug format is to use the dbg! macro,

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


### Structs
- Structs are custom data types that let you name and package together multiple related values that make up a meaningful group.
- Unlike tuples, you’ll name each piece of data so it’s clear what the values mean. As a result of these names, structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.
- To access the fields of a struct, we use dot notation.
-But with structs, the way println! should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of Display to use with println! and the {} placeholder.
-  help: the trait `std::fmt::Display` is not implemented for `Rectangle`
-  note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

### Methods
- Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object, and we’re going to cover enums and trait objects in Chapters 6 and 17, respectively), and their first parameter is always self, which represents the instance of the struct the method is being called on.

### Generic <T,E> (How to define your own types, functions, and methods with generics!)
- First, we’ll review how to extract a function to reduce code duplication & how to use generic types in struct and enum definitions.
- how to use generic types in struct and enum definitions.
- how to use traits to define behavior in a generic way (to constrain a generic type to accept only those types that have a particular behavior, as opposed to just any type.)
- lifetimes: a variety of generics that give the compiler information about how references relate to each other, Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.

<h6>To use the same function for different type of arguments we use Generics
 - The good news is that using generic types won't make your program run any slower than it would with concrete types.

- Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
</h6>


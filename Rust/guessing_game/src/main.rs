use std::io; //input/output library into scope
use rand::Rng;

fn main() {
    println!("Guess a no");

    let rand_no=rand::thread_rng().gen_range(1..100);
    println!("The random no is {rand_no}");
    let mut guess=String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

   println!("You guessed {guess}");
   if rand_no == 4{
    println!("");
   }

   let _y={
    let x=3;
    x+1 // no ; when you are returning a value.
   };


   let x = plus_one(5);

   println!("The value of x is: {x}");

}

fn plus_one(x:i32)->i32{
    x+1
}

/*
The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. 

Result’s variants are Ok and Err.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

Updating a Crate to Get a New Version
When you do want to update a crate, Cargo provides the command update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file. Otherwise, by default, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0. If the rand crate has released the two new versions 0.8.6 and 0.9.0, you would see the following if you ran cargo update:


Note: You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the cargo doc --open command will build documentation provided by all your dependencies locally and open it in your browser. If you’re interested in other functionality in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left.

mut and shadowing main differences , u can't change the type of the value.
 */
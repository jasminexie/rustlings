// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// Shadowing is different from marking a variable as mut, because we’ll get a compile-time error
// if we accidentally try to reassign to this variable without using the let keyword.
// By using let, we can perform a few transformations on a value but have the variable be
// immutable after those transformations have been completed.

fn main() {
    let number = "T-H-R-E-E";
    println!("Spell a Number : {}", number);
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}

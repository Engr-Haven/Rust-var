fn main() {
    let x = 2;
    let x = x + 10;

    {
        let x = x * 3;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
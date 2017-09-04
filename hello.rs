fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");

    let x = [1, 16];
    let y = x.iter();
    let foos = y.map(|&a| a+1).collect::<Vec<i32>>();
    
    println!("functional style: {:?}", foos);
}
use std::time::Duration;
use std::thread;

fn main() {

    let mut x = 1;
    println!("Hello, Rusty-fuzzers! Let's begin...");
    println!("------------------------------------");
    thread::sleep(Duration::from_millis(1000));
    
    while x != 101 
    {
    
        if x % 3 == 0 || x % 5 == 0
        {
            if x % 3 == 0 && x % 5 == 0
            {
                println!("RustyFuzz!");
                x += 1;
            }
            
            if x % 3 == 0
            {
                println!("Rusty!");
                x += 1;
            } else {
                println!("Fuzz!"); 
                x +=1 ;
            }
        } else {
            println!("{}", x);
            x += 1;
        }
    }
}
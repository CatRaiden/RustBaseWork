mod second_module;

fn main() {
    println!("Hello, CatRaiden world!");

    //if
    let number = 3;
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    //other function
    other_function();

    //while
    let mut i = 0;
    while i < 10 {
        i+=1; 
        println!("i number {}", i);       
    }


    second_module::other_function();
}

fn other_function() {
    println!("here is other function");
}
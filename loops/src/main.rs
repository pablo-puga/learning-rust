//
// This is an example of using a loop to generate a value.
// The expression indicated after the break statement is assigned to the variable.
//
fn loop_with_result() {
    println!(">> Started running loop_with_result");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!(">> Ended loop_with_result");
}

fn loop_with_labels() {
    println!(">> Started running loop_with_labels");
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!(">> Ended loop_with_labels");
}

fn loop_with_condition() {
    println!(">> Started running loop_with_condition");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!(">> Ended loop_with_condition");
}

fn loop_collection_with_for() {
    println!(">> Started running loop_collection_with_for");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    println!(">> Ended loop_collection_with_for");
}

fn loop_range() {
    println!(">> Started running loop_range");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    println!(">> Ended loop_range");
}

fn main() {
    loop_with_result();
    println!("");
    loop_with_labels();
    println!("");
    loop_with_condition();
    println!("");
    loop_collection_with_for();
    println!("");
    loop_range();
}

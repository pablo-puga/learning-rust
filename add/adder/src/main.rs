use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} pus one is {}!", add_one::add_one(num));
    println!("{num} plus random is {}!", add_one::add_rand(num));
}

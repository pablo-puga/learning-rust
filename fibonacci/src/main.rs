const SERIES_LENGTH: u32 = 20;

fn main() {
    let mut value = 1;
    let mut prev_value = 1;

    print!("Fibonacci of {} items: {prev_value}", SERIES_LENGTH);

    for _length in 1..SERIES_LENGTH {
        let current_value = value;
        print!(" {current_value}");

        value = current_value + prev_value;
        prev_value = current_value;
    }

    print!("\n");
}

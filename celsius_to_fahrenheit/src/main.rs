fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    let temperatures: [f64; 5] = [-40.0, -20.0, 0.0, 10.0, 30.0];

    for celsius in temperatures {
        let fahrenheit = celsius_to_fahrenheit(celsius);

        println!("{celsius}ºC = {fahrenheit}ºF");
    }
}

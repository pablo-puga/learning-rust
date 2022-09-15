fn main() {
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let presents = [
        "A partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day_index in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            ordinals[day_index]
        );

        for present_index in (0..day_index + 1).rev() {
            println!("{}", presents[present_index]);
        }

        println!();
    }
}

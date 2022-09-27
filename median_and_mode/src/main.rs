use std::collections::HashMap;

fn main() {
    let mut list = vec![1, 2, 45, 21, 3, 4, 7, 0, 10, 2, 3, 2, 2, 7, 1, 21];
    println!("Original list: {:?}", list);

    list.sort();
    println!("Sorted list: {:?}", list);

    let list_length = list.len();
    let middle_point = list_length / 2 - 1;
    println!(
        "The list have a length of {}, the median is at possition {} and is {}.",
        list_length,
        middle_point,
        list.get(middle_point).unwrap()
    );

    let mut item_count = HashMap::new();
    for item in list {
        let count = item_count.entry(item).or_insert(0);
        *count += 1;
    }

    let max_item = {
        let mut current_max_item = 0;
        let mut current_max_count = 0;

        for (item, count) in item_count {
            if count > current_max_count {
                current_max_item = item;
                current_max_count = count;
            }
        }

        (current_max_item, current_max_count)
    };
    println!(
        "The item that appears the most is {} with a total of {} times.",
        max_item.0, max_item.1
    );
}

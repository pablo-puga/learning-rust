#![allow(dead_code)]

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shows_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() returns owned values instead of references and consumes the original vec
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        // we can consume iterator items by manually calling the next() method
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        // sum() consumes the iterator taking its ownership, thus it is not valid after that
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];

        /*
           map() does not consume the original vector
           Because map takes a closure, we can specify any operation we want to perform on each item.
           This is a great example of how closures let you customize some behavior while reusing the
           iteration behavior that the Iterator trait provides.

           You can chain multiple calls to iterator adaptors to perform complex actions in a readable
           way. But because all iterators are lazy, you have to call one of the consuming adaptor
           methods to get results from calls to iterator adaptors.
        */
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

        for index in 0..2 {
            let v1_val = v1.get(index).unwrap();
            let v2_val = v2.get(index).unwrap();
            assert_eq!(v1_val + 1, *v2_val);
        }
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shows_in_size(shoes, 10);

        assert_eq!(in_my_size, vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]);
    }
}

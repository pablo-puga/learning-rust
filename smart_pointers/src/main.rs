#![allow(unused)]

use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct CustomBox<T>(T);

impl<T> CustomBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) -> () {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum SharedList {
    Cons(i32, Rc<SharedList>),
    Nil,
}

#[derive(Debug)]
enum MutableSharedList {
    Cons(Rc<RefCell<i32>>, Rc<MutableSharedList>),
    Nil,
}

#[derive(Debug)]
enum ListLeak {
    Cons(i32, RefCell<Rc<ListLeak>>),
    Nil,
}

impl ListLeak {
    fn tail(&self) -> Option<&RefCell<Rc<ListLeak>>> {
        match self {
            ListLeak::Cons(_, item) => Some(item),
            ListLeak::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let x = 5;
    let y = CustomBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let world = CustomBox::new(String::from("World"));
    hello(&world);
    println!("\n-----\n");

    let a_list = Rc::new(SharedList::Cons(
        5,
        Rc::new(SharedList::Cons(10, Rc::new(SharedList::Nil))),
    ));
    println!(
        "count after creating a_list = {}",
        Rc::strong_count(&a_list)
    );

    let b_list = SharedList::Cons(3, Rc::clone(&a_list));
    println!(
        "count after creating b_list = {}",
        Rc::strong_count(&a_list)
    );

    {
        let c_list = SharedList::Cons(4, Rc::clone(&a_list));
        println!(
            "count after creating c_list = {}",
            Rc::strong_count(&a_list)
        );
    }
    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&a_list)
    );

    println!("\n-----\n");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(MutableSharedList::Cons(
        Rc::clone(&value),
        Rc::new(MutableSharedList::Nil),
    ));
    let b = MutableSharedList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = MutableSharedList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!("\n-----\n");

    let a = Rc::new(ListLeak::Cons(5, RefCell::new(Rc::new(ListLeak::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(ListLeak::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    println!("\n-----\n");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("\n-----\n");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer c created.");
    std::mem::drop(c); // We can't directly call the .drop() method, we have to use this function (it is in the prelude)
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer d created.");
}

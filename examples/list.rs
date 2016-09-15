extern crate functils;
use functils::list::List;
fn main() {
    let mut list1: List<i32> = List::new();
    let mut list2: List<i32> = List::new();
    let list3: List<i32> = List::new();

    list1.cons(1);
    list1.cons(2);
    list1.cons(3);
    list1.cons(4);

    // [ 4 3 2 1 ]
    println!("list1: {}", list1);

    list2.cons(4);
    list2.cons(3);
    list2.cons(2);
    list2.cons(1);

    // [ 1 2 3 4 ]
    println!("list2: {}", list2);

    // []
    println!("list3: {}", list3);

    // This eats up the list
    list1.append(list2);

    // [ 4 3 2 1 1 2 3 4 ]
    println!("list1: {}", list1);

    // true
    println!("Is list3 null: {}", list3.null());

    // Some(1)
    println!("list1.head(): {:?}", list1.head());
    println!("list1: {}", list1);

    println!("list1.tail(): {}", list1.tail());

}

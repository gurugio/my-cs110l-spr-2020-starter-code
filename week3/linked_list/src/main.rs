use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    let mut cloned_list = list.clone();
    println!("cloned: {}", cloned_list);

    if list == cloned_list {
        println!("Just-cloned list is same");
    } else {
        println!("Just-cloned list is not same")
    }

    cloned_list.push_front(4444);
    println!("cloned+4444: {}", cloned_list);
    println!("original: {}", list);

    if list == cloned_list {
        println!("Changed-cloned list is same");
    } else {
        println!("Changed-cloned list is not same")
    }

    // If you implement iterator trait:
    for val in list {
        println!("iter of list: {}", val);
    }

    // list is not available because it has been moved
    //println!("list after iter: {}", list);

    // create a iterator with ref to the list
    // then list is not moved
    let ref_cloned_list = &cloned_list;
    for (i, val) in ref_cloned_list.into_iter().enumerate() {
        println!("into_iter of cloned_list: ({}, {})", i, val);
    }
    println!("list after into_iter: {}", cloned_list);
}

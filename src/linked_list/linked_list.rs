use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<u8> = LinkedList::new();

    /* Insert elements in the list */
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);

    /* Delete an element from the front */
    list.pop_front();

    /* Print the list */
    for item in list.iter_mut() {
        print!("{} -> ", item);
    }
    println!();
}

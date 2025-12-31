mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<i64> = LinkedList::new();

    list.push_front(20);
    list.push_front(10);
    list.display();
    list.push_back(30);
    list.push_back(50);
    list.display();
    list.insert_at_index(3, 40);
    list.display();

    let peak_first_value = list.peek();

    println!("Count = {}", list.len());

    if peak_first_value.is_some() {
        println!("First Peek = {}", peak_first_value.unwrap());
    }

    let peak_last_value = list.peek_back();
    if peak_last_value.is_some() {
        println!("Last Peek = {}", peak_last_value.unwrap());
    }

    let remove_at_index = list.remove_at_index(2);
    if remove_at_index.is_some() {
        println!("Removed at index = {}", remove_at_index.unwrap());
    }

    println!("Contains 20 value = {}", list.contains(20));

    list.display();
}

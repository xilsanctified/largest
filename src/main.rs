fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn smallest<T: PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let number_list = vec![1, 5, 2, 5, 7, 1, 199, 4, 6, 3, 12];

    println!("List of numbers: {:?}", number_list);
    println!("Largest number: {}", largest(&number_list));
    println!("Smallest number: {}", smallest(&number_list));
}

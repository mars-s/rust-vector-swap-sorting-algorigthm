use std::str;

fn get_value(a: &str) -> String {
    let mut line = String::new();
    println!("{}", a);
    std::io::stdin().read_line(&mut line).unwrap();
    return line
}

fn get_vec(a: String) -> Vec<i32> {
    let numbers: Vec<i32> = a
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    return numbers
}

fn sort(mut list: Vec<i32>) {
    let length = list.len();

    for x in 0..length {
        for y in 0..length {
            if list[x] < list[y] {
                list.swap(x,y)
            }
        }
    }

    println!("{:?}", list);
}

fn main () {
    let input = get_value("enter a list of numbers");
    let vectors = get_vec(input);
    sort(vectors);


}



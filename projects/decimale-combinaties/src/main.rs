const MIN_NUMBER: usize = 0;
const MAX_NUMBER: usize = 100;

fn main() {
    let output = (MIN_NUMBER..MAX_NUMBER)
        .map(|number| format!("{:02}", number))
        .collect::<Vec<_>>()
        .join(", ");

    println!("{}", output);
}

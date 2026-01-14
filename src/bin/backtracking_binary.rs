fn build_binary(size: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut current: String = String::new();

    backtrack_binary(&mut result, &mut current, size);

    result
}

fn backtrack_binary(result: &mut Vec<String>, current: &mut String, size: usize) {
    if current.len() == size {
        result.push(current.clone());
        return;
    }

    for choice in ['0', '1'] {
        current.push(choice);
        backtrack_binary(result, current, size);
        current.pop();
    }
}

fn main() {
    println!("{:?}", build_binary(2));
}

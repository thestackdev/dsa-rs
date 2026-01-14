fn subsets(nums: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut current: Vec<i32> = Vec::new();

    backtrack_subsets(&mut result, &mut current, nums, 0);

    result
}

fn backtrack_subsets(
    result: &mut Vec<Vec<i32>>,
    current: &mut Vec<i32>,
    nums: &Vec<i32>,
    index: usize,
) {
    if nums.len() == index {
        result.push(current.clone());
        return;
    }

    backtrack_subsets(result, current, nums, index + 1);

    current.push(nums[index]);
    backtrack_subsets(result, current, nums, index + 1);

    current.pop();
}

fn main() {
    let nums = vec![10, 20];
    println!("{:?}", subsets(&nums));
}


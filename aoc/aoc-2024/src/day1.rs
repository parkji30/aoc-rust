pub fn historian_hysteria() -> Vec<i32> {
    let mut list1: [i32; 6] = [3, 4, 2, 1, 3, 3];
    let mut list2: [i32; 6] = [4, 3, 5, 3, 9, 3];

    list1.sort();
    list2.sort();

    let mut distance_diff: Vec<i32> = Vec::new();
    for (value1, value2) in list1.iter().zip(list2.iter()){
        distance_diff.push(value1 - value2)
    }
    distance_diff
}
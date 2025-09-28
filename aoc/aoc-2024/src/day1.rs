pub fn historian_hysteria() -> i32 {
    let mut list1: [i32; 6] = [3, 4, 2, 1, 3, 3];
    let mut list2: [i32; 6] = [4, 3, 5, 3, 9, 3];
    let mut net_distace: i32 = 0;
    
    list1.sort();
    list2.sort();
 
    for (value1, value2) in list1.iter().zip(list2.iter()){
        net_distace += (value1 - value2).abs()
    }
    net_distace
}
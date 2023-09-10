fn gen_arr() -> Vec<i32> {
    let mut arr = Vec::new();
    for i in 0..10 {
        arr.push(i);
    }
    arr
}

fn contains(arr: &Vec<i32>, n: i32) -> bool {
    for i in arr {
        if *i == n {
            return true;
        }
    }
    false
}

fn unused() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let arr = gen_arr();
        assert_eq!(contains(&arr, 1), true);
    }
}

pub fn sort(x: &mut[u32], up: bool) {
    if x.len() > 1 {
        let middle_index = x.len() / 2;
        sort(&mut x[..middle_index], true);
        sort(&mut x[middle_index..], false);
        sort_sub(x, up);
    }
}

fn sort_sub(x: &mut[u32], up: bool) {
    if x.len() > 1 {
        let middle_index = x.len() / 2;
        swap_if_needed(x, up);
        sort_sub(&mut x[..middle_index], up);
        sort_sub(&mut x[middle_index..], up);
    }
}

fn swap_if_needed(x: &mut[u32], up: bool) {
    let middle_index = x.len() / 2;
    for i in 0..middle_index {
        if (x[i] > x[i + middle_index]) == up {
            x.swap(i, i + middle_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::sort;

    #[test]
    fn sort_u32_ascending() {
        let mut array = vec![1, 3, 65, 21, 2, 9, 7, 0];
        sort(&mut array, true);

        assert_eq!(array, vec![0, 1, 2, 3, 7, 9, 21, 65]);
    }

    #[test]
    fn test_u32_descending() {
        let mut array = vec![1, 3, 65, 21, 2, 9, 7, 0];
        sort(&mut array, false);

        assert_eq!(array, vec![65, 21, 9, 7, 3, 2, 1, 0]);
    }
}

use crate::SortOrder;

pub fn sort<T: Ord>(x: &mut[T], order: SortOrder) -> Result<(), String> {
    if x.len().is_power_of_two() {
        do_sort(x, order);
        Ok(())
    } else {
        Err(format!("array length is not power of 2 ({})", x.len()))
    }
}

fn do_sort<T: Ord>(x: &mut[T], order: SortOrder) {
    if x.len() > 1 {
        let middle_index = x.len() / 2;
        do_sort(&mut x[..middle_index], SortOrder::Ascending);
        do_sort(&mut x[middle_index..], SortOrder::Descending);
        sort_sub(x, order == SortOrder::Ascending);
    }
}

fn sort_sub<T: Ord>(x: &mut[T], up: bool) {
    if x.len() > 1 {
        let middle_index = x.len() / 2;
        swap_if_needed(x, up);
        sort_sub(&mut x[..middle_index], up);
        sort_sub(&mut x[middle_index..], up);
    }
}

fn swap_if_needed<T: Ord>(x: &mut[T], up: bool) {
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
    use crate::SortOrder;

    #[test]
    fn sort_u32_ascending() {
        let mut array: Vec<u32> = vec![1, 3, 65, 21, 2, 9, 7, 0];
        assert_eq!((), sort(&mut array, SortOrder::Ascending).unwrap());
        assert_eq!(array, vec![0, 1, 2, 3, 7, 9, 21, 65]);
    }

    #[test]
    fn test_u32_descending() {
        let mut array: Vec<u32> = vec![1, 3, 65, 21, 2, 9, 7, 0];
        assert_eq!((), sort(&mut array, SortOrder::Descending).unwrap());
        assert_eq!(array, vec![65, 21, 9, 7, 3, 2, 1, 0]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut array: Vec<&str> = vec!["hoge", "fuga", "fugo", "hage"];
        assert_eq!((), sort(&mut array, SortOrder::Ascending).unwrap());
        assert_eq!(array, vec!["fuga", "fugo", "hage", "hoge"]);
    }

    #[test]
    fn test_str_descending() {
        let mut array: Vec<&str> = vec!["hoge", "fuga", "fugo", "hage"];
        assert_eq!((), sort(&mut array, SortOrder::Descending).unwrap());
        assert_eq!(array, vec!["hoge", "hage", "fugo", "fuga"]);
    }
}

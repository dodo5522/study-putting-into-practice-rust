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

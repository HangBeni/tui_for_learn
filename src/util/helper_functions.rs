pub fn switch_list(selected: Option<usize>, len: usize) -> Option<usize> {
    if len == 0 {
        None
    } else if selected.unwrap() < len {
        Some(selected.unwrap())
    } else {
        Some(len - 1)
    }
}

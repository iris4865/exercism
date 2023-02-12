mod tests;

pub fn create_empty() -> Vec<u8> {
    vec![]
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    (0..count).into_iter().map(|_| 0).collect()
}

pub fn fibonacci() -> Vec<u8> {
    vec![1, 1, 2, 3, 5]
}

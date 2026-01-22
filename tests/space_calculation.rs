#[mini_anchor::account]
pub struct TestVecSpace {
    #[max_len(10)]
    pub items: Vec<u32>, // 4 + (10 * 4) = 44 bytes
}

#[mini_anchor::account]
pub struct TestOptionSpace {
    pub maybe_num: Option<u64>, // 1 + 8 = 9 bytes
    #[max_len(20)]
    pub maybe_name: Option<String>, // 1 + 4 + (max_len * char_size)
}

#[test]
fn test_vec_space_calculation() {
    assert_eq!(TestVecSpace::SPACE, 8 + (4 + (10 * 4)));
}

#[test]
fn test_option_space_calculation() {
    assert_eq!(TestOptionSpace::SPACE, 8 + (1 + 8) + (1 + (4 + 20)));
}

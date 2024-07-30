
pub fn align_up(value: usize, align: usize) -> usize {
    assert!(align > 0);
    assert!(value <= usize::MAX - align);

    if value % align == 0 { value }
    else { value + (align - (value % align)) }
}

pub fn align_down(value: usize, align: usize) -> usize {
    assert!(align > 0);

    if value % align == 0 { value }
    else { value - (value % align) }
}

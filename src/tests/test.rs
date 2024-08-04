use crate::*;

#[test]
fn test_quick_sort() {
    let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    quick_sort(&mut vec);
    assert_eq!(vec, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
}

#[test]
fn test_bubble_sort() {
    let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    bubble_sort(&mut vec);
    assert_eq!(vec, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
}

#[test]
fn test_merge_sort() {
    let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    merge_sort(&mut vec);
    assert_eq!(vec, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
}

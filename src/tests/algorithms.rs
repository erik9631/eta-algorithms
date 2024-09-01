use crate::algorithms::{best_flat_diversity, flat_diversity_score};

#[test]
fn test_flat_diversity_score_diverse() {
    let data = [
        (1, ()),
        (2, ()),
        (3, ()),
        (4, ()),
        (5, ()),
        (6, ()),
        (7, ()),
        (8, ()),
        (9, ()),
        (10, ()),
    ];
    let mut hash_map = std::collections::HashMap::with_capacity(data.len());
    let (score, bitmap) = flat_diversity_score(&data, &mut hash_map);
    assert_eq!(score, 10);
    assert_eq!(bitmap.capacity(), 1);
    assert_eq!(bitmap.bit_capacity(), 10);
    for i in 0..data.len() {
        assert_eq!(bitmap.get(i), Some(true));
    }
}

#[test]
fn test_flat_diversity_score_equal() {
    let data = [
        (1, ()),
        (1, ()),
        (1, ()),
        (1, ()),
        (1, ()),
        (1, ()),
        (1, ()),
        (1, ()),
        (1, ()),
        (1, ()),
    ];
    let mut hash_map = std::collections::HashMap::with_capacity(data.len());
    let (score, bitmap) = flat_diversity_score(&data, &mut hash_map);
    assert_eq!(score, 1);
    assert_eq!(bitmap.capacity(), 1);
    assert_eq!(bitmap.bit_capacity(), data.len());

    for i in 0..data.len() {
        assert_eq!(bitmap.get(i), Some(false));
    }
}

#[test]
fn test_flat_diversity_score_periodic() {
    let data = [
        (1, ()),
        (2, ()),
        (1, ()),
        (2, ()),
        (1, ()),
        (2, ()),
        (1, ()),
        (2, ()),
        (1, ()),
        (2, ()),
    ];
    let mut hash_map = std::collections::HashMap::with_capacity(data.len());
    let (score, bitmap) = flat_diversity_score(&data, &mut hash_map);
    assert_eq!(score, 2);
    assert_eq!(bitmap.capacity(), 1);
    assert_eq!(bitmap.bit_capacity(), data.len());

    for i in 0..data.len() {
        assert_eq!(bitmap.get(i), Some(false));
    }
}

#[test]
fn test_flat_diversity_score_diverse1() {
    let data = [(1, ()), (3, ()), (3, ()), (4, ())];
    let mut hash_map = std::collections::HashMap::with_capacity(data.len());
    let (score, bitmap) = flat_diversity_score(&data, &mut hash_map);
    assert_eq!(score, 3);
    assert_eq!(bitmap.capacity(), 1);
    assert_eq!(bitmap.bit_capacity(), data.len());
    assert_eq!(bitmap.get(0), Some(true));
    assert_eq!(bitmap.get(1), Some(false));
    assert_eq!(bitmap.get(2), Some(false));
    assert_eq!(bitmap.get(3), Some(true));
}

#[test]
fn test_best_flat_diversity_same() {
    let data = [(1, ()), (1, ()), (1, ()), (1, ())];
    let input_data = [(2, ()), (2, ()), (2, ()), (2, ())];
    let new_data = best_flat_diversity(&data, &input_data);
    assert_eq!(new_data[0].0, 2);
    assert_eq!(new_data[1].0, 1);
    assert_eq!(new_data[2].0, 1);
    assert_eq!(new_data[3].0, 1);
}

#[test]
fn test_best_flat_diversity_diverge_same() {
    let data = [(1, ()), (2, ()), (1, ()), (2, ())];
    let input_data = [(1, ()), (2, ()), (1, ()), (2, ())];
    let new_data = best_flat_diversity(&data, &input_data);
    assert_eq!(new_data[0].0, 1);
    assert_eq!(new_data[1].0, 2);
    assert_eq!(new_data[2].0, 1);
    assert_eq!(new_data[3].0, 2);
}
#[test]
fn test_best_flat_diversity_diverge_change() {
    let data = [(1, ()), (2, ()), (1, ()), (2, ())];
    let input_data = [(1, ()), (2, ()), (1, ()), (3, ())];
    let new_data = best_flat_diversity(&data, &input_data);
    assert_eq!(new_data[0].0, 3);
    assert_eq!(new_data[1].0, 2);
    assert_eq!(new_data[2].0, 1);
    assert_eq!(new_data[3].0, 2);
}

#[test]
fn test_best_flat_diversity_diverge_different_size() {
    let data = [(1, 1), (1, 2), (1, 4), (1, 8)];
    let input_data = [(1, 2), (1, 4), (1, 8), (2, 2), (2, 4), (2, 16), (3, 8)];
    let new_data = best_flat_diversity(&data, &input_data);

    assert_eq!(new_data[0], (3, 8));
    assert_eq!(new_data[1], (2, 16));
    assert_eq!(new_data[2], (1, 4));
    assert_eq!(new_data[3], (1, 8));
}

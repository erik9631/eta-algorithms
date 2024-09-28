use crate::algorithms::{extract_unique_pairs, find_uniques, optimize_diversity};
use std::collections::HashMap;

#[test]
fn test_calculate_uniqueness_diverse() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut hash_map = std::collections::HashMap::with_capacity(data.len());
    let (score, bitmap) = find_uniques(&data, &mut hash_map);
    assert_eq!(score, 10);
    assert_eq!(bitmap.capacity(), 1);
    assert_eq!(bitmap.bit_capacity(), 10);
    for i in 0..data.len() {
        assert_eq!(bitmap.get(i), Some(true));
    }
}

#[test]
fn test_calculate_uniqueness_equal() {
    let data = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let mut hash_map = std::collections::HashMap::with_capacity(data.len());
    let (score, bitmap) = find_uniques(&data, &mut hash_map);
    assert_eq!(score, 1);
    assert_eq!(bitmap.capacity(), 1);
    assert_eq!(bitmap.bit_capacity(), data.len());

    for i in 0..data.len() {
        assert_eq!(bitmap.get(i), Some(false));
    }
}

#[test]
fn test_calculate_uniqueness_periodic() {
    let data = [1, 2, 1, 2, 1, 2, 1, 2, 1, 2];
    let mut hash_map = std::collections::HashMap::with_capacity(data.len());
    let (score, bitmap) = find_uniques(&data, &mut hash_map);
    assert_eq!(score, 2);
    assert_eq!(bitmap.capacity(), 1);
    assert_eq!(bitmap.bit_capacity(), data.len());

    for i in 0..data.len() {
        assert_eq!(bitmap.get(i), Some(false));
    }
}

#[test]
fn test_calculate_uniqueness_diverse1() {
    let data = [1, 3, 3, 4];
    let mut hash_map = std::collections::HashMap::with_capacity(data.len());
    let (score, bitmap) = find_uniques(&data, &mut hash_map);
    assert_eq!(score, 3);
    assert_eq!(bitmap.capacity(), 1);
    assert_eq!(bitmap.bit_capacity(), data.len());
    assert_eq!(bitmap.get(0), Some(true));
    assert_eq!(bitmap.get(1), Some(false));
    assert_eq!(bitmap.get(2), Some(false));
    assert_eq!(bitmap.get(3), Some(true));
}

#[test]
fn test_calculate_uniqueness_same() {
    let data = [1, 1, 1, 1];
    let input_data = [2, 2, 2, 2];
    let new_data = optimize_diversity(&data, &input_data);
    assert_eq!(new_data[0], 2);
    assert_eq!(new_data[1], 1);
    assert_eq!(new_data[2], 1);
    assert_eq!(new_data[3], 1);
}

#[test]
fn test_best_flat_diversity_diverge_same() {
    let data = [1, 2, 1, 2];
    let input_data = [1, 2, 1, 2];
    let new_data = optimize_diversity(&data, &input_data);
    assert_eq!(new_data[0], 1);
    assert_eq!(new_data[1], 2);
    assert_eq!(new_data[2], 1);
    assert_eq!(new_data[3], 2);
}
#[test]
fn test_calculate_uniqueness_diverge_change() {
    let data = [1, 2, 1, 2];
    let input_data = [1, 2, 1, 3];
    let new_data = optimize_diversity(&data, &input_data);
    assert_eq!(new_data[0], 3);
    assert_eq!(new_data[1], 2);
    assert_eq!(new_data[2], 1);
    assert_eq!(new_data[3], 2);
}

#[test]
fn test_extract_unique_pairs_diverse() {
    let values = [1, 2, 4, 5, 6, 6, 8, 4];
    let identifiers = [1, 1, 1, 2, 2, 3, 3, 3];
    let (output_values, output_identifiers) = extract_unique_pairs(&values, &identifiers);
    let mut correct_values = HashMap::from([(1, 1), (5, 2), (6, 3), (4, 3), (8, 3), (2, 1)]);

    for (value, identifier) in output_values.iter().zip(output_identifiers.iter()) {
        let pair = correct_values.remove(value).unwrap();
        assert_eq!(*identifier, pair);
    }
    assert!(correct_values.is_empty());
}

#[test]
fn test_extract_unique_pairs_all_unique() {
    let values = vec![1, 2, 3, 4, 5];
    let identifiers = vec![10, 20, 30, 40, 50];
    let (output_values, output_indices) = extract_unique_pairs(&values, &identifiers);
    let mut correct_values = HashMap::from([(1, 10), (2, 20), (3, 30), (4, 40), (5, 50)]);

    for (value, identifier) in output_values.iter().zip(output_indices.iter()) {
        let pair = correct_values.remove(value).unwrap();
        assert_eq!(*identifier, pair);
    }
    assert!(correct_values.is_empty());
}

#[test]
fn test_extract_unique_pairs_with_duplicates() {
    let values = vec![1, 2, 2, 3, 1, 4];
    let identifiers = vec![10, 20, 30, 40, 50, 60];
    let (output_values, output_indices) = extract_unique_pairs(&values, &identifiers);
    let mut correct_values = HashMap::from([(1, 10), (2, 20), (3, 40), (4, 60)]);

    for (value, identifier) in output_values.iter().zip(output_indices.iter()) {
        let pair = correct_values.remove(value).unwrap();
        assert_eq!(*identifier, pair);
    }
    assert!(correct_values.is_empty());
}
#[test]
fn test_extract_unique_pairs_all_duplicates() {
    let values = vec![1, 1, 1, 1];
    let identifiers = vec![10, 20, 30, 40];
    let (output_values, output_indices) = extract_unique_pairs(&values, &identifiers);
    let mut correct_values = HashMap::from([(1, 10)]);
    for (value, identifier) in output_values.iter().zip(output_indices.iter()) {
        let pair = correct_values.remove(value).unwrap();
        assert_eq!(*identifier, pair);
    }
    assert!(correct_values.is_empty());
}

#[test]
fn test_extract_unique_pairs_empty_input() {
    let values: Vec<i32> = vec![];
    let identifiers: Vec<i32> = vec![];
    let (output_values, output_indices) = extract_unique_pairs(&values, &identifiers);
    assert!(output_values.is_empty());
    assert!(output_indices.is_empty());
}

#[test]
fn test_extract_unique_pairs_with_strings() {
    let values = vec!["apple", "banana", "apple", "cherry", "date", "banana"];
    let identifiers = vec![100, 200, 300, 400, 500, 600];
    let (output_values, output_indices) = extract_unique_pairs(&values, &identifiers);
    let mut correct_values = HashMap::from([("apple", 100), ("banana", 200), ("cherry", 400), ("date", 500)]);
    for (value, identifier) in output_values.iter().zip(output_indices.iter()) {
        let pair = correct_values.remove(value).unwrap();
        assert_eq!(*identifier, pair);
    }
    assert!(correct_values.is_empty());
}
#[test]
fn test_extract_unique_pairs_with_custom_struct() {
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    let values = vec![
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 1, y: 2 },
        Point { x: 5, y: 6 },
    ];

    let identifiers = vec![1000, 2000, 3000, 4000];
    let (output_values, output_indices) = extract_unique_pairs(&values, &identifiers);
    let mut correct_values = HashMap::from([
        (Point { x: 1, y: 2 }, 1000),
        (Point { x: 3, y: 4 }, 2000),
        (Point { x: 5, y: 6 }, 4000),
    ]);
    for (value, identifier) in output_values.iter().zip(output_indices.iter()) {
        let pair = correct_values.remove(value).unwrap();
        assert_eq!(*identifier, pair);
    }
    assert!(correct_values.is_empty());
}

#[test]
fn test_non_unique_pairs() {
    let values = vec![1, 2, 3, 1, 2, 3, 2, 3, 4];
    let identifiers = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
    let (output_values, output_indices) = extract_unique_pairs(&values, &identifiers);
    let mut correct_values = HashMap::from([(1, 1), (2, 2), (3, 3), (4, 3)]);
    for (value, identifier) in output_values.iter().zip(output_indices.iter()) {
        let pair = correct_values.remove(value).unwrap();
        assert_eq!(*identifier, pair);
    }
    assert!(correct_values.is_empty());
}

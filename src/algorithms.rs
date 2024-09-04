use crate::data_structs::array::Array;
use crate::data_structs::bitmap::Bitmap;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn calculate_uniqueness(identifiers: &[i32], indices_hash: &mut HashMap<i32, usize>) -> (i32, Bitmap) {
    let mut bitmap = Bitmap::new(identifiers.len());
    let mut score = 0;
    for (index, value) in identifiers.iter().enumerate() {
        if let Some(original_index) = indices_hash.get(value) {
            bitmap.set(*original_index, false);
            continue;
        }
        indices_hash.insert(*value, index);
        bitmap.set(index, true);
        score += 1;
    }

    (score, bitmap)
}

pub fn optimize_diversity(existing_identifiers: &[i32], input_identifiers: &[i32]) -> Array<i32> {
    let mut top = input_identifiers.len() - 1;
    let mut hash_map = HashMap::with_capacity(existing_identifiers.len());
    let (_, bitmap) = calculate_uniqueness(existing_identifiers, &mut hash_map);
    let mut new_data = Array::from_slice(existing_identifiers);
    let accessible_indices = bitmap.to_indices_false();
    for accessible_index in accessible_indices {
        if top == 0 {
            break;
        }

        let new_diversity = unsafe { input_identifiers.get_unchecked(top) };
        top -= 1;
        if hash_map.contains_key(new_diversity) {
            continue;
        }
        new_data[accessible_index] = *new_diversity;

        hash_map.insert(*new_diversity, accessible_index);
    }
    new_data
}

pub fn extract_unique_pairs<T>(values: &[T], identifiers: &[i32]) -> (Vec<T>, Vec<i32>)
where
    T: Copy + Sized + Hash + Eq,
{
    let mut value_hashmap = HashMap::with_capacity(values.len());
    let (_, unique_value_markers) = calculate_uniqueness(identifiers, &mut value_hashmap);
    let value_uniques = unique_value_markers.to_indices_true();
    let value_overlap = unique_value_markers.to_indices_false();
    let mut output_values = Vec::with_capacity(value_uniques.len());
    let mut output_indices = Vec::with_capacity(value_uniques.len());

    for index in value_uniques.iter() {
        output_values.push(unsafe { *values.get_unchecked(*index) });
        output_indices.push(*index as i32);
    }

    // Now fill in the non-unique values, but only once!
    let mut fill_out_hash = HashSet::with_capacity(value_overlap.len());
    for index in value_overlap.iter() {
        let value = unsafe { values.get_unchecked(*index) };
        if fill_out_hash.contains(value) {
            continue;
        }
        output_values.push(*value);
        output_indices.push(*index as i32);
        fill_out_hash.insert(value);
    }

    (output_values, output_indices)
}

use crate::data_structs::array::Array;
use crate::data_structs::bitmap::Bitmap;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn find_uniques<T>(values: &[T], values_hash: &mut HashMap<T, usize>) -> (u32, Bitmap)
where
    T: Copy + Sized + Hash + Eq,
{
    let mut bitmap = Bitmap::new(values.len());
    let mut score = 0;
    for (index, value) in values.iter().enumerate() {
        if let Some(original_index) = values_hash.get(value) {
            bitmap.set(*original_index, false);
            continue;
        }
        values_hash.insert(*value, index);
        bitmap.set(index, true);
        score += 1;
    }

    (score, bitmap)
}

pub fn optimize_diversity<T>(existing_values: &[T], values: &[T]) -> Array<T>
where
    T: Copy + Sized + Hash + Eq,
{
    let mut top = values.len() - 1;
    let mut hash_map = HashMap::with_capacity(existing_values.len());
    let (_, bitmap) = find_uniques(existing_values, &mut hash_map);
    let mut new_data = Array::from_slice(existing_values);
    let accessible_indices = bitmap.to_indices_false();
    for accessible_index in accessible_indices {
        if top == 0 {
            break;
        }

        let new_diversity = unsafe { values.get_unchecked(top) };
        top -= 1;
        if hash_map.contains_key(new_diversity) {
            continue;
        }
        new_data[accessible_index] = *new_diversity;

        hash_map.insert(*new_diversity, accessible_index);
    }
    new_data
}

pub fn extract_unique_pairs<T, U>(primary: &[T], secondary: &[U]) -> (Vec<T>, Vec<U>)
where
    T: Copy + Sized + Hash + Eq,
    U: Copy + Sized + Hash + Eq,
{
    let mut primary_set = HashSet::<T>::with_capacity(primary.len());
    let mut secondary_set = HashSet::<U>::with_capacity(secondary.len());
    let mut pre_emptive_map = HashMap::<T, U>::with_capacity(primary.len());
    let mut out_primary = Vec::<T>::with_capacity(primary.len());
    let mut out_secondary = Vec::<U>::with_capacity(secondary.len());

    for (primary, secondary) in primary.iter().zip(secondary) {
        if !primary_set.contains(primary) {
            if secondary_set.insert(*secondary) {
                primary_set.insert(*primary);
                out_primary.push(*primary);
                out_secondary.push(*secondary);
                pre_emptive_map.remove(primary);
                continue;
            }
            pre_emptive_map.insert(*primary, *secondary);
        }
    }

    for (primary, secondary) in pre_emptive_map {
        out_primary.push(primary);
        out_secondary.push(secondary);
    }
    (out_primary, out_secondary)
}

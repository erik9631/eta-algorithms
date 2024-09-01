use crate::data_structs::array::Array;
use crate::data_structs::bitmap::Bitmap;
use std::collections::HashMap;

pub fn flat_diversity_score<T>(data: &[(i32, T)], hash_set: &mut HashMap<i32, usize>) -> (i32, Bitmap) {
    let mut bitmap = Bitmap::new(data.len());
    let mut score = 0;
    for (index, value) in data.iter().enumerate() {
        if let Some(original_index) = hash_set.get(&value.0) {
            bitmap.set(*original_index, false);
            continue;
        }
        hash_set.insert(value.0, index);
        bitmap.set(index, true);
        score += 1;
    }

    (score, bitmap)
}

pub fn best_flat_diversity<T>(data: &[(i32, T)], input_data: &[(i32, T)]) -> Array<(i32, T)>
where
    T: Copy + Sized,
{
    let mut top = input_data.len() - 1;
    let mut hash_map = HashMap::with_capacity(data.len());
    let (_, bitmap) = flat_diversity_score(data, &mut hash_map);
    let mut new_data = Array::from_slice(data);
    let accessible_indices = bitmap.to_indices_false();
    for accessible_index in accessible_indices {
        if top == 0 {
            break;
        }

        let new_diversity = unsafe { input_data.get_unchecked(top) };
        top -= 1;
        if hash_map.contains_key(&new_diversity.0) {
            continue;
        }
        new_data[accessible_index].0 = new_diversity.0;
        new_data[accessible_index].1 = new_diversity.1;

        hash_map.insert(new_diversity.0, accessible_index);
    }
    new_data
}

use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

mod tests;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_map: HashMap<_, u64> = to_hash_map(magazine, |(count, _)| *count += 1);

    to_hash_map(note, |(count, _)| *count += 1)
        .into_iter()
        .all(|(k, n)| magazine_map.get(k).map(|m| m >= &n).unwrap_or_default())
}

// magazine.into_iter().fold(HashMap::new(), |mut map, c| {
//   *map.entry(c).or_default() += 1;
//   map
// });
fn to_hash_map<T, V>(arr: &[T], f: impl Fn((&mut V, T)) -> ()) -> HashMap<T, V>
where
    T: Eq + Hash + Copy,
    V: Default,
{
    arr.into_iter().fold(HashMap::new(), |mut map, v| {
        f((map.entry(*v).or_default(), *v));
        map
    })
}

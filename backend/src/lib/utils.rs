pub fn vec_diff<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T>
where
    T: Eq + PartialEq,
{
    vec1.into_iter().filter(|e| !vec2.contains(e)).collect()
}

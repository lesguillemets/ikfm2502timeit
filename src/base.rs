
pub fn group_by<T, U, F>(it: &[T], f: F) -> Vec<Vec<T>>
where
    T: Clone,
    F: Fn(&T) -> U,
    U: Copy + PartialEq + Eq,
{
    if it.is_empty() {
        return vec![];
    }
    let mut last: U = f(&it[0]);
    let mut result: Vec<Vec<T>> = vec![];
    let mut current_group: Vec<T> = vec![];
    for item in it {
        let fi = f(item);
        if fi == last {
            // continue current stream
            current_group.push(item.clone());
        } else {
            result.push(current_group);
            current_group = vec![item.clone()];
            last = fi;
        }
    }
    result
}

pub fn map<F, T, Y>(input: Vec<T>, mut f: F) -> Vec<Y>
where
    F: FnMut(T) -> Y,
{
    let mut result = Vec::with_capacity(input.len());

    for i in input {
        result.push(f(i));
    }

    result
}

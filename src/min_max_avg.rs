pub fn min_f64(list: &[f64]) -> f64 {
    return *list.iter().min_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap()).unwrap();
}

pub fn max_f64(list: &[f64]) -> f64 {
    return *list.iter().max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap()).unwrap();
}

pub fn avg_f64(list: &[f64]) -> f64 {
    return list.iter().sum::<f64>() / list.len() as f64;
}

pub fn min_usize(list: &[usize]) -> usize {
    return *list.iter().min_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap()).unwrap();
}

pub fn max_usize(list: &[usize]) -> usize {
    return *list.iter().max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap()).unwrap();
}

pub fn avg_usize(list: &[usize]) -> usize {
    return list.iter().sum::<usize>() / list.len();
}
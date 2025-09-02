// TODO: Fix the compiler error in this function.
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     // fill_vec is the sole owner of vec because it wasn't passed in as a reference
//     // thus we can modify it to be mutable
//     let mut vec = vec;
//
//     vec.push(88);
//
//     vec
// }

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    // now it's passed in as a mutable copy
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}

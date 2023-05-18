fn bubble_sort<T: Ord + Copy>(arr: &mut [T]) -> &[T] {
    let len = arr.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..len - 1 {
            if arr[i] > arr[i + 1] {
                let prev = arr[i + 1];
                arr[i + 1] = arr[i];
                arr[i] = prev;
                swapped = true;
            }
        }
    }
    arr
}



#[cfg(test)]
mod tests {
    use rand::{seq::SliceRandom, thread_rng};
    fn generate_vecs(size: u32) -> (Vec<u32>, Vec<u32>) {
        let mut rng = thread_rng();
        let sorted: Vec<u32> = (0..size).collect();
        let mut randoms: Vec<u32> = (0..size).collect();
        randoms.shuffle(&mut rng);
        (sorted, randoms)
    }

    use crate::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let (sorted, mut randoms) = generate_vecs(10);
        assert_eq!(bubble_sort(&mut randoms), sorted);
    }
}

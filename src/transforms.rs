type Line<T> = Vec<T>;
type Square<T> = Vec<Line<T>>;
type Cube<T> = Vec<Square<T>>;

pub fn reverse_rows<T>(vector: &mut Vec<Vec<T>>) {
    for row in vector.iter_mut() {
        row.reverse();
    }
}

pub fn reverse_columns<T: Copy>(vector: &mut Vec<Vec<T>>) {
    transpose(vector);
    reverse_rows(vector);
    transpose(vector);
}

pub fn transpose<T: Copy>(vector: &mut Vec<Vec<T>>) {
    let transpose: Vec<Vec<T>> = vector
        .iter()
        .enumerate()
        .map(|(i, _)| vector
             .iter()
             .flatten()
             .skip(i)
             .step_by(vector.len())
             .copied()
             .collect())
        .collect();

    vector
        .iter_mut()
        .flatten()
        .zip(transpose.iter().flatten())
        .for_each(|(v, t)| *v = *t);
}

pub fn rotate90<T: Copy>(vector: &mut Vec<Vec<T>>) {
    transpose(vector);
    reverse_rows(vector);
}

pub fn rotate_minus90<T: Copy>(vector: &mut Vec<Vec<T>>) {
    transpose(vector);
    reverse_columns(vector);
}

pub fn rotate_180<T: Copy>(vector: &mut Vec<Vec<T>>) {
    reverse_columns(vector);
    reverse_rows(vector);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn vec_compare<T: std::cmp::Eq>(A: &Vec<T>, B: &Vec<T>) -> bool {
        assert_eq!(A.len(), B.len(), "vectors not the same length");

        for (a, b) in A.iter().zip(B) {
            if *a != *b {
                return false;
            }
        }

        return true;
    }

    fn vec2d_compare<T: std::cmp::Eq>(A: &Vec<Vec<T>>, B: &Vec<Vec<T>>) -> bool {
        assert_eq!(A.len(), B.len(), "vectors not the same length");

        for (a, b) in A.iter().zip(B) {
            if vec_compare(a, b) == false {
                return false;
            }
        }

        return true;
    }

    #[test]
    fn reverse_rows_should_reverse_rows() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let expected = vec![
            vec![3, 2, 1],
            vec![6, 5, 4],
            vec![9, 8, 7],
        ];

        reverse_rows(&mut actual);

        assert_eq!(true, vec_compare(&actual, &expected));
    }

    #[test]
    fn reverse_columns_should_reverse_columns() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let expected: Vec<Vec<u32>> = vec![
            vec![7, 8, 9],
            vec![4, 5, 6],
            vec![1, 2, 3],
        ];


        reverse_columns(&mut actual);

        assert_eq!(true, vec_compare(&actual, &expected));
    }

    #[test]
    fn transponse_should_transpose() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];


        let expected: Vec<Vec<u32>> = vec![
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
        ];

        transpose(&mut actual);

        assert_eq!(true, vec_compare(&actual, &expected));

    }

    #[test]
    fn rotate90_should_rotate_by_90() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let expected: Vec<Vec<u32>> = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ];

        rotate90(&mut actual);

        assert_eq!(true, vec_compare(&actual, &expected));
    }

    #[test]
    fn rotate_minus90_should_rotate_by_minus90() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let expected: Vec<Vec<u32>> = vec![
            vec![3, 6, 9],
            vec![2, 5, 8],
            vec![1, 4, 7],
        ];

        rotate_minus90(&mut actual);

        assert_eq!(true, vec_compare(&actual, &expected));
    }

    #[test]
    fn rotate_180_should_rotate_by_180() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let expected: Vec<Vec<u32>> = vec![
            vec![9, 8, 7],
            vec![6, 5, 4],
            vec![3, 2, 1],
        ];

        rotate_180(&mut actual);

        assert_eq!(true, vec_compare(&actual, &expected));
    }
}

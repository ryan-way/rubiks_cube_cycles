type Line<T> = Vec<T>;
type Square<T> = Vec<Line<T>>;
type Cube<T> = Vec<Square<T>>;

pub fn reverse_rows<T>(vector: &mut Vec<Vec<T>>) {
    for row in vector.iter_mut() {
        row.reverse();
    }
}

pub fn reverse_columns<T: Copy>(vector: &mut Vec<Vec<T>>) {
    vector.reverse();
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

pub fn get_x_y_plane<T: Copy>(cube: &mut Vec<Vec<Vec<T>>>, idx: usize) -> Vec<Vec<T>> {
    if cube.len() < idx {
        panic!("Index {} is out of bounds for cube length: {}", idx, cube.len());
    }
    cube
        .iter()
        .skip(idx)
        .take(1)
        .map(|x| x.clone())
        .flatten()
        .collect()

}

pub fn set_x_y_plane<T: Copy>(cube: &mut Cube<T>, square: &Square<T>, idx: usize) {
    cube
        .iter_mut()
        .skip(idx)
        .take(1)
        .flatten()
        .flatten()
        .zip(square.iter().flatten())
        .for_each(|(c, s)| *c = *s);
}

pub fn get_y_z_plane<T: Copy>(cube: &mut Vec<Vec<Vec<T>>>, idx: usize) -> Vec<Vec<T>> {
    cube
        .iter()
        .map(|square| square
             .iter()
             .flatten()
             .skip(idx)
             .step_by(square.len())
             .copied()
             .collect())
        .collect()
}

pub fn set_y_z_plane<T: Copy>(cube: &mut Cube<T>, square: &Square<T>, idx: usize) {
    cube
        .iter_mut()
        .map::<Line<&mut T>, _>(|square| {
            let size = square.len();
            square
             .iter_mut()
             .flatten()
             .skip(idx)
             .step_by(size)
             .collect()
        })
        .flatten()
        .zip(square.iter().flatten())
        .for_each(|(c, s)| *c = *s);
}

pub fn get_x_z_plane<T: Copy>(cube: &mut Vec<Vec<Vec<T>>>, idx: usize) -> Vec<Vec<T>> {
    cube
        .iter()
        .map(|square| square
             .iter()
             .flatten()
             .skip(idx*cube.len())
             .take(cube.len())
             .copied()
             .collect())
        .collect()
}

pub fn set_x_z_plane<T: Copy>(cube: &mut Cube<T>, square: &Square<T>, idx: usize) {
    let size = cube.len();
    cube
        .iter_mut()
        .map::<Line<&mut T>, _>(|square| square
             .iter_mut()
             .flatten()
             .skip(idx*size)
             .take(size)
             .collect())
        .flatten()
        .zip(square.iter().flatten())
        .for_each(|(c, s)| *c = *s);
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

    fn get_data_cube() -> Vec<Vec<Vec<u32>>> {
        vec![
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
            ],
            vec! [
                vec![10, 11, 12],
                vec![13, 14, 15],
                vec![16, 17, 18],
            ],
            vec![
                vec![19, 20, 21],
                vec![22, 23, 24],
                vec![25, 26, 27],
            ]
        ]
    }

    #[test]
    fn get_x_y_plane_should_get_x_y_plane() {
        let mut cube = get_data_cube();

        let first_plane = get_x_y_plane(&mut cube, 0);

        assert_eq!(true, vec_compare(&first_plane, &vec![
                                     vec![1, 2, 3],
                                     vec![4, 5, 6],
                                     vec![7, 8, 9],
        ]));

        let second_plane = get_x_y_plane(&mut cube, 1);

        assert_eq!(true, vec_compare(&second_plane, &vec![
                                     vec![10, 11, 12],
                                     vec![13, 14, 15],
                                     vec![16, 17, 18],
        ]));

        let third_plane = get_x_y_plane(&mut cube, 2);

        assert_eq!(true, vec_compare(&third_plane, &vec![
                                     vec![19, 20, 21],
                                     vec![22, 23, 24],
                                     vec![25, 26, 27],
        ]));
    }

    #[test]
    fn get_y_z_plane_should_get_y_z_plane() {
        let mut cube = get_data_cube();

        let first_plane = get_y_z_plane(&mut cube, 0);

        assert_eq!(true, vec_compare(&first_plane, &vec![
                                     vec![1, 4, 7],
                                     vec![10, 13, 16],
                                     vec![19, 22, 25],
        ]));

        let second_plane = get_y_z_plane(&mut cube, 1);

        assert_eq!(true, vec_compare(&second_plane, &vec![
                                     vec![2, 5, 8],
                                     vec![11, 14, 17],
                                     vec![20, 23, 26],
        ]));

        let third_plane = get_y_z_plane(&mut cube, 2);

        assert_eq!(true, vec_compare(&third_plane, &vec![
                                     vec![3, 6, 9],
                                     vec![12, 15, 18],
                                     vec![21, 24, 27],
        ]));
    }

    #[test]
    fn get_x_z_plane_should_get_x_z_plane() {
        let mut cube = get_data_cube();

        let first_plane = get_x_z_plane(&mut cube, 0);

        assert_eq!(true, vec_compare(&first_plane, &vec![
                                     vec![1, 2, 3],
                                     vec![10, 11, 12],
                                     vec![19, 20, 21],
        ]));

        let second_plane = get_x_z_plane(&mut cube, 1);

        assert_eq!(true, vec_compare(&second_plane, &vec![
                                     vec![4, 5, 6],
                                     vec![13, 14, 15],
                                     vec![22, 23, 24],
        ]));

        let third_plane = get_x_z_plane(&mut cube, 2);

        assert_eq!(true, vec_compare(&third_plane, &vec![
                                     vec![7, 8, 9],
                                     vec![16, 17, 18],
                                     vec![25, 26, 27],
        ]));
    }

    #[test]
    fn set_x_y_plane_should_set_x_y_plane() {
        let mut cube = get_data_cube();

        let first = get_x_y_plane(&mut cube, 0);

        set_x_y_plane(&mut cube, &first, 1);
        set_x_y_plane(&mut cube, &first, 2);

        first.iter().flatten().for_each(|x| println!("{}", x));

        assert_eq!(true, vec_compare(&first, &get_x_y_plane(&mut cube, 1)));
        assert_eq!(true, vec_compare(&first, &get_x_y_plane(&mut cube, 2)));
    }

    #[test]
    fn set_y_z_plane_should_set_y_z_plane() {
        let mut cube = get_data_cube();

        let first = get_y_z_plane(&mut cube, 0);

        set_y_z_plane(&mut cube, &first, 1);
        set_y_z_plane(&mut cube, &first, 2);

        assert_eq!(true, vec_compare(&first, &get_y_z_plane(&mut cube, 1)));
        assert_eq!(true, vec_compare(&first, &get_y_z_plane(&mut cube, 2)));
    }

    #[test]
    fn set_x_z_plane_should_set_x_z_plane() {
        let mut cube = get_data_cube();

        let first = get_x_z_plane(&mut cube, 0);

        set_x_z_plane(&mut cube, &first, 1);
        set_x_z_plane(&mut cube, &first, 2);

        assert_eq!(true, vec_compare(&first, &get_x_z_plane(&mut cube, 1)));
        assert_eq!(true, vec_compare(&first, &get_x_z_plane(&mut cube, 2)));
    }

    use test::Bencher;

    #[bench]
    fn bench_transpose(b: &mut Bencher) {
        let mut square = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        b.iter(|| { transpose(&mut square) });
    }
    
    #[bench]
    fn bench_reverse_rows(b: &mut Bencher) {
        let mut square = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        b.iter(|| { reverse_rows(&mut square) });
    }

    #[bench]
    fn bench_reverse_columns(b: &mut Bencher) {
        let mut square = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        b.iter(|| { reverse_columns(&mut square) });
    }
}

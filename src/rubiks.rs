use crate::transforms::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RubiksCell {
    RYB, RY, RYG, RB, R, RG, RBW, RW, RGW,
    YB, Y, YG, B, C, G, BW, W, GW,
    YBO, YO, YGO, BO, O, GO, BWO, WO, GWO,
}

type Cube = [[[RubiksCell; 3]; 3]; 3];

impl RubiksCell {

    pub const fn collect() -> Cube {
        use RubiksCell::*;

        [
            [
                [RYB, RY, RYG],
                [RB, R, RG],
                [RBW, RW, RGW]
            ],
            [
                [YB, Y, YG],
                [B, C, G],
                [BW, W, GW]
            ],
            [
                [YBO, YO, YGO],
                [BO, O, GO],
                [BWO, WO, GWO]
            ],
        ]
    }
}

pub struct RubiksCube3x3 {
    cube: Vec<Vec<Vec<RubiksCell>>>,
}

impl RubiksCube3x3 {
    const SOLVED_CUBE: Cube = RubiksCell::collect();

    fn new() -> Self {
        RubiksCube3x3 {
           cube: RubiksCube3x3::SOLVED_CUBE
               .iter()
               .map(|vec2d| vec2d
                    .iter()
                    .map(|vec| vec
                         .iter()
                         .copied()
                         .collect())
                    .collect())
               .collect(),
        }
    }

    fn single_u(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 0);
        rotate90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 0);
    }

    fn single_r(&mut self) {
        let mut plane = get_y_z_plane(&mut self.cube, 2);
        rotate90(&mut plane);
        set_y_z_plane(&mut self.cube, &plane, 2);
    }

    fn single_f(&mut self) {
        let mut plane = get_x_y_plane(&mut self.cube, 0);
        rotate90(&mut plane);
        set_y_z_plane(&mut self.cube, &plane, 0);
    }

    fn single_d(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 2);
        rotate90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 2);
    }

    fn single_u_prime(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 0);
        rotate_minus90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 0);
    }
}

pub trait RubiksCube {
    fn solved(&self) -> bool;
    fn get_cube(&self) -> &Vec<Vec<Vec<RubiksCell>>>;
}


impl RubiksCube for RubiksCube3x3 {
    fn solved(&self) -> bool {
        self.cube.iter()
            .zip(&RubiksCube3x3::SOLVED_CUBE)
            .filter(|&(a, b)| a != b).count() == 0
    }

    fn get_cube(&self) -> &Vec<Vec<Vec<RubiksCell>>> {
        &self.cube
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rubiks_cube_solved_when_created() {
        let rc = RubiksCube3x3::new();

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn rubiks_cube_not_solved_when_modified() {
        let mut rc = RubiksCube3x3::new();

        rc.single_u();

        assert_eq!(false, rc.solved());

        rc.single_u_prime();

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_u_solves_if_repeated_four_times() {
        let mut rc = RubiksCube3x3::new();

        for _ in (0..4) {
            rc.single_u();
        }

        assert_eq!(true, rc.solved());
    }

}

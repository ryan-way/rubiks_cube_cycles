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

    pub fn new() -> Self {
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

    pub fn single_u(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 0);
        rotate90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 0);
    }

    pub fn single_r(&mut self) {
        let mut plane = get_y_z_plane(&mut self.cube, 2);
        rotate90(&mut plane);
        set_y_z_plane(&mut self.cube, &plane, 2);
    }

    pub fn single_f(&mut self) {
        let mut plane = get_x_y_plane(&mut self.cube, 0);
        rotate90(&mut plane);
        set_x_y_plane(&mut self.cube, &plane, 0);
    }

    pub fn single_d(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 2);
        rotate90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 2);
    }

    pub fn single_l(&mut self) {
        let mut plane = get_y_z_plane(&mut self.cube, 0);
        rotate90(&mut plane);
        set_y_z_plane(&mut self.cube, &plane, 0);
    }

    pub fn single_b(&mut self) {
        let mut plane = get_x_y_plane(&mut self.cube, 2);
        rotate90(&mut plane);
        set_x_y_plane(&mut self.cube, &plane, 2);
    }

    pub fn single_m(&mut self) {
        let mut plane = get_y_z_plane(&mut self.cube, 1);
        rotate90(&mut plane);
        set_y_z_plane(&mut self.cube, &plane, 1);
    }

    pub fn single_e(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 1);
        rotate90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 1);
    }

    pub fn single_s(&mut self) {
        let mut plane = get_x_y_plane(&mut self.cube, 1);
        rotate90(&mut plane);
        set_x_y_plane(&mut self.cube, &plane, 1);
    }

    pub fn single_u_prime(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 0);
        rotate_minus90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 0);
    }

    pub fn single_r_prime(&mut self) {
        let mut plane = get_y_z_plane(&mut self.cube, 2);
        rotate_minus90(&mut plane);
        set_y_z_plane(&mut self.cube, &plane, 2);
    }

    pub fn single_f_prime(&mut self) {
        let mut plane = get_x_y_plane(&mut self.cube, 0);
        rotate_minus90(&mut plane);
        set_x_y_plane(&mut self.cube, &plane, 0);
    }

    pub fn single_d_prime(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 2);
        rotate_minus90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 2);
    }

    pub fn single_l_prime(&mut self) {
        let mut plane = get_y_z_plane(&mut self.cube, 0);
        rotate_minus90(&mut plane);
        set_y_z_plane(&mut self.cube, &plane, 0);
    }

    pub fn single_b_prime(&mut self) {
        let mut plane = get_x_y_plane(&mut self.cube, 2);
        rotate_minus90(&mut plane);
        set_x_y_plane(&mut self.cube, &plane, 2);
    }

    pub fn single_m_prime(&mut self) {
        let mut plane = get_y_z_plane(&mut self.cube, 1);
        rotate_minus90(&mut plane);
        set_y_z_plane(&mut self.cube, &plane, 1);
    }

    pub fn single_e_prime(&mut self) {
        let mut plane = get_x_z_plane(&mut self.cube, 1);
        rotate_minus90(&mut plane);
        set_x_z_plane(&mut self.cube, &plane, 1);
    }

    pub fn single_s_prime(&mut self) {
        let mut plane = get_x_y_plane(&mut self.cube, 1);
        rotate_minus90(&mut plane);
        set_x_y_plane(&mut self.cube, &plane, 1);
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
    fn single_u_prime_undoes_single_u() {
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

    #[test]
    fn single_r_prime_undoes_single_r() {
        let mut rc = RubiksCube3x3::new();

        rc.single_r();

        assert_eq!(false, rc.solved());

        rc.single_r_prime();

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_r_solves_if_repeated_four_times() {
        let mut rc = RubiksCube3x3::new();

        for _ in (0..4) {
            rc.single_r();
        }

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_f_prime_undoes_single_f() {
        let mut rc = RubiksCube3x3::new();

        rc.single_f();

        assert_eq!(false, rc.solved());

        rc.single_f_prime();

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_f_solves_if_repeated_four_times() {
        let mut rc = RubiksCube3x3::new();

        for _ in (0..4) {
            rc.single_f();
        }

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_d_prime_undoes_single_d() {
        let mut rc = RubiksCube3x3::new();

        rc.single_d();

        assert_eq!(false, rc.solved());

        rc.single_d_prime();

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_d_solves_if_repeated_four_times() {
        let mut rc = RubiksCube3x3::new();

        for _ in (0..4) {
            rc.single_d();
        }

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_l_prime_undoes_single_l() {
        let mut rc = RubiksCube3x3::new();

        rc.single_l();

        assert_eq!(false, rc.solved());

        rc.single_l_prime();

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_l_solves_if_repeated_four_times() {
        let mut rc = RubiksCube3x3::new();

        for _ in (0..4) {
            rc.single_l();
        }

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_b_prime_undoes_single_b() {
        let mut rc = RubiksCube3x3::new();

        rc.single_b();

        assert_eq!(false, rc.solved());

        rc.single_b_prime();

        assert_eq!(true, rc.solved());
    }

    #[test]
    fn single_b_solves_if_repeated_four_times() {
        let mut rc = RubiksCube3x3::new();

        for _ in (0..4) {
            rc.single_b();
        }

        assert_eq!(true, rc.solved());
    }
}

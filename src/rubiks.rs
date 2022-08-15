
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
                [BW, B, GW]
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
        let rc = RubiksCube3x3::new();
    }
}

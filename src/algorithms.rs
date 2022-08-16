use crate::rubiks::*;

pub fn one_of_everything(rc: &mut RubiksCube3x3) {
    rc.single_r();
    rc.single_u();
    rc.single_f();
    rc.single_d();
    rc.single_l();
    rc.single_b();
} 

pub fn one_of_everything_prime(rc: &mut RubiksCube3x3) {
    rc.single_r_prime();
    rc.single_u_prime();
    rc.single_f_prime();
    rc.single_d_prime();
    rc.single_l_prime();
    rc.single_b_prime();
} 

pub fn r_u(rc: &mut RubiksCube3x3) {
    rc.single_r();
    rc.single_u();
}

pub fn one_of_everything_really(rc: &mut RubiksCube3x3) {
    one_of_everything(rc);
    one_of_everything_prime(rc);
}

pub fn basic_move(rc: &mut RubiksCube3x3) {
    rc.single_r();
    rc.single_u();
    rc.single_r_prime();
    rc.single_u_prime();
}

#![feature(test)]

extern crate test;

mod transforms;
mod rubiks;

use std::collections::HashMap;
use rubiks::*;


fn run_ops_until_solved<F: Fn(&mut RubiksCube3x3)-> ()>(ops: &Vec<F>) -> i32 {
    let mut rc = RubiksCube3x3::new();

    let mut count = 0;

    loop {
        for op in ops {
            op(&mut rc);
        }

        count += 1;

        if rc.solved() {
            break;
        }
    }

    count
}

fn search_for_and_run_tests<'a, F: Fn(&mut RubiksCube3x3)>(start_depth: i32,
                                                       depth: i32, 
                                                       operation_pool: &'a HashMap<String, F>,
                                                       ops_name: &mut Vec<String>,
                                                       ops: &mut Vec<&'a F>) {
    if start_depth == depth {
        let count = run_ops_until_solved(ops);
        println!("{0:<10} {1:<10} {2:<10}",
                 format!("depth: {}", depth),
                 &count,
                 &ops_name.join(""));
        return;
    }

    for (op_name, op) in operation_pool.iter() {
        if ops.len() >= 3 {
            if ops_name[ops_name.len()-1] == *op_name
                && ops_name[ops_name.len()-2] == *op_name
                    && ops_name[ops_name.len()-3] == *op_name {
                continue;
            }
        }

        ops_name.push(op_name.clone());
        ops.push(op);
        search_for_and_run_tests(start_depth+1,
                                             depth,
                                             operation_pool,
                                             ops_name,
                                             ops);
        ops_name.pop();
        ops.pop();
    }
}

fn generate_and_run_tests<F: Fn(&mut RubiksCube3x3)>(depth: i32,
                                                     operation_pool: &HashMap<String, F>) {
    let mut ops: Vec<&F> = vec![];
    let mut ops_name: Vec<String> = vec![];
    search_for_and_run_tests(0,
                             depth,
                             operation_pool,
                             &mut ops_name,
                             &mut ops)
}


fn main() {
    let mut operation_pool: HashMap<String, &dyn Fn(&mut RubiksCube3x3)> = HashMap::new();
    operation_pool.insert(String::from("R"), &|rc: &mut RubiksCube3x3| { rc.single_r()});
    operation_pool.insert(String::from("U"), &|rc: &mut RubiksCube3x3| { rc.single_u()});
    operation_pool.insert(String::from("F"), &|rc: &mut RubiksCube3x3| { rc.single_f()});
    operation_pool.insert(String::from("D"), &|rc: &mut RubiksCube3x3| { rc.single_d()});
    operation_pool.insert(String::from("L"), &|rc: &mut RubiksCube3x3| { rc.single_l()});
    operation_pool.insert(String::from("B"), &|rc: &mut RubiksCube3x3| { rc.single_b()});
    operation_pool.insert(String::from("M"), &|rc: &mut RubiksCube3x3| { rc.single_m()});
    operation_pool.insert(String::from("E"), &|rc: &mut RubiksCube3x3| { rc.single_e()});
    operation_pool.insert(String::from("S"), &|rc: &mut RubiksCube3x3| { rc.single_s()});

    operation_pool.insert(String::from("R_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_r_prime()});
    operation_pool.insert(String::from("U_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_u_prime()});
    operation_pool.insert(String::from("F_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_f_prime()});
    operation_pool.insert(String::from("D_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_d_prime()});
    operation_pool.insert(String::from("L_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_l_prime()});
    operation_pool.insert(String::from("B_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_b_prime()});
    operation_pool.insert(String::from("M_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_m_prime()});
    operation_pool.insert(String::from("E_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_e_prime()});
    operation_pool.insert(String::from("S_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_s_prime()});

    for depth in 1..8 {
        generate_and_run_tests(depth, &operation_pool);
    }

}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;

    fn bench_generate_and_run_tests(b: &mut Bencher, iterations: i32) {
        let mut operation_pool: HashMap<String, &dyn Fn(&mut RubiksCube3x3)> = HashMap::new();
        operation_pool.insert(String::from("R"), &|rc: &mut RubiksCube3x3| { rc.single_r()});
        operation_pool.insert(String::from("U"), &|rc: &mut RubiksCube3x3| { rc.single_u()});
        operation_pool.insert(String::from("F"), &|rc: &mut RubiksCube3x3| { rc.single_f()});
        operation_pool.insert(String::from("D"), &|rc: &mut RubiksCube3x3| { rc.single_d()});
        operation_pool.insert(String::from("L"), &|rc: &mut RubiksCube3x3| { rc.single_l()});
        operation_pool.insert(String::from("B"), &|rc: &mut RubiksCube3x3| { rc.single_b()});

        operation_pool.insert(String::from("R_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_r_prime()});
        operation_pool.insert(String::from("U_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_u_prime()});
        operation_pool.insert(String::from("F_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_f_prime()});
        operation_pool.insert(String::from("D_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_d_prime()});
        operation_pool.insert(String::from("L_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_l_prime()});
        operation_pool.insert(String::from("B_Prime"), &|rc: &mut RubiksCube3x3| { rc.single_b_prime()});

        b.iter(|| { 
            generate_and_run_tests(iterations, &operation_pool)
        });
    }

    #[bench]
    fn bench_generate_and_run_tests_1(b: &mut Bencher) {
        bench_generate_and_run_tests(b, 1);
    }

    #[bench]
    fn bench_generate_and_run_tests_2(b: &mut Bencher) {
        bench_generate_and_run_tests(b, 2);
    }

    #[bench]
    fn bench_generate_and_run_tests_3(b: &mut Bencher) {
        bench_generate_and_run_tests(b, 3);
    }

    #[bench]
    fn bench_generate_and_run_tests_4(b: &mut Bencher) {
        bench_generate_and_run_tests(b, 4);
    }


    #[bench]
    fn sleep_for_2_sec(b: &mut Bencher) {
    }
}

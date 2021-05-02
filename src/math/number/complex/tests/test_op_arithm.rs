use super::*;


fn generate_data() -> Vec<Complex> {
    vec![
        Complex::from( 0.,  0.),
        Complex::from( 1.,  0.),
        Complex::from(-1.,  0.),
        Complex::from( 0.,  1.),
        Complex::from( 0., -1.),
        Complex::from( 1.,  1.),
        Complex::from(-1., -1.)
    ]
}



#[test]
fn test_op_arithm_add() {
    macro_rules! assert_op {
        ($z1:ident, $z2:ident) => {
            let z_th = $z1 + $z2;
            
            let z_exp = Complex::from(
                $z1.real() + $z2.real(),
                $z1.imag() + $z2.imag()
            );

            assert_eq!(z_th.real(), z_exp.real());
            assert_eq!(z_th.imag(), z_exp.imag());
        };
    }

    let gen_data = generate_data();
    

    for z1 in &gen_data {
        for z2 in &gen_data {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in gen_data.clone() {
        for z2 in &gen_data {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in &gen_data {
        for z2 in gen_data.clone() {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in gen_data.clone() {
        for z2 in gen_data.clone() {
            assert_op!(z1, z2);
        }
    }
}



#[test]
fn test_op_arithm_sub() {
    macro_rules! assert_op {
        ($z1:ident, $z2:ident) => {
            let z_th = $z1 - $z2;
            
            let z_exp = Complex::from(
                $z1.real() - $z2.real(),
                $z1.imag() - $z2.imag()
            );

            assert_eq!(z_th.real(), z_exp.real());
            assert_eq!(z_th.imag(), z_exp.imag());
        };
    }

    let gen_data = generate_data();

    for z1 in &gen_data {
        for z2 in &gen_data {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in gen_data.clone() {
        for z2 in &gen_data {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in &gen_data {
        for z2 in gen_data.clone() {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in gen_data.clone() {
        for z2 in gen_data.clone() {
            assert_op!(z1, z2);
        }
    }
}



#[test]
fn test_op_arithm_mul() {
    macro_rules! assert_op {
        ($z1:ident, $z2:ident) => {
            let z_th = $z1 * $z2;
            
            let z_exp = Complex::from(
                $z1.real() * $z2.real() - $z1.imag() * $z2.imag(),
                $z1.real() * $z2.imag() + $z1.imag() * $z2.real(),
            );

            assert_eq!(z_th.real(), z_exp.real());
            assert_eq!(z_th.imag(), z_exp.imag());
        };
    }


    let gen_data = generate_data();
    

    for z1 in &gen_data {
        for z2 in &gen_data {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in gen_data.clone() {
        for z2 in &gen_data {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in &gen_data {
        for z2 in gen_data.clone() {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in gen_data.clone() {
        for z2 in gen_data.clone() {
            assert_op!(z1, z2);
        }
    }
}



#[test]
fn test_op_arithm_div() {
    macro_rules! assert_op {
        ($z1:ident, $z2:ident) => {
            if $z2.real() == 0. && 
               $z2.imag() == 0. { 
                continue; 
            }

            let z_th = $z1 / $z2;
            
            let denom = ($z2.real() * $z2.real()) + ($z2.imag() * $z2.imag());

            let z_exp = Complex::from(
                ($z1.real() * $z2.real() + $z1.imag() * $z2.imag()) / denom,
                ($z1.imag() * $z2.real() - $z1.real() * $z2.imag()) / denom,
            );

            assert_eq!(z_th.real(), z_exp.real());
            assert_eq!(z_th.imag(), z_exp.imag());
        };
    }


    let gen_data = generate_data();
    

    for z1 in &gen_data {
        for z2 in &gen_data {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in gen_data.clone() {
        for z2 in &gen_data {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in &gen_data {
        for z2 in gen_data.clone() {
            assert_op!(z1, z2);
        }
    }
    
    for z1 in gen_data.clone() {
        for z2 in gen_data.clone() {
            assert_op!(z1, z2);
        }
    }
}
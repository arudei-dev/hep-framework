use super::*;

#[test]
fn case_real_basis_into() {
    let list_of_vec4: Vec<[f64; 4]> = vec![
        [ 1.,  1.,  1.,  1.,],
        [-1., -1., -1., -1.,],
        [ 1., -1., -1., -1.,],

        [-1.,  1.,  1.,  1.,],
        [ 1., -1.,  1.,  1.,],
        [ 1.,  1., -1.,  1.,],
        [ 1.,  1.,  1., -1.,],

        [-1., -1.,  1.,  1.,],
        [-1.,  1., -1.,  1.,],
        [-1.,  1.,  1., -1.,],

        [ 1., -1., -1.,  1.,],
        [ 1.,  1., -1., -1.,],
        [ 1., -1.,  1., -1.,],

        [-1., -1., -1.,  1.,],
        [-1.,  1., -1., -1.,],
        [-1., -1.,  1., -1.,],
    ];

    for vec4 in &list_of_vec4 {
        let vec4_co = Lorentz4::<f64>::new(*vec4, Covariant);

        let vec4_contra = vec4_co.to_basis(Contravariant);

        assert_eq!(vec4_contra[0],  vec4_co[0]);
        assert_eq!(vec4_contra[1], -vec4_co[1]);
        assert_eq!(vec4_contra[2], -vec4_co[2]);
        assert_eq!(vec4_contra[3], -vec4_co[3]);
    }

    for vec4 in &list_of_vec4 {
        let vec4_contra = Lorentz4::<f64>::new(*vec4, Contravariant);

        let vec4_co = vec4_contra.to_basis(Covariant);

        assert_eq!(vec4_co[0],  vec4_contra[0]);
        assert_eq!(vec4_co[1], -vec4_contra[1]);
        assert_eq!(vec4_co[2], -vec4_contra[2]);
        assert_eq!(vec4_co[3], -vec4_contra[3]);
    }
}
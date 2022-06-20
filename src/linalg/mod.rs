/*
 * Copyright (c) 2022 bernik86.
 *
 * This file is part of rusty-arrays 
 * (see https://github.com/bernik86/rusty-arrays).
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::array2d;
use crate::array2d::Array2d;
use num;

pub fn norm<T>(arr: &Array2d<T>, axis: i8) -> f64
where
    T: num::ToPrimitive,
    T: num::Num,
    T: std::ops::Mul<Output = T>,
    T: Copy,
    T: Clone,
{
    match axis {
        -1 => {
            let arr_norm: f64 = (**arr)
                .iter()
                .map(|el| (*el) * (*el))
                .fold(0.0_f64, |n, el| n + el.to_f64().unwrap());
            arr_norm.sqrt()
        }
        _ => panic!("Norm not implemented for axis {}!", axis),
    }
}

#[allow(non_snake_case)]
pub fn plu(arr: &Array2d<f64>) -> Option<(Array2d<f64>, Array2d<f64>, Array2d<f64>)>
{
    let (n_rows, n_cols) = arr.shape();
    assert_eq!(n_rows, n_cols);
    let mut P = Array2d::<f64>::identity(n_rows, n_rows);
    let mut L = Array2d::<f64>::identity(n_rows, n_rows);
    let mut U = Array2d::new_from_array(arr);

    for i in 0..(n_rows - 1) {
        let mut Ln = Array2d::<f64>::identity(n_rows, n_rows);

        // Swap lines if U[i, i] is 0
        let mut n: usize = 1;
        if U[(i, i)] == 0.0 {
            while U[(i, i + n)] == 0.0 {
                n += 1;
                if i + n >= n_rows {
                    return None;
                }
            }
            array2d::swapping::swap_rows_inplace(&mut U, i, i + n);
            array2d::swapping::swap_rows_non_diag_inplace(&mut L, i, i + n);
            array2d::swapping::swap_rows_inplace(&mut P, i, i + n);
        }

        //Swap lines if U[i, i] < U[i + 1, i]
        for j in (i + 1)..n_rows {
            if U[(j, i)] > U[(j - 1, i)] {
                array2d::swapping::swap_rows_inplace(&mut U, j, j - 1);
                array2d::swapping::swap_rows_non_diag_inplace(&mut L, j, j - 1);
                array2d::swapping::swap_rows_inplace(&mut P, j, j - 1);
            }
        }

        //Update L and U
        for j in (i + 1)..n_rows {
            Ln[(j, i)] = U[(j, i)] / U[(i, i)];
        }
        let L_inv = inv(&Ln).unwrap();
        U = L_inv.mat_mul_iter(&U);
        array2d::numerics::add_non_diag_inplace(&mut L, &Ln);
    }
    Some((P, L, U))
}

#[allow(non_snake_case)]
pub fn gauss_jordan(coeff: &Array2d<f64>, rhs: &Array2d<f64>)
    -> Option<Array2d<f64>>
{
    let (n_rows, n_cols) = coeff.shape();
    let (rhs_rows, rhs_cols) = rhs.shape();
    assert_eq!(n_rows, n_cols);
    assert_eq!(n_rows, rhs_rows);
    let mut C = Array2d::new_from_array(coeff);
    let mut R = Array2d::new_from_array(rhs);

    for i in 0..n_rows {
        if C[(i, i)] == 0.0 {
            let col = C.col(i);
            let mut k_iter =
                (0..n_rows).into_iter().filter(|k| col[*k].abs() > 0.0001);

            match k_iter.next() {
                Some(k) => {
                    array2d::swapping::swap_rows_inplace(&mut C, i, k);
                    array2d::swapping::swap_rows_inplace(&mut R, i, k);
                }
                None => return None,
            }
        }

        for j in 0..n_rows {
            if i != j {
                let ratio: f64 = C[(j, i)] / C[(i, i)];

                for k in 0..n_rows {
                    C[(j, k)] -= ratio * C[(i, k)];
                }
                for k in 0..rhs_cols {
                    R[(j, k)] -= ratio * R[(i, k)];
                }
            }
        }
    }
    for i in 0..n_rows {
        for k in 0..rhs_cols {
            R[(i, k)] /= C[(i, i)];
        }
    }
    Some(R)
}

#[allow(non_snake_case)]
pub fn inv(arr: &Array2d<f64>) -> Option<Array2d<f64>>
{
    let (n_rows, n_cols) = arr.shape();
    assert_eq!(n_rows, n_cols);
    let I = Array2d::<f64>::identity(n_rows, n_rows);
    gauss_jordan(arr, &I)
}

#[allow(non_snake_case)]
pub fn det(arr: &Array2d<f64>) -> f64
{
    let (p, _, u) = plu(arr).unwrap();
    let sum_diag_P: f64 = p.diag_iter().sum();
    let nswaps: f64 = p.shape().0 as f64 - sum_diag_P - 1.0;
    let det_P = (-1.0_f64).powf(nswaps);
    let det_L: f64 = 1.0; // diagonal elements of L from plu decomposition are all 1
    let det_U: f64 = u.diag_iter().product();

    det_P * det_L * det_U
}

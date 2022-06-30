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

use rusty_arrays::array2d::Array2d;
use rusty_arrays::linalg;

#[test]
fn norm()
{
    let v1 = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0,
        15.0, 16.0,
    ];
    let arr1 = Array2d::<f64>::new(v1, 4, 4);
    let res = Array2d::<f64>::new_fill(38.67815921162743, 1, 1);
    assert_eq!(linalg::norm(&arr1, -1), res);

    let v_res = vec![
        5.477225575051661,
        13.19090595827292,
        21.118712081942874,
        29.086079144497972,
    ];
    let res = Array2d::<f64>::new(v_res, 4, 1);
    assert_eq!(linalg::norm(&arr1, 0), res);

    let v_res = vec![
        16.61324772583615,
        18.33030277982336,
        20.09975124224178,
        21.908902300206645,
    ];
    let res = Array2d::<f64>::new(v_res, 4, 1);
    assert_eq!(linalg::norm(&arr1, 1), res);
}

#[allow(non_snake_case)]
#[test]
fn qr()
{
    let v1 = vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0];
    let arr1 = Array2d::<f64>::new(v1, 3, 3);
    let (Q, R) = linalg::gram_schmidt(&arr1).unwrap();
    Q.print();
    R.print();

    let q_res = vec![
        0.8571428571428571,
        -0.3942857142857143,
        -0.33142857142857146,
        0.42857142857142855,
        0.9028571428571428,
        0.034285714285714364,
        -0.2857142857142857,
        0.17142857142857143,
        -0.9428571428571428,
    ];
    let Q_res = Array2d::<f64>::new(q_res, 3, 3);

    let r_res = vec![
        13.999999999999998,
        20.999999999999996,
        -14.000000000000002,
        -6.661338147750939e-16,
        175.0,
        -70.0,
        0.0,
        1.4210854715202004e-14,
        34.99999999999999,
    ];
    let R_res = Array2d::<f64>::new(r_res, 3, 3);

    assert_eq!(Q, Q_res);
    assert_eq!(R, R_res);
}

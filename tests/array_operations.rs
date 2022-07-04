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

use rusty_arrays::array2d;
use rusty_arrays::array2d::Array2d;

#[test]
fn transpose()
{
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let v2 = vec![1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15, 4, 8, 12, 16];
    let arr1 = Array2d::<usize>::new(v1, 4, 4);
    let mut arr2 = Array2d::<usize>::new(v2, 4, 4);
    assert_eq!(arr1, arr2.transpose());
    arr2.transpose_inplace();
    assert_eq!(arr1, arr2);

    let v1 = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let v2 = vec![0, 4, 1, 5, 2, 6, 3, 7];
    let mut arr1 = Array2d::<usize>::new(v1, 2, 4);
    let arr2 = Array2d::<usize>::new(v2, 4, 2);
    arr1.transpose_inplace();
    assert_eq!(arr1, arr2);
}

#[test]
fn mat_mul()
{
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let v2 = vec![1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15, 4, 8, 12, 16];
    let arr1 = Array2d::<usize>::new(v1, 4, 4);
    let arr2 = Array2d::<usize>::new(v2, 4, 4);
    let identity = Array2d::<usize>::identity(4, 4);
    assert_eq!(arr1, arr1.mat_mul_iter(&identity));
    assert_eq!(arr1, arr1.mat_mul_iter(&identity));

    let v_res = vec![
        30, 70, 110, 150, 70, 174, 278, 382, 110, 278, 446, 614, 150, 382, 614, 846,
    ];
    let res = Array2d::<usize>::new(v_res, 4, 4);
    assert_eq!(arr1.mat_mul_iter(&arr2), res);

    let v_res = vec![
        276, 304, 332, 360, 304, 336, 368, 400, 332, 368, 404, 440, 360, 400, 440,
        480,
    ];
    let res = Array2d::<usize>::new(v_res, 4, 4);
    assert_eq!(arr2.mat_mul_iter(&arr1), res);
}

#[test]
fn dot()
{
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    assert_eq!(array2d::numerics::dot(&v1, &v2), 14);

    let v1 = vec![1.0, 2.0, 3.0];
    let v2 = vec![1.0, 2.0, 3.0];
    assert_eq!(array2d::numerics::dot(&v1, &v2), 14.0);

    let v1 = vec![1.5, 2.2, 1.0, 0.3, 7.2];
    let v2 = vec![0.4, 8.4, 6.7, 0.1, -9.0];
    assert_eq!(array2d::numerics::dot(&v1, &v2), -38.989999999999995);
}

#[test]
fn set_row()
{
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut arr1 = Array2d::<usize>::new(v1, 4, 4);

    let v2 = vec![0, 0, 0, 0];
    arr1.set_row(2, v2);
    assert_eq!(arr1.row(2), vec![0, 0, 0, 0]);
}

#[test]
fn is_upper_triangular()
{
    let vec = vec![
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
    let arr = Array2d::<f64>::new(vec, 3, 3);
    assert!(arr.is_upper_triangular(1.0E-13));

    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let arr = Array2d::<usize>::new(vec, 4, 4);
    assert!(!arr.is_upper_triangular(1.0E-13));
}

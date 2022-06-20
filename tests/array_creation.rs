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

use arrays::array2d;
use arrays::array2d::Array2d;

#[test]
fn create_identity_float()
{
    let id: Vec<f64> = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    let id = Array2d::<f64>::new(id, 3, 3);
    let id_constr = Array2d::<f64>::identity(3, 3);
    assert_eq!(id, id_constr);

    let id: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    let id = Array2d::<f32>::new(id, 3, 3);
    let id_constr = Array2d::<f32>::identity(3, 3);
    assert_eq!(id, id_constr);
}

#[test]
fn create_identity_int()
{
    let id: Vec<usize> = vec![1, 0, 0, 0, 1, 0, 0, 0, 1];
    let id = Array2d::<usize>::new(id, 3, 3);
    let id_constr = Array2d::<usize>::identity(3, 3);
    assert_eq!(id, id_constr);

    let id: Vec<isize> = vec![1, 0, 0, 0, 1, 0, 0, 0, 1];
    let id = Array2d::<isize>::new(id, 3, 3);
    let id_constr = Array2d::<isize>::identity(3, 3);
    assert_eq!(id, id_constr);

    let id: Vec<u8> = vec![1, 0, 0, 0, 1, 0, 0, 0, 1];
    let id = Array2d::<u8>::new(id, 3, 3);
    let id_constr = Array2d::<u8>::identity(3, 3);
    assert_eq!(id, id_constr);

    let id: Vec<i8> = vec![1, 0, 0, 0, 1, 0, 0, 0, 1];
    let id = Array2d::<i8>::new(id, 3, 3);
    let id_constr = Array2d::<i8>::identity(3, 3);
    assert_eq!(id, id_constr);

    let id: Vec<u64> = vec![1, 0, 0, 0, 1, 0, 0, 0, 1];
    let id = Array2d::<u64>::new(id, 3, 3);
    let id_constr = Array2d::<u64>::identity(3, 3);
    assert_eq!(id, id_constr);
}

#[test]
fn create_zeros_float()
{
    let zero = vec![0.0_f32; 9];
    let zero = Array2d::new(zero, 3, 3);
    let zero_constr = Array2d::<f32>::zeros(3, 3);
    assert_eq!(zero, zero_constr);

    let zero = vec![0.0_f64; 9];
    let zero = Array2d::new(zero, 3, 3);
    let zero_constr = Array2d::<f64>::zeros(3, 3);
    assert_eq!(zero, zero_constr);
}

#[test]
fn create_zeros_int()
{
    let zero = vec![0_usize; 9];
    let zero = Array2d::new(zero, 3, 3);
    let zero_constr = Array2d::<usize>::zeros(3, 3);
    assert_eq!(zero, zero_constr);

    let zero = vec![0_i64; 9];
    let zero = Array2d::new(zero, 3, 3);
    let zero_constr = Array2d::<i64>::zeros(3, 3);
    assert_eq!(zero, zero_constr);
}

#[test]
fn create_float_arrays()
{
    let arr: Vec<f64> = vec![1.0, 2.0, 3.7, 3.0, 1.4, 3.0, 7.0, 3.1, 4.1];
    let arr = Array2d::<f64>::new(arr, 3, 3);
    assert_eq!(arr.shape(), (3, 3));
    assert_eq!(
        *arr.get_data(),
        vec![1.0_f64, 2.0, 3.7, 3.0, 1.4, 3.0, 7.0, 3.1, 4.1]
    );

    let arr: Vec<f64> = vec![3.2, 4.6, 9.0, 1.0, 2.2, 10.13];
    let arr = Array2d::<f64>::new(arr, 2, 3);
    assert_eq!(arr.shape(), (2, 3));
    assert_eq!(*arr.get_data(), vec![3.2, 4.6, 9.0, 1.0, 2.2, 10.13]);
}

#[test]
fn create_int_arrays()
{
    let arr: Vec<i64> = vec![1, 2, 7, 0, 4, 3, 0, 3, 1];
    let arr = Array2d::<i64>::new(arr, 3, 3);
    assert_eq!(arr.shape(), (3, 3));
    assert_eq!(*arr.get_data(), vec![1, 2, 7, 0, 4, 3, 0, 3, 1]);

    let arr: Vec<i64> = vec![3, 4, 9, 1, 2, 10];
    let arr = Array2d::<i64>::new(arr, 2, 3);
    assert_eq!(arr.shape(), (2, 3));
    assert_eq!(*arr.get_data(), vec![3, 4, 9, 1, 2, 10]);
}

#[test]
fn create_str_arrays()
{
    let arr: Vec<&str> = vec!["1", "2", "A", " p", "0x", "ö", "  ", "bb", "^"];
    let arr = Array2d::<&str>::new(arr, 3, 3);
    assert_eq!(arr.shape(), (3, 3));
    assert_eq!(
        *arr.get_data(),
        vec!["1", "2", "A", " p", "0x", "ö", "  ", "bb", "^"]
    );
}

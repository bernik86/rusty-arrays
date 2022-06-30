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

use rusty_arrays::vector_functions;

#[test]
fn add()
{
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![1, 2, 3, 4];
    vector_functions::add_vec_inplace(&mut v1, &v2);
    assert_eq!(v1, vec![2, 4, 6, 8]);

    let mut v1 = vec![1.0, 2.0, 3.0, 4.0];
    let v2 = vec![1.0, 2.0, 3.0, 4.0];
    vector_functions::add_vec_inplace(&mut v1, &v2);
    assert_eq!(v1, vec![2.0, 4.0, 6.0, 8.0]);
}

#[test]
fn sub()
{
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![1, 2, 3, 4];
    vector_functions::sub_vec_inplace(&mut v1, &v2);
    assert_eq!(v1, vec![0, 0, 0, 0]);

    let mut v1 = vec![1.0, 2.0, 3.0, 4.0];
    let v2 = vec![1.0, 2.0, 3.0, 4.0];
    vector_functions::sub_vec_inplace(&mut v1, &v2);
    assert_eq!(v1, vec![0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn scalar_div()
{
    let mut v1 = vec![1.0, 2.0, 3.0, 4.0];
    vector_functions::scalar_div_inplace(&mut v1, 2.0);
    assert_eq!(v1, vec![0.5, 1.0, 1.5, 2.0]);
}

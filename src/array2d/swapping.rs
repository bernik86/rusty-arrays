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

use super::Array2d;

pub fn swap_rows_inplace<T>(arr: &mut Array2d<T>, i: usize, j: usize)
{
    let row_one_start = arr.cols * i;
    let row_two_start = arr.cols * j;

    for k in 0..arr.cols {
        (**arr).swap(row_one_start + k, row_two_start + k);
    }
}

/// Swaps the elements of two rows that are not on the diagonal of either
/// of the two rows.
pub fn swap_rows_non_diag_inplace<T>(arr: &mut Array2d<T>, i: usize, j: usize)
{
    let row_one_start = arr.cols * i;
    let row_two_start = arr.cols * j;

    for k in 0..arr.cols {
        if k != i && k != j {
            //arr.data.swap(row_one_start + k, row_two_start + k);
            (**arr).swap(row_one_start + k, row_two_start + k);
        }
    }
}

pub fn swap_cols_inplace<T>(arr: &mut Array2d<T>, i: usize, j: usize)
{
    for k in 0..arr.rows {
        arr.data.swap(i + k * arr.cols, j + k * arr.cols);
    }
}

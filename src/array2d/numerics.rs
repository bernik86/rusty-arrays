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

use core::fmt::Debug;
use num;
use std::clone::Clone;
use std::cmp::Ord;

use super::Array2d;

impl<
        T: num::FromPrimitive
            + num::Num
            + Debug
            + Clone
            + Copy
            + std::ops::AddAssign
            + std::iter::Sum,
    > Array2d<T>
{
    pub fn zeros(rows: usize, cols: usize) -> Array2d<T>
    {
        let n_el = rows * cols;
        let temp: Vec<T> = vec![num::Zero::zero(); n_el];

        Array2d {
            data: temp,
            rows,
            cols,
        }
    }

    pub fn identity(rows: usize, cols: usize) -> Array2d<T>
    {
        let n_el = rows * cols;
        let mut identity = Self::zeros(rows, cols);
        for i in (0..n_el).step_by(cols + 1) {
            identity.data[i] = num::One::one();
        }

        identity
    }

    pub fn mat_mul(&self, other: &Self) -> Array2d<T>
    {
        let trans = &other.transpose();
        let mut prod = Self::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..trans.rows {
                prod[(i, j)] = dot(&self.row(i), &trans.row(j));
            }
        }
        prod
    }

    pub fn mat_mul_iter(&self, other: &Self) -> Array2d<T>
    {
        let trans = &other.transpose();
        let mut prod = Self::zeros(self.rows, other.cols);
        let iter = self.iter();

        let p = iter.flat_map(|row| trans.iter().map(move |col| dot(&row, &col)));
        prod.data = p.collect::<Vec<T>>();
        prod
    }

    pub fn trace(&self) -> T
    {
        self.diag_iter().sum()
    }

    /// Rather brute-force approach fof calculating the determinant.
    /// Could be replaced by PLU decomposition.
    pub fn det(&self) -> T
    {
        assert_eq!(self.rows, self.cols);
        if self.rows == 2 {
            return self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)];
        }
        let mut iter = self.iter();
        let row = iter.next().unwrap();
        let mut det = num::Zero::zero();
        let mut fac;
        for (i, r) in row.iter().enumerate() {
            if i % 2 == 0 {
                fac = T::from_usize(1).unwrap();
            } else {
                fac = T::from_isize(-1).unwrap();
            }
            det += fac * *r * self.get_subarray(0, i).det();
        }
        det
    }
}

pub fn dot<T: num::Num + Debug + Clone + Copy + std::ops::AddAssign>(
    vec_1: &Vec<T>,
    vec_2: &Vec<T>,
) -> T
{
    assert_eq!(vec_1.len(), vec_2.len());
    let mut sum: T = num::Zero::zero();
    for (v1, v2) in vec_1.iter().zip(vec_2.iter()) {
        sum += *v1 * *v2;
    }
    sum
}

pub fn imin<'a, T: 'a + Ord + Clone + Copy + Debug>(
    arr: &'a Array2d<T>,
) -> Option<&'a T>
{
    (**arr).iter().min()
}

pub fn imax<'a, T: 'a + Ord + Clone + Copy + Debug>(
    arr: &'a Array2d<T>,
) -> Option<&'a T>
{
    (**arr).iter().max()
}

pub fn min<'a, T: 'a + PartialOrd + Clone + Copy + Debug>(
    arr: &'a Array2d<T>,
) -> Option<&'a T>
{
    (**arr)
        .iter()
        .min_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
}

pub fn max<'a, T: 'a + PartialOrd + Clone + Copy + Debug>(
    arr: &'a Array2d<T>,
) -> Option<&'a T>
{
    (**arr)
        .iter()
        .max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
}

pub fn iargmin<'a, T: 'a + Ord + Clone + Copy + Debug>(
    arr: &'a Array2d<T>,
) -> Option<(usize, &T)>
{
    (**arr).iter().enumerate().min_by_key(|&(_, el)| el)
}

pub fn iargmax<'a, T: 'a + Ord + Clone + Copy + Debug>(
    arr: &'a Array2d<T>,
) -> Option<(usize, &T)>
{
    (**arr).iter().enumerate().max_by_key(|&(_, el)| el)
}

pub fn argmin<'a, T: 'a + PartialOrd + Clone + Copy + Debug>(
    arr: &'a Array2d<T>,
) -> Option<(usize, &T)>
{
    (**arr)
        .iter()
        .enumerate()
        .min_by(|&(_, lhs), &(_, rhs)| lhs.partial_cmp(rhs).unwrap())
}

pub fn argmax<'a, T: 'a + PartialOrd + Clone + Copy + Debug>(
    arr: &'a Array2d<T>,
) -> Option<(usize, &T)>
{
    (**arr)
        .iter()
        .enumerate()
        .max_by(|&(_, lhs), &(_, rhs)| lhs.partial_cmp(rhs).unwrap())
}

pub fn add_non_diag_inplace<T: Copy + Clone + std::ops::AddAssign>(
    lhs: &mut Array2d<T>,
    rhs: &Array2d<T>,
)
{
    assert_eq!(lhs.rows, rhs.rows);
    assert_eq!(lhs.cols, rhs.cols);

    let mut diag_idx = (0..lhs.data.len()).step_by(lhs.cols + 1);
    let mut skip = diag_idx.next().unwrap();
    for i in 0..lhs.data.len() {
        if i == skip {
            match diag_idx.next() {
                Some(idx) => skip = idx,
                None => continue,
            }
            continue;
        }
        lhs.data[i] += rhs.data[i];
    }
}

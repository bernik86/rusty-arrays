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
use std::ops::Neg;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use super::Array2d;

// Scalar operators
// Implementation of elementwise addition
impl<T: Add<Output = T> + Copy> Add<T> for &Array2d<T>
{
    type Output = Array2d<T>;
    fn add(self, rhs: T) -> Array2d<T>
    {
        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for el in self.data.iter() {
            vec.push(*el + rhs);
        }
        Array2d {
            data: vec,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

// Implementation of elementwise subtraction
impl<T: Sub<Output = T> + Copy> Sub<T> for &Array2d<T>
{
    type Output = Array2d<T>;
    fn sub(self, rhs: T) -> Array2d<T>
    {
        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for el in self.data.iter() {
            vec.push(*el - rhs);
        }
        Array2d {
            data: vec,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

// Implementation of elementwise multiplication
impl<T: Mul<Output = T> + Copy> Mul<T> for &Array2d<T>
{
    type Output = Array2d<T>;
    fn mul(self, rhs: T) -> Array2d<T>
    {
        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for el in self.data.iter() {
            vec.push(*el * rhs);
        }
        Array2d {
            data: vec,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

// Implementation of elementwise division
impl<
        T: Div<Output = T>
            + Copy
            + Debug
            + num::Num
            + num::FromPrimitive
            + num::ToPrimitive,
    > Div<T> for &Array2d<T>
{
    type Output = Array2d<T>;
    fn div(self, rhs: T) -> Array2d<T>
    {
        assert!(rhs != num::Zero::zero());
        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for el in self.data.iter() {
            let el = T::to_f64(el).unwrap();
            let rhs = T::to_f64(&rhs).unwrap();
            vec.push(T::from_f64(el / rhs).unwrap());
        }
        Array2d {
            data: vec,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

//Coumpund assignment opertators
// Implementation of elementwise addition
impl<T: AddAssign + Copy> AddAssign<T> for Array2d<T>
{
    fn add_assign(&mut self, rhs: T)
    {
        for i in 0..self.data.len() {
            self.data[i] += rhs;
        }
    }
}

// Implementation of elementwise subtraction
impl<T: SubAssign + Copy> SubAssign<T> for Array2d<T>
{
    fn sub_assign(&mut self, rhs: T)
    {
        for i in 0..self.data.len() {
            self.data[i] -= rhs;
        }
    }
}

// Implementation of elementwise multiplication
impl<T: MulAssign + Copy> MulAssign<T> for Array2d<T>
{
    fn mul_assign(&mut self, rhs: T)
    {
        for i in 0..self.data.len() {
            self.data[i] *= rhs;
        }
    }
}

// Implementation of elementwise division
impl<
        T: DivAssign + Copy + Debug + num::Num + num::FromPrimitive + num::ToPrimitive,
    > DivAssign<T> for Array2d<T>
{
    fn div_assign(&mut self, rhs: T)
    {
        assert!(rhs != num::Zero::zero());
        for i in 0..self.data.len() {
            let el = T::to_f64(&self.data[i]).unwrap();
            let rhs = T::to_f64(&rhs).unwrap();
            self.data[i] = T::from_f64(el / rhs).unwrap();
        }
    }
}

// unary neg operator
// Implementation of elementwise negation
impl<T: Neg<Output = T> + Copy> Neg for Array2d<T>
{
    type Output = Array2d<T>;
    fn neg(self) -> Array2d<T>
    {
        let mut vec = Vec::with_capacity(self.rows * self.cols);
        for el in self.data {
            vec.push(-el);
        }
        Array2d {
            data: vec,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

//Array operators
// Implementation of elementwise addition of two arrays
impl<T: Add<Output = T> + Copy> Add<&Array2d<T>> for &Array2d<T>
{
    type Output = Array2d<T>;
    fn add(self, rhs: &Array2d<T>) -> Array2d<T>
    {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        let vec = (0..self.data.len())
            .map(|i| self.data[i] + rhs.data[i])
            .collect::<Vec<T>>();
        Array2d {
            data: vec,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

// Implementation of elementwise addition of two arrays
impl<T: Sub<Output = T> + Copy> Sub<&Array2d<T>> for &Array2d<T>
{
    type Output = Array2d<T>;
    fn sub(self, rhs: &Array2d<T>) -> Array2d<T>
    {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        let vec = (0..self.data.len())
            .map(|i| self.data[i] - rhs.data[i])
            .collect::<Vec<T>>();
        Array2d {
            data: vec,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

// Array assignment operators
// Implementation of elementwise addition of two arrays
impl<T: AddAssign + Copy> AddAssign<&Array2d<T>> for Array2d<T>
{
    fn add_assign(&mut self, rhs: &Array2d<T>)
    {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        for i in 0..self.data.len() {
            self.data[i] += rhs.data[i];
        }
    }
}

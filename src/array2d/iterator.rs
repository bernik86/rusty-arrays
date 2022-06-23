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
use core::fmt::Debug;
use std::clone::Clone;
use std::slice::Chunks;

pub struct RowIter<'a, T>
{
    arr: Chunks<'a, T>,
}

pub struct DiagIter<'a, T>
{
    data:  &'a Vec<T>,
    start: usize,
    stop:  usize,
    step:  usize,
}

impl<'a, T: Debug + Clone + Copy> Array2d<T>
{
    pub fn iter(&self) -> RowIter<T>
    {
        let iter = RowIter {
            arr: self.data.chunks(self.cols),
        };
        iter
    }

    pub fn diag_iter(&'a self) -> DiagIter<'a, T>
    {
        let n_el = self.rows * self.cols;
        DiagIter {
            data:  &self.data,
            start: 0,
            stop:  n_el,
            step:  self.cols + 1,
        }
    }
}

impl<'a, T: 'a + Debug + Clone + Copy> IntoIterator for &'a Array2d<T>
{
    type Item = Vec<T>;
    type IntoIter = RowIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter
    {
        self.iter()
    }
}

impl<'a, T: 'a + Debug + Clone + Copy> Iterator for RowIter<'a, T>
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>>
    {
        let next = self.arr.next()?;
        Some(next.to_vec())
    }
}

impl<'a, T: Debug + Clone + Copy> Iterator for DiagIter<'a, T>
{
    type Item = T;
    fn next(&mut self) -> Option<T>
    {
        if self.start >= self.stop {
            return None;
        }
        let el_diag = Some(self.data[self.start]);
        self.start += self.step;
        el_diag
    }
}

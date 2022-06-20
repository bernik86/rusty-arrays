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
use std::clone::Clone;
use std::ops::{Index, IndexMut};

use super::Array2d;

// Indexing via tuple of usize
impl<T: Debug + Clone + Copy> Index<(usize, usize)> for Array2d<T>
{
    type Output = T;
    fn index(&self, idx: (usize, usize)) -> &Self::Output
    {
        assert!(idx.0 < self.rows);
        assert!(idx.1 < self.cols);

        let i_el = self.cols * idx.0 + idx.1;
        assert!(i_el < self.data.len());

        &self.data[i_el]
    }
}

impl<T: Debug + Clone + Copy> IndexMut<(usize, usize)> for Array2d<T>
{
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output
    {
        assert!(idx.0 < self.rows);
        assert!(idx.1 < self.cols);

        let i_el = self.cols * idx.0 + idx.1;
        assert!(i_el < self.data.len());

        &mut self.data[i_el]
    }
}

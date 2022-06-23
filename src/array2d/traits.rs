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

use std::fmt;
use std::ops::{Deref, DerefMut};

use super::Array2d;

impl<T> Deref for Array2d<T>
{
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T>
    {
        &self.data
    }
}

/// ATTENTION: this implementation of DerefMut is a bit dangerous. There
/// is no check whether the new vector has the same length. Replacing the
/// data with a shorter vector will cause most other Array2d methiods to
/// panic. Replacing the data with a longer vector leads to non-accessible
/// data.
/// However, this implementation can be quite useful, e.g., for swapping
/// rows or columns.
impl<T> DerefMut for Array2d<T>
{
    fn deref_mut(&mut self) -> &mut Vec<T>
    {
        &mut self.data
    }
}

impl<'a, T: fmt::Debug + Clone + Copy> fmt::Display for Array2d<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut s = String::new();
        for row in self.iter() {
            let row_string = row
                .iter()
                .map(|r| format!("{:?}", r))
                .collect::<Vec<String>>()
                .join("\t");
            s.push_str(&row_string);
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

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
use std::fs;
use std::path::PathBuf;

use crate::array2d::Array2d;

pub fn savetxt<T>(filename: PathBuf, arr: &Array2d<T>) -> std::io::Result<()>
where
    T: Clone + Debug + std::marker::Copy,
{
    let data = format!("{}", arr);
    fs::write(filename, data)?;
    Ok(())
}

/// Load matrix from file and store elements as f64
pub fn loadtxt(filename: PathBuf) -> std::io::Result<Array2d<f64>> {
    let string = fs::read_to_string(filename).unwrap();
    Ok(Array2d::<f64>::from_string(string))
}

/// Load matrix from file and store elements as usize
pub fn loadtxt_isize(filename: PathBuf) -> std::io::Result<Array2d<isize>>
{
    let string = fs::read_to_string(filename).unwrap();
    Ok(Array2d::<isize>::from_string(string))
}

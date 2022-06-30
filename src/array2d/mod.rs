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

pub mod indexing;
pub mod numerics;
pub mod swapping;

mod iterator;
mod operators;
mod traits;

use core::fmt::Debug;
use std::clone::Clone;
use std::string::String;

#[derive(PartialEq, Debug)]
pub struct Array2d<T>
{
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<'a, T: Debug + Clone + Copy> Array2d<T>
{
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Array2d<T>
    {
        if data.len() != rows * cols {
            panic!("Error: length of vector does not  match dimensions");
        }

        Array2d { data, rows, cols }
    }

    pub fn new_fill(value: T, rows: usize, cols: usize) -> Array2d<T>
    {
        let n_el = rows * cols;

        Array2d {
            data: vec![value; n_el],
            rows,
            cols,
        }
    }

    pub fn new_from_array(arr: &Array2d<T>) -> Array2d<T>
    {
        let mut data: Vec<T> = Vec::with_capacity(arr.rows * arr.cols);
        for el in &arr.data {
            data.push(*el);
        }
        Array2d {
            data,
            rows: arr.rows,
            cols: arr.cols,
        }
    }

    fn new_empty(rows: usize, cols: usize) -> Array2d<T>
    {
        let n_el = rows * cols;
        Array2d {
            data: Vec::<T>::with_capacity(n_el),
            rows,
            cols,
        }
    }

    pub fn transpose(&self) -> Array2d<T>
    {
        let mut temp = Self::new_from_array(self);
        temp.transpose_inplace();
        temp
    }

    pub fn transpose_inplace(&mut self)
    {
        let mut row: usize = 0;
        for i_el in 0..(self.data.len() - 1) {
            if i_el >= self.cols * (row + 1) {
                row += 1;
            }
            if i_el < row * (self.cols + 1) {
                continue;
            }
            let col = i_el - self.cols * row;
            let j_el = self.cols * col + row;
            self.data.swap(i_el, j_el);
        }
    }

    pub fn row(&self, row: usize) -> Vec<T>
    {
        let start = row * self.cols;
        let end = (row + 1) * self.cols;
        self.data[start..end].to_vec()
    }

    pub fn set_row(&mut self, row: usize, data: Vec<T>)
    {
        let start = row * self.cols;
        let end = (row + 1) * self.cols;
        for (i, j) in (start..end).enumerate() {
            self.data[j] = data[i];
        }
    }

    pub fn col(&self, col: usize) -> Vec<T>
    {
        let mut col_data = Vec::<T>::with_capacity(self.rows);
        for i in 0..self.rows {
            col_data.push(self.data[col + i * self.cols]);
        }
        col_data
    }

    pub fn print(&self)
    {
        println!("{}x{} Array", self.rows, self.cols);
        for i in 0..self.rows {
            let start = i * self.cols;
            let end = (i + 1) * self.cols;
            println!("{:?}", &self.data[start..end]);
        }
    }

    pub fn shape(&self) -> (usize, usize)
    {
        (self.rows, self.cols)
    }

    pub fn get_data(&self) -> &Vec<T>
    {
        &self.data
    }

    /// Return array of shape (self.rows-1, self.cols-1) conatining
    /// all elements not in row or col.
    pub fn get_subarray(&self, row: usize, col: usize) -> Array2d<T>
    {
        let tmp = self
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != row)
            .flat_map(|(_, v)| v)
            .collect::<Vec<T>>()
            .to_vec();
        let tmp = Self::new(tmp, self.rows - 1, self.cols).transpose();
        let tmp = tmp
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != col)
            .flat_map(|(_, v)| v)
            .collect::<Vec<T>>()
            .to_vec();
        Self::new(tmp, self.rows - 1, self.cols - 1).transpose()
    }

    pub fn flatten(&self) -> Array2d<T>
    {
        let mut flat = Self::new_empty(1, self.rows * self.cols);
        for el in &self.data {
            flat.data.push(*el);
        }
        flat
    }
}

impl<'a, T: Debug + Clone + Copy + std::str::FromStr> Array2d<T>
{
    pub fn from_string(string: String) -> Array2d<T>
    where
        <T as std::str::FromStr>::Err: Debug,
    {
        let mut data: Vec<T> = Vec::new();
        let rows = string
            .split('\n')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();
        let n_rows: usize = rows.len();
        let mut n_elem: usize = 0;
        for row in rows {
            row.split_whitespace().for_each(|r| {
                data.push(r.parse::<T>().unwrap());
                n_elem += 1;
            })
        }
        let n_cols = n_elem / n_rows;
        Array2d {
            data,
            rows: n_rows,
            cols: n_cols,
        }
    }
}

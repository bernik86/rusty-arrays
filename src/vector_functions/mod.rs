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

pub fn add_vec_inplace<T: Clone + Copy + std::ops::AddAssign>(v1: &mut [T], v2: &[T])
{
    v1.iter_mut().zip(v2.iter()).for_each(|(e1, e2)| *e1 += *e2);
}

pub fn sub_vec_inplace<T: Clone + Copy + std::ops::SubAssign>(v1: &mut [T], v2: &[T])
{
    v1.iter_mut().zip(v2.iter()).for_each(|(e1, e2)| *e1 -= *e2);
}

pub fn scalar_div_inplace(vec: &mut [f64], div: f64)
{
    vec.iter_mut().for_each(|el| *el /= div);
}

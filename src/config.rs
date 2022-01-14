/* xmlparse - An extensible xml processing tool that converts xml data to
 * a line oriented format similar to that of xpath.
 * Copyright (C) 2021 Saleh Bakra'a
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub struct ProgramOpts {
	pub map_whitespace: bool,
	pub tab_map: char,
	pub space_map: char,
	pub newline_map: char,

	pub compress_whitespace: bool,
	pub compress_level: usize,
}

impl Default for ProgramOpts {
	fn default() -> Self {
		ProgramOpts {
			map_whitespace: false,
			tab_map: '»',
			space_map: '·',
			newline_map: '↵',

			compress_whitespace: false,
			compress_level: 4,
		}
	}
}

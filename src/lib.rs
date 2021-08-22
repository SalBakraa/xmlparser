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

mod sax;

mod parser_data;

use sax::default_sax_handler;
use sax::init_sax_handler;
use sax::sax_user_parse_file;

use parser_data::ParserData;

use std::ffi::CString;
use std::iter::FromIterator;

use cty::*;

use phf::{ phf_map, Map };

use once_cell::sync::OnceCell;

pub static DO_MAP_WHITESPACE: OnceCell<bool> = OnceCell::new();
pub static DO_COMPRESS_WHITESPACE: OnceCell<bool> = OnceCell::new();

pub static COMPRESSION_LEVEL: OnceCell<usize> = OnceCell::new();

static WHITESPACE_MAP: Map<char, char> = phf_map! {
    ' ' => '␣',
    '\t' => '→',
    '\n' => '↵',
};

static COMPRESSED_WHITESPACE: char = '·';

pub fn print_whitespace_mappings() {
    let map = String::from_iter(WHITESPACE_MAP.values());
    println!("{}|{}", map, COMPRESSED_WHITESPACE);
}

pub fn print_nodes(file: String) {
    let file = CString::new(file).unwrap();

    let mut handler = default_sax_handler();
    init_sax_handler(&mut handler);

    let mut data = ParserData::new();
    let data_ptr = &mut data as *mut _ as *mut c_void;
    sax_user_parse_file(&mut handler, data_ptr, file);
}

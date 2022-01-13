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

mod defaults {
	pub static TAB_MAP: char = '→';
	pub static SPACE_MAP: char = '␣';
	pub static NEWLINE_MAP: char = '↵';

	pub static COMPRESS_LEVEL: fn() -> usize = || { 4 };
	pub static MAP_WHITESPACE: fn() -> bool = || { true };
	pub static COMPRESS_WHITESPACE: fn() -> bool = || { true };
}

mod bindings {
	// Since libxml2 does not follow rust's coding conventions
	#![allow(non_upper_case_globals)]
	#![allow(non_camel_case_types)]
	#![allow(non_snake_case)]

	// The bindings generated contain references to types without stable ABIs
	#![allow(improper_ctypes)]

	// Most functions generated are not used
	#![allow(dead_code)]

	include!(concat!(env!("OUT_DIR"), "/parser.rs"));
	include!(concat!(env!("OUT_DIR"), "/sax_funcs.rs"));
}

mod ptr_conversions {
	use crate::bindings::xmlChar;

	use std::ffi::CStr;

	pub fn str_from_xmlchar_with_null<'a>(chars: *const xmlChar) -> &'a str {
		unsafe {
			let chars = CStr::from_ptr(chars as *const i8).to_bytes();
			std::str::from_utf8_unchecked(chars)
		}
	}

	pub fn str_from_xmlchar<'a>(chars: *const xmlChar, len: isize) -> &'a str {
		unsafe {
			let chars = std::slice::from_raw_parts(chars, len as usize);
			std::str::from_utf8_unchecked(chars)
		}
	}

	pub fn vec_from_ptr_with_null(ptr: *mut *const xmlChar) -> Vec<*const xmlChar> {
		if ptr.is_null() {
			return Vec::new();
		}

		let len = unsafe {
			let mut i = 0;
			while !(*ptr.add(i)).is_null() { i += 1; }
			i
		};

		let mut container = Vec::with_capacity(len);
		unsafe {
			std::ptr::copy(ptr, container.as_mut_ptr(), len);
			container.set_len(len);
		}
		container
	}

	// TODO: Figuere how to efficiently implement this.
	// fn _translate_whitespace(c: char) -> char {
	//     if !DO_MAP_WHITESPACE.get_or_init(|| true) {
	//         return c;
	//     }

	//     *WHITESPACE_MAP.get(&c).unwrap_or(&c)
	// }

	// TODO: Figuere how to efficiently implement this.
	// fn _compress_whitespace(string: String) -> String {
	//     if !DO_COMPRESS_WHITESPACE.get_or_init(|| true) {
	//         return string;
	//     }

	//     let compressed_string = "␣".repeat(*COMPRESSION_LEVEL.get_or_init(|| 4));
	//     string.replace(&compressed_string, &COMPRESSED_WHITESPACE.to_string())
	// }
}

mod sax;

mod parser_data;

use parser_data::ParserData;

use std::ffi::CString;

use cty::c_void;

use once_cell::sync::OnceCell;

pub static MAP_WHITESPACE: OnceCell<bool> = OnceCell::new();
pub static COMPRESS_WHITESPACE: OnceCell<bool> = OnceCell::new();
pub static COMPRESSION_LEVEL: OnceCell<usize> = OnceCell::new();

pub fn print_nodes(file: String) {
	let file = CString::new(file).unwrap();

	let mut handler = sax::default_sax_handler();
	sax::init_sax_handler(&mut handler);

	let mut data = ParserData::with_capacity(10);
	let data_ptr = &mut data as *mut _ as *mut c_void;
	sax::sax_user_parse_file(&mut handler, data_ptr, file);
}

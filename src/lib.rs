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

	pub fn str_from_xmlchar_with_null<'a>(chars: *const xmlChar) -> &'a str {
		unsafe {
			let chars = std::ffi::CStr::from_ptr(chars as *const i8).to_bytes();
			std::str::from_utf8_unchecked(chars)
		}
	}

	pub fn str_from_xmlchar<'a>(chars: *const xmlChar, len: isize) -> &'a str {
		unsafe {
			let chars = std::slice::from_raw_parts(chars, len as usize);
			std::str::from_utf8_unchecked(chars)
		}
	}

	pub fn slice_from_ptr_with_null<'a>(ptr: *mut *const xmlChar) -> &'a [*const xmlChar] {
		unsafe {
			let mut len = 0;
			while !(*ptr.add(len)).is_null() { len += 1; }

			std::slice::from_raw_parts(ptr, len)
		}
	}
}

mod sax;

mod parser_data;

mod config;

pub use config::ProgramOpts;

pub fn print_nodes(file: String, opts: &ProgramOpts) {
	let file = std::ffi::CString::new(file).unwrap();

	let mut handler = sax::default_sax_handler();
	sax::init_sax_handler(&mut handler);

	let mut data = parser_data::ParserData::with_capacity(10, opts);
	let data_ptr = &mut data as *mut _ as *mut cty::c_void;
	sax::sax_user_parse_file(&mut handler, data_ptr, file);
}

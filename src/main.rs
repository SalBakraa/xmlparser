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

mod cli;

use cli::build_cli;

fn main() {
	let exit_code = real_main();
	std::process::exit(exit_code);
}

fn real_main() -> i32 {
	let app = build_cli();
	let matches = app.get_matches();

	if matches.is_present("print whitespace map") {
		xmlparse::print_whitespace_mappings();
		return 0;
	}

	if matches.is_present("No whitespace mapping") {
		let _ = xmlparse::MAP_WHITESPACE.set(false);

		// FIXME
		//let _ = xmlparse::DO_COMPRESS_WHITESPACE.set(false);
	}

	if matches.is_present("No whitespace compressing") {
		eprintln!("DEPRECATED FLAG! whitespace is not compressed in normal behaviour.");
		return 1;

		// FIXME
		//let _ = xmlparse::DO_COMPRESS_WHITESPACE.set(false);
	}

	if let Some(_level) = matches.value_of("Compression level") {
		eprintln!("DEPRECATED FLAG! whitespace is not compressed in normal behaviour.");
		return 1;

		// FIXME
		//let level: usize = level.parse().unwrap_or(4);
		//let _ = xmlparse::COMPRESSION_LEVEL.set(level);
	}

	for file in matches.values_of("FILES").unwrap().collect::<Vec<_>>() {
		xmlparse::print_nodes(file.to_owned());
	}

	0
}

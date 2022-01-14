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

fn main() {
	let exit_code = real_main();
	std::process::exit(exit_code);
}

fn real_main() -> i32 {
	let mut opts = xmlparse::ProgramOpts::default();

	let app = cli::build_cli();
	let matches = app.get_matches();

	if matches.is_present("Print Mappings") {
		println!("{}{}{}", opts.space_map, opts.tab_map, opts.newline_map);
		return 0;
	}

	if matches.is_present("Map Whitespace") {
		opts.map_whitespace = true;
	}

	if matches.is_present("Compress Whitespace") {
		opts.compress_whitespace = true;
	}

	if let Some(string) = matches.value_of("Space Character") {
		opts.space_map = string.chars().nth(0).unwrap();
	}

	if let Some(string) = matches.value_of("Tab Character") {
		opts.tab_map = string.chars().nth(0).unwrap();
	}

	if let Some(string) = matches.value_of("Newline Character") {
		opts.newline_map = string.chars().nth(0).unwrap();
	}

	if let Some(string) = matches.value_of("Whitespace Mapping") {
		opts.space_map = string.chars().nth(0).unwrap();
		opts.tab_map = string.chars().nth(1).unwrap();
		opts.newline_map = string.chars().nth(2).unwrap();
	}

	if let Some(level) = matches.value_of("Compression Level") {
		let level: usize = level.parse().unwrap();
		opts.compress_level = level;
	}

	for file in matches.values_of("FILES").unwrap().collect::<Vec<_>>() {
		xmlparse::print_nodes(file.to_owned(), &opts);
	}

	0
}

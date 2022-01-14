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

use clap::{ App, Arg };
use clap::{ crate_name, crate_version, crate_authors, crate_description };

// Mark the function public so that it can be used by build.rs to generate the
// shell completions
pub fn build_cli() -> App<'static, 'static> {
	App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.arg(
			Arg::with_name("Print Mappings")
				.short("p")
				.long("print-mappings")
				.help("Print the characters used to visualize whitespace characters \
					  in the following order <SPACE><TAB><LF> and exits.")
				.display_order(1)
		)
		.arg(
			Arg::with_name("Map Whitespace")
				.short("m")
				.long("map-whitespace")
				.help("Transliterates whitespace characters to printable characters.")
				.display_order(2)
		)
		.arg(
			Arg::with_name("Compress Whitespace")
				.short("c")
				.long("compress-whitespace")
				.help("Compresses consecutive `space` characters to a `tab` character\
					  according to the compression level.")
				.display_order(3)
		)
		.arg(
			Arg::with_name("Compression Level")
				.short("l")
				.long("compress-level")
				.help("Specifies the number consecutive spaces compressed to a \
					  single character. Default: 4 spaces")
				.takes_value(true)
				.value_name("LEVEL")
				.allow_hyphen_values(true)
				.display_order(4)
		)
		.arg(
			Arg::with_name("FILES")
				.required_unless("Print Mappings")
				.help("XML files to read")
				.multiple(true)
				.display_order(5)
		)
		.after_help(
			"EXAMPLES: \n\
			\tIf you want to keep visual whitespace while text processing; You can use sed to \n\
			\tremove the visualizations as the last step of text processing. \n\
			\n\
			\t$ MAPS=\"$(xmlparse  --print-mappings)\" \n\
			\t$ xmlparse -m foo.xml | <Your text processing here> | sed \"y/$MAPS/ \\t\\n/\""
		)
}

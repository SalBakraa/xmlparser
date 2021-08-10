/* xmlparser - An xml parser meant to be used extensibly in shell scripts
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

use clap::{App, Arg};
use clap::{ crate_name, crate_version, crate_authors, crate_description };

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}

fn real_main() -> i32 {
    let app = build_cli();
    let matches = app.get_matches();

    if matches.is_present("print whitespace map") {
        xmlparser::print_whitespace_mappings();
        return 0;
    }

    if matches.is_present("No whitespace mapping") {
        let _ = xmlparser::DO_MAP_WHITESPACE.set(false);
        let _ = xmlparser::DO_COMPRESS_WHITESPACE.set(false);
    }

    if matches.is_present("No whitespace compressing") {
        let _ = xmlparser::DO_COMPRESS_WHITESPACE.set(false);
    }

    for file in matches.values_of("FILES").unwrap().collect::<Vec<_>>() {
        xmlparser::print_nodes(file.to_owned());
    }

    println!("Hello, world!");
    0
}

fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("print whitespace map")
                .short("p")
                .long("whitespace-maps")
                .help("Print the characters used to visualize/compress whitespace \
                      charcters in the following order <SPACE><\\t><\\n>|<COMPRESSOR> and exits")
                .display_order(1)
        )
        .arg(
            Arg::with_name("No whitespace mapping")
                .short("k")
                .long("keep-whitespace")
                .help("Prevents whitespace from being mapped to display charcters. Assumes --no-compress")
                .display_order(2)
        )
        .arg(
            Arg::with_name("No whitespace compressing")
                .long("no-compress")
                .help("Prevents whitespace from being compressed")
                .display_order(3)
        )
        .arg(
            Arg::with_name("FILES")
                .required_unless("print whitespace map")
                .help("XML files to read")
                .index(1)
                .multiple(true)
        )
        .after_help(
            "EXAMPLES \n\
             \tIf you want to keep visual whitespace while text processing; You can use sed to \n\
             \tremove the visualizations as the last step of text processing. \n\
             \n\
             \t$ MAPS=\"$(xmlparser --whitespace-maps)\" \n\
             \t$ xmlparser foo.xml | <Your text processing here> | sed -e \
             'y/${MAPS%|*}/ \\t\\n' -e 's/${MAPS#*|}/    /g'"
            )
}

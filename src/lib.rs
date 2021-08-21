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

use sax::default_sax_handler;
use sax::init_sax_handler;
use sax::sax_user_parse_file;

use std::ffi::CString;
use std::iter::FromIterator;
use std::io::{ stdout as rust_stdout, Stdout, BufWriter };

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

struct ParserData {
    result: u32,
    path: PathNode,
    // can't be named stdout since parser.rs already defined it
    stdoutput: BufWriter<Stdout>,
}

struct PathNode {
    child: NodeLink,
    name: String,
    printed: bool,
}

type NodeLink = Option<Box<PathNode>>;

impl ParserData {
    fn new() -> Self {
        ParserData {
            result: 0,
            path: PathNode::new(),
            stdoutput: BufWriter::new(rust_stdout())
        }
    }

    fn last_path_node(&self) -> &PathNode {
        let mut temp = &self.path;
        while temp.child.is_some() { temp = temp.child(); }
        temp
    }

    fn last_path_node_mut(&mut self) -> &mut PathNode {
        let mut temp = &mut self.path;
        while temp.child.is_some() { temp = temp.child_mut(); }
        temp
    }

    fn set_last_path_node(&mut self, node: PathNode) {
        self.last_path_node_mut().set_child(node);
    }

    fn remove_last_path_node(&mut self) -> PathNode {
        let mut parent = &mut self.path;
        while parent.child.is_some() {
            if !parent.child().child.is_some() { break; }
            parent = parent.child_mut();
        }

        let mut removedNode: NodeLink = None;
        std::mem::swap(&mut parent.child, &mut removedNode);

        return *removedNode.unwrap();
    }
}

impl PathNode {
    fn new() -> PathNode {
        PathNode {
            child: None,
            name: String::new(),
            printed: false
        }
    }

    fn from(child: Option<Box<PathNode>>, name: String, printed: bool) -> PathNode {
        PathNode { child, name, printed }
    }

    fn child(&self) -> &PathNode {
        &*(self.child.as_ref().unwrap())
    }

    fn child_mut(&mut self) -> &mut PathNode {
        &mut *(self.child.as_mut().unwrap())
    }

    fn set_printed(&mut self, val: bool) {
        self.printed = val;
    }

    fn set_child(&mut self, val: PathNode) {
        self.child = Some(Box::new(val));
    }
}

impl std::fmt::Display for PathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.child.is_some() {
            write!(f, "{}/{}", self.name, *(self.child.as_ref().unwrap()))
        } else {
            write!(f, "{}", self.name)
        }
    }
}

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

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

mod path_node;

pub use path_node::PathNode;
pub use path_node::NodeLink;

use std::io::{ stdout, Stdout, BufWriter };

pub struct ParserData {
    result: u32,
    path: PathNode,
    stdout: BufWriter<Stdout>,
}

impl ParserData {
    pub fn new() -> Self {
        ParserData {
            result: 0,
            path: PathNode::new(),
            stdout: BufWriter::new(stdout())
        }
    }

    pub fn path_and_buf_mut(&mut self) -> (&mut PathNode, &mut BufWriter<Stdout>) {
        (&mut self.path, &mut self.stdout)
    }

    pub fn last_path_node(&self) -> &PathNode {
        let mut temp = &self.path;
        while temp.has_child() { temp = temp.child(); }
        temp
    }

    pub fn last_path_node_mut(&mut self) -> &mut PathNode {
        let mut temp = &mut self.path;
        while temp.has_child() { temp = temp.child_mut(); }
        temp
    }

    pub fn set_last_path_node(&mut self, node: PathNode) {
        self.last_path_node_mut().set_child(node);
    }

    pub fn remove_last_path_node(&mut self) -> PathNode {
        let mut parent = &mut self.path;
        while parent.has_child() {
            if !parent.child().has_child() { break; }
            parent = parent.child_mut();
        }

        return *parent.take_child().unwrap();
    }
}

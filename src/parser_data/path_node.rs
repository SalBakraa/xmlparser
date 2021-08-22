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

pub struct PathNode {
    child: NodeLink,
    name: String,
    printed: bool,
}

pub type NodeLink = Option<Box<PathNode>>;

impl PathNode {
    pub fn new() -> PathNode {
        PathNode {
            child: None,
            name: String::new(),
            printed: false
        }
    }

    pub fn from(child: NodeLink, name: String, printed: bool) -> PathNode {
        PathNode { child, name, printed }
    }

    pub fn child(&self) -> &PathNode {
        &*(self.child.as_ref().unwrap())
    }

    pub fn child_mut(&mut self) -> &mut PathNode {
        &mut *(self.child.as_mut().unwrap())
    }

    pub fn set_child(&mut self, val: PathNode) {
        self.child = Some(Box::new(val));
    }

    pub fn has_child(&self) -> bool {
        self.child.is_some()
    }

    pub fn take_child(&mut self) -> NodeLink {
        self.child.take()
    }

    pub fn printed(&self) -> bool {
        self.printed
    }

    pub fn set_printed(&mut self, val: bool) {
        self.printed = val;
    }

    pub fn name(&self) -> &String {
        &self.name
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

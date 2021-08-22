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

#[derive(Default)]
pub struct XmlTag {
    name: String,
    printed: bool,
}

impl XmlTag {
    pub fn from(name: String, printed: bool) -> Self {
        XmlTag { name, printed }
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

impl std::fmt::Display for XmlTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

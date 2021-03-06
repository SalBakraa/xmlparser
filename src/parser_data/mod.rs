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

mod xml_tag;

pub use xml_tag::XmlTag;

use crate::config::ProgramOpts;

use std::io::Write;
use std::io::{ stdout, Stdout, BufWriter };

pub struct XmlTags<'a>(Vec<XmlTag<'a>>);

pub struct ParserData<'a> {
	result: u32,
	opts: &'a ProgramOpts,
	tags: XmlTags<'a>,
	stdout: BufWriter<Stdout>,
}

impl<'a> ParserData<'a> {
	pub fn with_capacity(cap: usize, opts: &'a ProgramOpts) -> Self {
		ParserData {
			result: 0,
			opts,
			tags: XmlTags(Vec::with_capacity(cap)),
			stdout: BufWriter::new(stdout()),
		}
	}

	pub fn opts_tags_and_buf_mut(&mut self) -> (&ProgramOpts, &mut XmlTags<'a>, &mut BufWriter<Stdout>) {
		(self.opts, &mut self.tags, &mut self.stdout)
	}

	pub fn opts(&mut self) -> &ProgramOpts {
		self.opts
	}

	pub fn last_tag(&self) -> Option<&XmlTag<'a>> {
		self.tags.0.last()
	}

	pub fn last_tag_mut(&mut self) -> Option<&mut XmlTag<'a>> {
		self.tags.0.last_mut()
	}

	pub fn push_tag(&mut self, node: XmlTag<'a>) {
		self.tags.0.push(node)
	}

	pub fn pop_tag(&mut self) -> Option<XmlTag<'a>> {
		self.tags.0.pop()
	}

	pub fn tags_is_empty(&self) -> bool {
		self.tags.0.is_empty()
	}

	pub fn print_last_tag(&mut self) {
		if self.tags_is_empty() {
			return;
		}

		let tag = self.last_tag().unwrap();
		if !tag.printed() {
			writeln!(self.stdout, "{}", self.tags).unwrap();
			self.last_tag_mut().unwrap().set_printed(true);
		}
	}
}

impl<'a> std::fmt::Display for XmlTags<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.iter().try_for_each(|t| write!(f, "/{}", t))
	}
}

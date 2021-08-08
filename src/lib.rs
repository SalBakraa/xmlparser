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

// Since libxml2 does not follow rust's coding conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// The bindings generated by bindgen contain references to types without stable ABIs
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::ptr::{ null, null_mut };

use cty::*;

use phf::{ phf_map, Map };

static WHITESPACE_MAP: Map<char, char> = phf_map! {
    ' ' => '␣',
    '\t' => '→',
    '\n' => '↵',
};

struct ParserData {
    result: u32,
    path: PathNode
}

struct PathNode {
    child: NodeLink,
    name: String,
    printed: bool,
}

type NodeLink = Option<Box<PathNode>>;

impl ParserData {
    fn new() -> ParserData {
        ParserData {
            result: 0,
            path: PathNode::new()
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

pub fn print_nodes(file: &String) {
    let file = CString::new(file.clone()).unwrap();

    let mut handler = default_sax_handler();
    init_sax_handler(&mut handler);

    let mut data = ParserData::new();
    let data_ptr = &mut data as *mut _ as *mut c_void;
    unsafe { xmlSAXUserParseFile(&mut handler, data_ptr, file.into_raw()); }
}

fn default_sax_handler() -> xmlSAXHandler {
    xmlSAXHandler {
        internalSubset: None,
        isStandalone: None,
        hasInternalSubset: None,
        hasExternalSubset: None,
        resolveEntity: None,
        getEntity: None,
        entityDecl: None,
        notationDecl: None,
        attributeDecl: None,
        elementDecl: None,
        unparsedEntityDecl: None,
        setDocumentLocator: None,
        startDocument: None,
        endDocument: None,
        startElement: None,
        endElement: None,
        reference: None,
        characters: None,
        ignorableWhitespace: None,
        processingInstruction: None,
        comment: None,
        warning: None,
        error: None,
        fatalError: None,
        getParameterEntity: None,
        cdataBlock: None,
        externalSubset: None,
        initialized: 0,
        _private: null_mut::<c_void>(),
        startElementNs: None,
        endElementNs: None,
        serror: None,
    }
}

fn init_sax_handler(sax: xmlSAXHandlerPtr) {
    unsafe {
        (*sax).startDocument = Some(sax_start_document);
        (*sax).startElement = Some(sax_start_element);
        (*sax).endElement = Some(sax_end_element);
        (*sax).characters = Some(sax_characters);
        (*sax).initialized = 1;
    }
}

extern fn sax_start_document(_user_data_ptr: *mut c_void) {
    println!("Started parsing :]");
}

fn deref_mut_void_ptr<'a, T>(ptr: *mut c_void) -> &'a mut T {
    let ptr = ptr as *mut T;
    unsafe { &mut *ptr }
}

extern fn sax_start_element(user_data_ptr: *mut c_void, name: *const xmlChar, attrs: *mut *const xmlChar) {
    let user_data = deref_mut_void_ptr::<ParserData>(user_data_ptr);

    let parent = (*user_data).last_path_node();
    if !parent.printed {
        println!("{}", (*user_data).path);
        (*user_data).last_path_node_mut().set_printed(true);
    }

    let name = string_from_xmlchar_with_null(name);
    (*user_data).set_last_path_node(PathNode::from(None, name, false));

    let attrs = vec_from_ptr_with_null(attrs);
    if attrs.is_empty() {
        return;
    }

    let attrs: Vec<String> = attrs.iter().map(|e| string_from_xmlchar_with_null(*e)).collect();
    let attrs: Vec<String> = attrs.chunks(2).map(|c| format!("{}=\"{}\"", c[0], c[1])).collect();
    let attrs = attrs.join(",");

    println!("{}@[{}]", (*user_data).path, attrs);
    (*user_data).last_path_node_mut().set_printed(true);
}

extern fn sax_end_element(user_data_ptr: *mut c_void, name: *const xmlChar) {
    let user_data = deref_mut_void_ptr::<ParserData>(user_data_ptr);
    let name = string_from_xmlchar_with_null(name);

    let last = (*user_data).last_path_node();
    if last.name != name {
        return
    }

    if !last.printed {
        println!("{}", (*user_data).path);
    }

    (*user_data).remove_last_path_node();
}

extern fn sax_characters(user_data_ptr: *mut c_void, chars: *const xmlChar, len: i32) {
    let user_data = deref_mut_void_ptr::<ParserData>(user_data_ptr);
    let chars = string_from_xmlchar(chars, len as isize);
    if chars.trim().is_empty() {
        return;
    }

    let chars = translate_whitespace(chars).replace("␣␣␣␣","·");
    println!("{}=\"{}\"", (*user_data).path, chars);

    (*user_data).last_path_node_mut().set_printed(true);
}

fn translate_whitespace(string: String) -> String {
    let mut container = String::with_capacity(string.len());
    for c in string.chars() {
        match WHITESPACE_MAP.get(&c) {
            Some(rep) => container.push(*rep),
            None => container.push(c),
        }
    }
    container
}

fn string_from_xmlchar(chars: *const xmlChar, len: isize) -> String {
    if len < 0 {
        panic!("Length must be positive.")
    }

    let len = len as usize;
    let mut container = vec![b'\0'; len];
    for i in 0..len {
        container[i] = unsafe { *(chars.add(i)) };
    }

    String::from_utf8(container).unwrap()
}

fn string_from_xmlchar_with_null(chars: *const xmlChar) -> String {
    let mut container = Vec::new();
    if chars.is_null() {
        return String::from_utf8(container).unwrap();
    }

    unsafe {
        let mut i = 0;
        while *(chars.offset(i)) != b'\0' {
            container.push(*(chars.offset(i)));
            i += 1;
        }
    }

    String::from_utf8(container).unwrap()
}

fn vec_from_ptr_with_null(ptr: *mut *const xmlChar) -> Vec<*const xmlChar> {
    let mut container = Vec::new();
    if ptr.is_null() {
        return container;
    }

    unsafe {
        let mut i = 0;
        while !(*ptr.add(i)).is_null() {
            container.push(*ptr.add(i));
            i += 1;
        }
        container
    }
}

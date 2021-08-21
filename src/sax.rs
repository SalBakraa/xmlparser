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

mod bindings{
    // Since libxml2 does not follow rust's coding conventions
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    // The bindings generated contain references to types without stable ABIs
    #![allow(improper_ctypes)]

    // Most functions generated are not used
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
    include!(concat!(env!("OUT_DIR"), "/sax_funcs.rs"));
}

use bindings::xmlChar;
use bindings::xmlSAXHandler;
use bindings::xmlSAXHandlerPtr;
use bindings::xmlSAXUserParseFile;
use bindings::sax_warning;
use bindings::sax_error;
use bindings::sax_fatal_error;

use super::WHITESPACE_MAP;
use super::DO_MAP_WHITESPACE;
use super::COMPRESSED_WHITESPACE;
use super::DO_COMPRESS_WHITESPACE;
use super::COMPRESSION_LEVEL;
use super::ParserData;
use super::PathNode;

use std::ffi::CString;
use std::ptr::null_mut;
use std::io::Write;

use cty::c_void;

pub fn default_sax_handler() -> xmlSAXHandler {
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

pub fn init_sax_handler(sax: xmlSAXHandlerPtr) {
    unsafe {
        (*sax).startElement = Some(sax_start_element);
        (*sax).endElement = Some(sax_end_element);
        (*sax).characters = Some(sax_characters);
        (*sax).processingInstruction = Some(sax_processing_instruction);
        (*sax).comment = Some(sax_comment);
        (*sax).warning = Some(sax_warning);
        (*sax).error = Some(sax_error);
        (*sax).fatalError = Some(sax_fatal_error);
        (*sax).initialized = 1;
    }
}

pub fn sax_user_parse_file(sax: xmlSAXHandlerPtr, data_ptr: *mut c_void, file: CString) {
    unsafe { xmlSAXUserParseFile(sax, data_ptr, file.into_raw()); }
}

fn deref_mut_void_ptr<'a, T>(ptr: *mut c_void) -> &'a mut T {
    let ptr = ptr as *mut T;
    unsafe { &mut *ptr }
}

extern fn sax_start_element(user_data_ptr: *mut c_void, name: *const xmlChar, attrs: *mut *const xmlChar) {
    let user_data = deref_mut_void_ptr::<ParserData>(user_data_ptr);

    let parent = (*user_data).last_path_node();
    if !parent.printed {
        writeln!((*user_data).stdoutput, "{}", (*user_data).path).unwrap();
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

    writeln!((*user_data).stdoutput, "{}@[{}]", (*user_data).path, attrs).unwrap();
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
        writeln!((*user_data).stdoutput, "{}", (*user_data).path).unwrap();
    }

    (*user_data).remove_last_path_node();
}

extern fn sax_characters(user_data_ptr: *mut c_void, chars: *const xmlChar, len: i32) {
    let user_data = deref_mut_void_ptr::<ParserData>(user_data_ptr);
    let chars = string_from_xmlchar(chars, len as isize);
    if is_only_whitespace(&chars) {
        return;
    }

    writeln!((*user_data).stdoutput, "{}=\"{}\"", (*user_data).path, chars).unwrap();

    (*user_data).last_path_node_mut().set_printed(true);
}

extern fn sax_processing_instruction(user_data_ptr: *mut c_void, target: *const xmlChar, data: *const xmlChar) {
    let user_data = deref_mut_void_ptr::<ParserData>(user_data_ptr);
    let target = string_from_xmlchar_with_null(target);
    let data = string_from_xmlchar_with_null(data);

    writeln!((*user_data).stdoutput, "{}/{}?[{}]", (*user_data).path, target, data).unwrap();
}

extern fn sax_comment(user_data_ptr: *mut c_void, comment: *const xmlChar) {
    let user_data = deref_mut_void_ptr::<ParserData>(user_data_ptr);
    let comment = string_from_xmlchar_with_null(comment);

    writeln!((*user_data).stdoutput, "{}/![{}]", (*user_data).path, comment).unwrap();
}

fn string_from_xmlchar(chars: *const xmlChar, len: isize) -> String {
    if len < 0 {
        panic!("Length must be positive.")
    }

    let len = len as usize;
    let mut container = String::with_capacity(len);
    unsafe {
        std::ptr::copy(chars, container.as_mut_vec().as_mut_ptr(), len);
        container.as_mut_vec().set_len(len);
    }

    compress_whitespace(container)
}

fn string_from_xmlchar_with_null(chars: *const xmlChar) -> String {
    if chars.is_null() {
        return String::new();
    }

    let len = unsafe {
        let mut i = 0;
        while *(chars.offset(i)) != b'\0' { i += 1; }
        i
    };

    string_from_xmlchar(chars, len)
}

fn translate_whitespace(c: char) -> char {
    if !DO_MAP_WHITESPACE.get_or_init(|| true) {
        return c;
    }
    *WHITESPACE_MAP.get(&c).unwrap_or(&c)
}

fn compress_whitespace(string: String) -> String {
    if !DO_COMPRESS_WHITESPACE.get_or_init(|| true) {
        return string;
    }

    let compressed_string = "␣".repeat(*COMPRESSION_LEVEL.get_or_init(|| 4));
    string.replace(&compressed_string, &COMPRESSED_WHITESPACE.to_string())
}

fn is_only_whitespace(string: &String) -> bool {
    if string.trim().is_empty() {
        return true
    }

    for c in string.chars() {
        if !WHITESPACE_MAP.values().any(|&val| val == c) && c != COMPRESSED_WHITESPACE {
            return false;
        }
    }

    true
}

fn vec_from_ptr_with_null(ptr: *mut *const xmlChar) -> Vec<*const xmlChar> {
    if ptr.is_null() {
        return Vec::new();
    }

    let len = unsafe {
        let mut i = 0;
        while !(*ptr.add(i)).is_null() { i += 1; }
        i
    };

    let mut container = Vec::with_capacity(len);
    unsafe {
        std::ptr::copy(ptr, container.as_mut_ptr(), len);
        container.set_len(len);
    }
    container
}

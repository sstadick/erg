//! provides utilities for parser, compiler, and vm crate.
use std::fmt;

pub mod cache;
pub mod codeobj;
pub mod color;
pub mod combinations;
pub mod config;
pub mod datetime;
pub mod deserialize;
pub mod dict;
pub mod error;
pub mod fxhash;
pub mod lazy;
pub mod lazy_buffer;
pub mod levenshtein;
pub mod macros;
pub mod opcode;
pub mod python_util;
pub mod rccell;
pub mod serialize;
pub mod set;
pub mod stdin;
pub mod str;
pub mod traits;
pub mod tsort;
pub mod ty;
pub mod value;

use crate::set::Set;
pub use crate::str::Str;

pub type RcArray<T> = std::rc::Rc<[T]>;

pub fn open_read(filename: &str) -> std::io::Result<String> {
    let f = std::fs::File::open(filename)?;
    read_file(f)
}

pub fn read_file(mut f: std::fs::File) -> std::io::Result<String> {
    let mut s = "".to_string();
    std::io::Read::read_to_string(&mut f, &mut s).unwrap();
    Ok(s)
}

pub fn fmt_vec<T: fmt::Display>(v: &Vec<T>) -> String {
    fmt_iter(v.iter())
}

pub fn fmt_slice<T: fmt::Display>(v: &[T]) -> String {
    fmt_iter(v.iter())
}

pub fn fmt_vec_split_with<T: fmt::Display>(v: &Vec<T>, splitter: &str) -> String {
    fmt_iter_split_with(v.iter(), splitter)
}

pub fn fmt_set_split_with<T: fmt::Display + std::hash::Hash>(s: &Set<T>, splitter: &str) -> String {
    fmt_iter_split_with(s.iter(), splitter)
}

pub fn debug_fmt_iter<T: fmt::Debug, I: Iterator<Item = T>>(iter: I) -> String {
    let mut s = iter.fold("".to_string(), |sum, elem| format!("{sum}{elem:?}, "));
    s.pop();
    s.pop();
    s
}

pub fn fmt_iter<T: fmt::Display, I: Iterator<Item = T>>(iter: I) -> String {
    let mut s = iter.fold("".to_string(), |sum, elem| sum + &elem.to_string() + ", ");
    s.pop();
    s.pop();
    s
}

pub fn fmt_iter_split_with<T: fmt::Display, I: Iterator<Item = T>>(i: I, splitter: &str) -> String {
    let mut s = i.fold("".to_string(), |sum, elem| {
        sum + &elem.to_string() + splitter
    });
    for _ in 0..splitter.len() {
        s.pop();
    }
    s
}

pub fn fmt_indent(s: String, depth: usize) -> String {
    let indent = " ".repeat(depth);
    s.split('\n').map(|s| indent.clone() + s).collect()
}

pub fn get_hash<T: std::hash::Hash>(t: &T) -> usize {
    let mut s = fxhash::FxHasher::default();
    t.hash(&mut s);
    let res = std::hash::Hasher::finish(&s);
    if cfg!(target_pointer_width = "64") {
        res as usize
    } else {
        (res % usize::MAX as u64) as usize
    }
}

/// \r\n (Windows), \r (old MacOS) -> \n (Unix)
#[inline]
pub fn normalize_newline(src: &str) -> String {
    src.replace("\r\n", "\n").replace("\r", "\n")
}

/// cut \n
#[inline]
pub fn chomp(src: &str) -> String {
    normalize_newline(src).replace("\n", "")
}

pub fn try_map<T, U, E, F, I>(i: I, f: F) -> Result<Vec<U>, E>
where
    F: Fn(T) -> Result<U, E>,
    I: Iterator<Item = T>,
{
    let mut v = vec![];
    for x in i {
        let y = f(x)?;
        v.push(y);
    }
    Ok(v)
}

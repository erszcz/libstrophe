#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]


#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;



pub mod src {
pub mod auth;
pub mod conn;
pub mod crypto;
pub mod ctx;
pub mod event;
pub mod handler;
pub mod hash;
pub mod jid;
pub mod md5;
pub mod parser_expat;
pub mod rand;
pub mod resolver;
pub mod sasl;
pub mod scram;
pub mod sha1;
pub mod snprintf;
pub mod sock;
pub mod stanza;
pub mod tls_openssl;
pub mod util;
pub mod uuid;
} // mod src


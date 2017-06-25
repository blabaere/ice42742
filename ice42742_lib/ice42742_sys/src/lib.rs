#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

pub use libc::*;

pub use posix_consts::*;

pub const ETERM: c_int = posix_consts::NN_HAUSNUMERO + 53;
pub const EFSM: c_int = posix_consts::NN_HAUSNUMERO + 54;

#[cfg(not(windows))]
pub mod posix_consts {
    use libc::*;
    pub const NN_HAUSNUMERO: c_int = 156384712;
    pub const EACCESS: c_int = ::libc::EACCES;
}

#[cfg(windows)]
pub mod posix_consts {
    use libc::c_int;

    pub const NN_HAUSNUMERO: c_int = 156384712;

    pub const ENOTSUP:         c_int = NN_HAUSNUMERO + 1;
    pub const EPROTO:          c_int = NN_HAUSNUMERO + 11;
    pub const EACCESS:         c_int = NN_HAUSNUMERO + 17;
    pub const EISCONN:         c_int = NN_HAUSNUMERO + 27;
    pub const ESOCKTNOSUPPORT: c_int = NN_HAUSNUMERO + 28;

    pub const EADDRINUSE:      c_int = 100;
    pub const EADDRNOTAVAIL:   c_int = 101;
    pub const EAFNOSUPPORT:    c_int = 102;
    pub const ECONNABORTED:    c_int = 106;
    pub const ECONNREFUSED:    c_int = 107;
    pub const ECONNRESET:      c_int = 108;
    pub const EHOSTUNREACH:    c_int = 110;
    pub const EINPROGRESS:     c_int = 112;
    pub const EMSGSIZE:        c_int = 115;
    pub const ENETDOWN:        c_int = 116;
    pub const ENETRESET:       c_int = 117;
    pub const ENETUNREACH:     c_int = 118;
    pub const ENOBUFS:         c_int = 119;
    pub const ENOPROTOOPT:     c_int = 123;
    pub const ENOTCONN:        c_int = 126;
    pub const ENOTSOCK:        c_int = 128;
    pub const EPROTONOSUPPORT: c_int = 135;
    pub const ETIMEDOUT:       c_int = 138;
}


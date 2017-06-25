extern crate libc;

use libc::{c_int};
use ice42742_sys;

use std::str;
use std::fmt;
use std::io;
use std::convert::From;
use std::result;
use std::error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Error {
    Unknown                    = 0 as isize,
    OperationNotSupported      = ice42742_sys::ENOTSUP          as isize,
    ProtocolNotSupported       = ice42742_sys::EPROTONOSUPPORT  as isize,
    NoBufferSpace              = ice42742_sys::ENOBUFS          as isize,
    NetworkDown                = ice42742_sys::ENETDOWN         as isize,
    AddressInUse               = ice42742_sys::EADDRINUSE       as isize,
    AddressNotAvailable        = ice42742_sys::EADDRNOTAVAIL    as isize,
    ConnectionRefused          = ice42742_sys::ECONNREFUSED     as isize,
    OperationNowInProgress     = ice42742_sys::EINPROGRESS      as isize,
    NotSocket                  = ice42742_sys::ENOTSOCK         as isize,
    AddressFamilyNotSupported  = ice42742_sys::EAFNOSUPPORT     as isize,
    #[cfg(not(target_os = "openbsd"))]
    WrongProtocol              = ice42742_sys::EPROTO           as isize,
    #[cfg(target_os = "openbsd")]
    WrongProtocol              = ice42742_sys::EPROTOTYPE       as isize,
    TryAgain                   = ice42742_sys::EAGAIN           as isize,
    BadFileDescriptor          = ice42742_sys::EBADF            as isize,
    InvalidInput               = ice42742_sys::EINVAL           as isize,
    TooManyOpenFiles           = ice42742_sys::EMFILE           as isize,
    BadAddress                 = ice42742_sys::EFAULT           as isize,
    PermissionDenied           = ice42742_sys::EACCESS          as isize,
    NetworkReset               = ice42742_sys::ENETRESET        as isize,
    NetworkUnreachable         = ice42742_sys::ENETUNREACH      as isize,
    HostUnreachable            = ice42742_sys::EHOSTUNREACH     as isize,
    NotConnected               = ice42742_sys::ENOTCONN         as isize,
    MessageTooLong             = ice42742_sys::EMSGSIZE         as isize,
    TimedOut                   = ice42742_sys::ETIMEDOUT        as isize,
    ConnectionAborted          = ice42742_sys::ECONNABORTED     as isize,
    ConnectionReset            = ice42742_sys::ECONNRESET       as isize,
    ProtocolNotAvailable       = ice42742_sys::ENOPROTOOPT      as isize,
    AlreadyConnected           = ice42742_sys::EISCONN          as isize,
    SocketTypeNotSupported     = ice42742_sys::ESOCKTNOSUPPORT  as isize,
    Terminating                = ice42742_sys::ETERM            as isize,
    NameTooLong                = ice42742_sys::ENAMETOOLONG     as isize,
    NoDevice                   = ice42742_sys::ENODEV           as isize,
    FileStateMismatch          = ice42742_sys::EFSM             as isize,
    Interrupted                = ice42742_sys::EINTR            as isize
}

impl Error {
    pub fn to_raw(&self) -> c_int {
        *self as c_int
    }

    pub fn from_raw(raw: c_int) -> Error {
        match raw {
            ice42742_sys::ENOTSUP         => Error::OperationNotSupported    ,
            ice42742_sys::EPROTONOSUPPORT => Error::ProtocolNotSupported     ,
            ice42742_sys::ENOBUFS         => Error::NoBufferSpace            ,
            ice42742_sys::ENETDOWN        => Error::NetworkDown              ,
            ice42742_sys::EADDRINUSE      => Error::AddressInUse             ,
            ice42742_sys::EADDRNOTAVAIL   => Error::AddressNotAvailable      ,
            ice42742_sys::ECONNREFUSED    => Error::ConnectionRefused        ,
            ice42742_sys::EINPROGRESS     => Error::OperationNowInProgress   ,
            ice42742_sys::ENOTSOCK        => Error::NotSocket                ,
            ice42742_sys::EAFNOSUPPORT    => Error::AddressFamilyNotSupported,
            #[cfg(not(target_os = "openbsd"))]
            ice42742_sys::EPROTO          => Error::WrongProtocol            ,
            #[cfg(target_os = "openbsd")]
            ice42742_sys::EPROTOTYPE      => Error::WrongProtocol            ,
            ice42742_sys::EAGAIN          => Error::TryAgain                 ,
            ice42742_sys::EBADF           => Error::BadFileDescriptor        ,
            ice42742_sys::EINVAL          => Error::InvalidInput             ,
            ice42742_sys::EMFILE          => Error::TooManyOpenFiles         ,
            ice42742_sys::EFAULT          => Error::BadAddress               ,
            ice42742_sys::EACCESS         => Error::PermissionDenied         ,
            ice42742_sys::ENETRESET       => Error::NetworkReset             ,
            ice42742_sys::ENETUNREACH     => Error::NetworkUnreachable       ,
            ice42742_sys::EHOSTUNREACH    => Error::HostUnreachable          ,
            ice42742_sys::ENOTCONN        => Error::NotConnected             ,
            ice42742_sys::EMSGSIZE        => Error::MessageTooLong           ,
            ice42742_sys::ETIMEDOUT       => Error::TimedOut                 ,
            ice42742_sys::ECONNABORTED    => Error::ConnectionAborted        ,
            ice42742_sys::ECONNRESET      => Error::ConnectionReset          ,
            ice42742_sys::ENOPROTOOPT     => Error::ProtocolNotAvailable     ,
            ice42742_sys::EISCONN         => Error::AlreadyConnected         ,
            ice42742_sys::ESOCKTNOSUPPORT => Error::SocketTypeNotSupported   ,
            ice42742_sys::ETERM           => Error::Terminating              ,
            ice42742_sys::ENAMETOOLONG    => Error::NameTooLong              ,
            ice42742_sys::ENODEV          => Error::NoDevice                 ,
            ice42742_sys::EFSM            => Error::FileStateMismatch        ,
            ice42742_sys::EINTR           => Error::Interrupted              ,
            _                            => Error::Unknown
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Some error" 
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        match err.kind() {
            io::ErrorKind::PermissionDenied    => Error::PermissionDenied,
            io::ErrorKind::ConnectionRefused   => Error::ConnectionRefused,
            io::ErrorKind::ConnectionReset     => Error::ConnectionReset,
            io::ErrorKind::ConnectionAborted   => Error::ConnectionAborted,
            io::ErrorKind::NotConnected        => Error::NotConnected,
            io::ErrorKind::AddrInUse           => Error::AddressInUse,
            io::ErrorKind::AddrNotAvailable    => Error::AddressNotAvailable,
            io::ErrorKind::AlreadyExists       => Error::AlreadyConnected,
            io::ErrorKind::WouldBlock          => Error::TryAgain,
            io::ErrorKind::InvalidInput        => Error::InvalidInput,
            io::ErrorKind::TimedOut            => Error::TimedOut,
            io::ErrorKind::Interrupted         => Error::Interrupted,
            _                                  => Error::Unknown
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        let as_std_error: &error::Error = &err;
        let description = as_std_error.description();
        match err {
            Error::PermissionDenied      => io::Error::new(io::ErrorKind::PermissionDenied,  description ),
            Error::ConnectionRefused     => io::Error::new(io::ErrorKind::ConnectionRefused, description ),
            Error::ConnectionReset       => io::Error::new(io::ErrorKind::ConnectionReset,   description ),
            Error::ConnectionAborted     => io::Error::new(io::ErrorKind::ConnectionAborted, description ),
            Error::NotConnected          => io::Error::new(io::ErrorKind::NotConnected,      description ),
            Error::AddressInUse          => io::Error::new(io::ErrorKind::AddrInUse,         description ),
            Error::AddressNotAvailable   => io::Error::new(io::ErrorKind::AddrNotAvailable,  description ),
            Error::AlreadyConnected      => io::Error::new(io::ErrorKind::AlreadyExists,     description ),
            Error::TryAgain              => io::Error::new(io::ErrorKind::WouldBlock,        description ),
            Error::InvalidInput          => io::Error::new(io::ErrorKind::InvalidInput,      description ),
            Error::TimedOut              => io::Error::new(io::ErrorKind::TimedOut,          description ),
            Error::Interrupted           => io::Error::new(io::ErrorKind::Interrupted,       description ),
            _                            => io::Error::new(io::ErrorKind::Other,             description )
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let as_std_error: &error::Error = self;
        let description = as_std_error.description();
        write!(formatter, "{}", description)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let as_std_error: &error::Error = self;
        let description = as_std_error.description();
        write!(formatter, "{}", description)
    }
}

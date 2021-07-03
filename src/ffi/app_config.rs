use crate::ffi::typedefs;

type LogFn = fn(typedefs::ConstCCharPtr);

#[repr(C)]
pub struct LogFunctions {
    pub debug: LogFn,
    pub error: LogFn,
    pub info: LogFn,
    pub warning: LogFn,
}

#[repr(C)]
pub struct Functions {
    pub log: LogFunctions,
}

#[repr(C)]
pub struct AppConfig {
    pub functions: Functions
}
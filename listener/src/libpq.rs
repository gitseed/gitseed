#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types, dead_code)]
pub enum ConnStatusType {
    CONNECTION_OK = 0,
    CONNECTION_BAD = 1,
    CONNECTION_STARTED = 2,
    CONNECTION_MADE = 3,
    CONNECTION_AWAITING_RESPONSE = 4,
    CONNECTION_AUTH_OK = 5,
    CONNECTION_SETENV = 6,
    CONNECTION_SSL_STARTUP = 7,
    CONNECTION_NEEDED = 8,
    CONNECTION_CHECK_WRITABLE = 9,
    CONNECTION_CONSUME = 10,
    CONNECTION_GSS_STARTUP = 11,
    CONNECTION_CHECK_TARGET = 12,
    CONNECTION_CHECK_STANDBY = 13,
    CONNECTION_ALLOCATED = 14,
}

unsafe extern "C" {
    // Note, we need to use the extern_types unstable feature for this
    // This is because the layout and size of PGconn is not guaranteed even between minor versions of postgres
    pub type PGconn;

    pub unsafe fn PQconnectdb(conninfo: *const std::ffi::c_char) -> *const PGconn;
    pub unsafe fn PQserverVersion(conn: *const PGconn) -> std::ffi::c_int;
    pub unsafe fn PQerrorMessage(conn: *const PGconn) -> *const std::ffi::c_char;
    pub unsafe fn PQstatus(conn: *const PGconn) -> ConnStatusType;
}

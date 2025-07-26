unsafe extern "C" {
    // Note, we need to use the extern_types unstable feature for this
    // This is because the layout and size of PGconn is not guaranteed even between minor versions of postgres
    pub type PGconn;
    pub unsafe fn PQconnectdb(conninfo: *const std::ffi::c_char) -> *const PGconn;
    pub unsafe fn PQserverVersion(conn: *const PGconn) -> std::ffi::c_int;
}

use crate::libpq;

pub fn connect(conninfo: &str) -> *const libpq::PGconn {
    let raw_conninfo: *mut std::ffi::c_char = std::ffi::CString::new(conninfo)
        .expect("Postgres connection info should not contain internal null characters")
        .into_raw();
    unsafe { libpq::PQconnectdb(raw_conninfo) }
}

pub fn server_version(conn: *const libpq::PGconn) -> i64 {
    (unsafe { libpq::PQserverVersion(conn) }) as i64
}

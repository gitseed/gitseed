#![allow(warnings)]

type uuid_t = [std::ffi::c_char; 16];

unsafe extern "C" {
    fn uuid_generate(out: *mut uuid_t);
}

fn main() {
    unsafe {
        let mut uuid : uuid_t = [0; 16];
        uuid_generate(&raw mut uuid);
        let uuid_numeric: u128 = std::mem::transmute::<uuid_t, u128>(uuid);
        println!("{}", uuid_numeric);
    }
}

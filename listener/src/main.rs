mod c_uuid {
    use std::mem::MaybeUninit;
    #[allow(non_camel_case_types)] // I want to use the same type name that the C header is using
    pub type uuid_t = [std::ffi::c_uchar; 16];
    unsafe extern "C" {
        pub unsafe fn uuid_generate(out: *mut MaybeUninit<uuid_t>);
    }
}
use std::mem::MaybeUninit;

fn main() {
    let new_uuid: u128 = uuid_generate();
    println!("{:?}", new_uuid)
}

pub fn c_uuid_to_u128(uuid: c_uuid::uuid_t) -> u128 {
    u128::from_ne_bytes(uuid.map(|b| b as u8))
}

fn uuid_generate() -> u128 {
    let mut result: MaybeUninit<c_uuid::uuid_t> = MaybeUninit::<c_uuid::uuid_t>::uninit();
    c_uuid_to_u128(unsafe {
        c_uuid::uuid_generate(&raw mut result);
        result.assume_init()
    })
}

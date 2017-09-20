mod ffi {
    extern crate libc;
    use self::libc::{size_t, c_int, c_char};

    #[link(name = "loader")]
    extern {
        pub fn load_pem_key(pemstr: *mut c_char, pemstr_len: size_t, password: *mut c_char,
                            out_priv_key: *mut c_char, out_pub_key: *mut c_char) -> c_int;
    }

}
extern crate libc;
use std::ffi::CString;

pub fn load_pem_key(pemstr: &str, pemstr_len: usize, password: &str) -> Result<(String, String), String> {
    let c_pemstr = CString::new(pemstr).unwrap();
    let c_password = CString::new(password).unwrap();
    let mut c_out_priv_key = CString::new("-----------------------------------------------------------------").unwrap();
    let mut c_out_pub_key = CString::new("-----------------------------------------------------------------------------------------------------------------------------------").unwrap();

    let err_num = unsafe {
        let c_ptr_pemstr = c_pemstr.into_raw();
        let c_ptr_password = c_password.into_raw();
        let c_ptr_out_priv_key = c_out_priv_key.into_raw();
        let c_ptr_out_pub_key = c_out_pub_key.into_raw();

        let err_num = ffi::load_pem_key(c_ptr_pemstr, pemstr_len, c_ptr_password,
            c_ptr_out_priv_key, c_ptr_out_pub_key);

        // Need to take back ownership of all pointers to avoid memory leak
        let _ = CString::from_raw(c_ptr_pemstr);
        let _ = CString::from_raw(c_ptr_password);

        // Need to return these
        c_out_priv_key = CString::from_raw(c_ptr_out_priv_key);
        c_out_pub_key = CString::from_raw(c_ptr_out_pub_key);

        err_num
    };

    match err_num {
        -1 => Err(String::from("Failed to decrypt or decode private key")),
        -2 => Err(String::from("Failed to create new big number context")),
        -3 => Err(String::from("Failed to load group")),
        -4 => Err(String::from("Failed to load private key")),
        -5 => Err(String::from("Failed to load public key point")),
        -6 => Err(String::from("Failed to construct public key from point")),
        _ => Ok((c_out_priv_key.into_string().unwrap(), c_out_pub_key.into_string().unwrap())),
    }
}

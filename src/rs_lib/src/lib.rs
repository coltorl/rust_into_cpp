extern crate libc;
use libc::{c_char, free, malloc};
use serde::Deserialize;
use std::ffi::{CString, CStr};
use std::ptr;

#[no_mangle]
pub extern "C" fn rs_lib_unsafe_hello() -> *mut libc::c_char {
    unsafe {
        let rust_string = "Hello Unsafe!";
        let c_string = CString::new(rust_string).expect("CString::new failed");
        let len = c_string.as_bytes_with_nul().len();
        let buffer = malloc(len) as *mut u8;

        // Check if malloc failed
        if buffer.is_null() {
            return ptr::null_mut();
        }

        // Copy the CString into the malloc'd buffer
        ptr::copy_nonoverlapping(c_string.as_ptr(), buffer as *mut libc::c_char, len);

        buffer as *mut libc::c_char
    }
}


#[cxx::bridge]
mod ffi {

    struct QuoteFfi {
        q: String,
        a: String,
        h: String,
    }

    struct QuoteResult {
        success: bool,
        data: QuoteFfi,
        msg: String,
    }

    extern "Rust" {
        fn rs_lib_hello() -> String;
        unsafe fn rs_lib_c_hello() -> String;
        fn rs_lib_cxx_hello() -> String;
        fn rs_lib_quote() -> QuoteResult;
    }
}

extern "C" {
    fn c_lib_hello(str: *mut *mut c_char) -> i32;
}

#[cxx::bridge(namespace = "cxx_lib")]
mod fficxx {
    unsafe extern "C++" {
        include!("cxx_lib.hpp");
        fn hello() -> UniquePtr<CxxString>;
    }
}

fn rs_lib_hello() -> String {
    "Hello Safety!".to_string()
}

unsafe fn rs_lib_c_hello() -> String {
    let mut buffer: *mut c_char = ptr::null_mut();

    let res = c_lib_hello(&mut buffer);

    if res == 0 {
        let ret = format!("C Says: {}", CStr::from_ptr(buffer).to_str().unwrap());
        free(buffer as *mut libc::c_void);
        ret
    } else {
        "C failed!".to_string()
    }
}

fn rs_lib_cxx_hello() -> String {
    format!("CXX Says: {}", fficxx::hello())
}

fn rs_lib_quote() -> ffi::QuoteResult {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(get_quote());

    match result {
        Ok(resp) => ffi::QuoteResult {
            success: true,
            data: resp.into(),
            msg: "".to_string(),
        },
        Err(e) => ffi::QuoteResult {
            success: false,
            data: ffi::QuoteFfi {
                q: "".to_string(),
                a: "".to_string(),
                h: "".to_string(),
            },
            msg: match e {
                QuoteError::Network(s) => s,
                QuoteError::NoQuotes => "No Quotes Found".to_string(),
                QuoteError::DeserlizationError => "DeserializationError".to_string(),
                QuoteError::Other => "Unknown Error".to_string(),
            },
        },
    }
}

pub enum QuoteError {
    Network(String),
    NoQuotes,
    DeserlizationError,
    Other,
}

#[derive(Deserialize, Debug)]
struct Quote {
    q: String,
    a: String,
    h: String,
}

// Implement conversion from Rust Quote to ffi Quote
impl From<Quote> for ffi::QuoteFfi {
    fn from(data: Quote) -> Self {
        ffi::QuoteFfi {
            q: data.q,
            a: data.a,
            h: data.h,
        }
    }
}

async fn get_quote() -> Result<Quote, QuoteError> {
    let url = "https://zenquotes.io/api/random";
    let response = reqwest::get(url).await;

    match response {
        Ok(resp) => {
            let quotes = resp
                .json::<Vec<Quote>>()
                .await
                .map_err(|_| QuoteError::DeserlizationError)?;
            quotes.into_iter().next().ok_or(QuoteError::NoQuotes)
        }
        Err(e) => {
            if e.is_connect() {
                Err(QuoteError::Network("Connection error".to_string()))
            } else if e.is_timeout() {
                Err(QuoteError::Network("Timeout error".to_string()))
            } else {
                Err(QuoteError::Other)
            }
        }
    }
}

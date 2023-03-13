//#![deny(warnings)]
#![no_implicit_prelude]
#![no_std]

extern crate core as core_real;
extern crate wit_bindgen_real;
extern crate wit_bindgen_rust;

use {
    core_real::{
        arch::wasm32,
        clone::Clone,
        convert::{From, Into},
        marker::PhantomData,
        marker::Sized,
        ops::{Deref, Fn},
        panic, ptr,
    },
    http_types as ht, spin_http as sh,
    std::{
        alloc::Layout,
        boxed::Box,
        iter::{FromIterator, IntoIterator, Iterator},
        option::Option::{self, None, Some},
        result::Result::{self, Err, Ok},
        string::String,
        vec::Vec,
    },
    wasi_outbound_http as woh,
};

pub mod core {
    use super::*;
    pub use crate::core_real::{hint, mem, ptr};

    pub mod alloc {
        use super::*;
        pub use crate::std::alloc::Layout;
    }
}

pub mod std {
    use super::*;
    pub use crate::core::ptr;

    pub mod option {
        use super::*;

        pub enum Option<T> {
            Some(T),
            None,
        }

        impl<T> Option<T> {
            pub fn as_deref<U: ?Sized>(&self) -> Option<&U>
            where
                T: Deref<Target = U>,
            {
                if let Some(v) = self {
                    Some(v.deref())
                } else {
                    None
                }
            }
        }

        impl<T: Clone> Clone for Option<T> {
            fn clone(&self) -> Self {
                if let Some(v) = self {
                    Some(v.clone())
                } else {
                    None
                }
            }
        }
    }

    pub mod result {
        use super::*;

        pub enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        impl<T, E> Result<T, E> {
            pub fn unwrap(self) -> T {
                wasm32::unreachable()
            }

            pub fn map<U>(self, f: impl Fn(T) -> U) -> Result<U, E> {
                match self {
                    Ok(v) => Ok(f(v)),
                    Err(e) => Err(e),
                }
            }

            pub fn map_err<U>(self, f: impl Fn(E) -> U) -> Result<T, U> {
                match self {
                    Ok(v) => Ok(v),
                    Err(e) => Err(f(e)),
                }
            }
        }
    }

    pub mod alloc {
        use super::*;

        #[derive(Copy, Clone)]
        pub struct Layout {
            size: usize,
            align: usize,
        }

        impl Layout {
            pub fn from_size_align_unchecked(size: usize, align: usize) -> Self {
                Self { size, align }
            }

            pub fn size(&self) -> usize {
                self.size
            }

            pub fn align(&self) -> usize {
                self.align
            }
        }

        pub unsafe fn alloc(layout: Layout) -> *mut u8 {
            #[link(wasm_import_module = "__main_module__")]
            extern "C" {
                fn cabi_realloc(
                    old_ptr: *mut u8,
                    old_len: usize,
                    align: usize,
                    new_len: usize,
                ) -> *mut u8;
            }

            cabi_realloc(ptr::null_mut(), 0, layout.align(), layout.size())
        }

        pub fn handle_alloc_error(layout: Layout) -> ! {
            wasm32::unreachable()
        }

        pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
            #[link(wasm_import_module = "__main_module__")]
            extern "C" {
                fn free(ptr: *mut u8, len: usize, align: usize);
            }

            free(ptr, layout.size(), layout.align())
        }
    }

    pub mod string {
        use super::*;

        #[derive(Clone)]
        pub struct String;

        impl String {
            pub fn from_utf8(bytes: Vec<u8>) -> Result<Self, ()> {
                wasm32::unreachable()
            }

            pub fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
                wasm32::unreachable()
            }

            pub fn into_bytes(self) -> Vec<u8> {
                wasm32::unreachable()
            }

            pub fn as_str(&self) -> &str {
                wasm32::unreachable()
            }
        }

        impl Deref for String {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                wasm32::unreachable()
            }
        }
    }

    pub mod iter {
        use super::*;

        pub struct Enumerate<T>(T);

        impl<T: Iterator> Iterator for Enumerate<T> {
            type Item = (usize, T::Item);
        }

        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item> {
                wasm32::unreachable()
            }

            fn enumerate(self) -> Enumerate<Self>
            where
                Self: Sized,
            {
                wasm32::unreachable()
            }

            fn collect<B: FromIterator<Self::Item>>(self) -> B
            where
                Self: Sized,
            {
                wasm32::unreachable()
            }

            fn map<U, F: Fn(Self::Item) -> U>(self, f: F) -> Map<Self, F>
            where
                Self: Sized,
            {
                wasm32::unreachable()
            }
        }

        pub trait FromIterator<A>: Sized {
            fn from_iter<T>(iter: T) -> Self
            where
                T: IntoIterator<Item = A>;
        }

        pub trait IntoIterator {
            type Item;
            type IntoIter: Iterator<Item = Self::Item>;

            fn into_iter(self) -> Self::IntoIter;
        }

        pub struct SliceIterator<'a, T>(PhantomData<&'a T>);

        impl<'a, T> Iterator for SliceIterator<'a, T> {
            type Item = &'a T;
        }

        impl<'a, T> IntoIterator for &'a [T] {
            type Item = &'a T;
            type IntoIter = SliceIterator<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                wasm32::unreachable()
            }
        }

        pub struct Map<I, F>(I, F);

        impl<T, I: Iterator<Item = T>, U, F: Fn(T) -> U> Iterator for Map<I, F> {
            type Item = U;
        }
    }

    pub mod boxed {
        use super::*;

        #[derive(Clone)]
        pub struct Box<T: ?Sized>(PhantomData<T>);

        impl<T> Box<[T]> {
            pub fn len(&self) -> usize {
                wasm32::unreachable()
            }

            pub fn as_ptr(&self) -> *const T {
                wasm32::unreachable()
            }
        }
    }

    pub mod vec {
        use super::*;

        #[derive(Clone)]
        pub struct Vec<T>(PhantomData<T>);

        impl<T> Vec<T> {
            pub fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
                wasm32::unreachable()
            }

            pub fn with_capacity(capacity: usize) -> Self {
                wasm32::unreachable()
            }

            pub fn push(&mut self, v: T) {
                wasm32::unreachable()
            }

            pub fn len(&self) -> usize {
                wasm32::unreachable()
            }

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            pub fn into_boxed_slice(&self) -> Box<[T]> {
                wasm32::unreachable()
            }

            pub fn iter(&self) -> super::iter::SliceIterator<'_, T> {
                wasm32::unreachable()
            }
        }

        impl<T> Deref for Vec<T> {
            type Target = [T];

            fn deref(&self) -> &Self::Target {
                wasm32::unreachable()
            }
        }

        pub struct VecIterator<T>(PhantomData<T>);

        impl<T> Iterator for VecIterator<T> {
            type Item = T;

            fn next(&mut self) -> Option<T> {
                wasm32::unreachable()
            }
        }

        impl<T> IntoIterator for Vec<T> {
            type Item = T;
            type IntoIter = VecIterator<T>;

            fn into_iter(self) -> Self::IntoIter {
                wasm32::unreachable()
            }
        }

        impl<T> FromIterator<T> for Vec<T> {
            fn from_iter<I>(iter: I) -> Self {
                wasm32::unreachable()
            }
        }
    }
}

mod wit_bindgen {
    use super::*;

    pub mod rt {
        use super::*;
        pub use crate::std::{alloc, string, vec};

        pub unsafe fn dealloc(ptr: i32, size: usize, align: usize) {
            if size == 0 {
                return;
            }
            let layout = Layout::from_size_align_unchecked(size, align);
            alloc::dealloc(ptr as *mut u8, layout);
        }

        pub fn as_i32(v: u16) -> i32 {
            v as i32
        }

        pub fn run_ctors_once() {
            static mut RUN: bool = false;
            unsafe {
                if !RUN {
                    // This function is synthesized by `wasm-ld` to run all static
                    // constructors. wasm-ld will either provide an implementation
                    // of this symbol, or synthesize a wrapper around each
                    // exported function to (unconditionally) run ctors. By using
                    // this function, the linked module is opting into "manually"
                    // running ctors.
                    extern "C" {
                        fn __wasm_call_ctors();
                    }
                    __wasm_call_ctors();
                    RUN = true;
                }
            }
        }
    }
}

wit_bindgen_rust::import!("../../wit/ephemeral/spin-http.wit");
wit_bindgen_rust::export!("../../wit/ephemeral/wasi-outbound-http.wit");
wit_bindgen_real::generate!("spin" in "../../wit/preview2/spin.wit");

impl From<woh::Method> for ht::Method {
    fn from(method: woh::Method) -> Self {
        use woh::Method::*;

        match method {
            Get => Self::Get,
            Post => Self::Post,
            Put => Self::Put,
            Delete => Self::Delete,
            Patch => Self::Patch,
            Head => Self::Head,
            Options => Self::Options,
        }
    }
}

impl From<ht::Method> for sh::Method {
    fn from(method: ht::Method) -> Self {
        use ht::Method::*;

        match method {
            Get => Self::Get,
            Post => Self::Post,
            Put => Self::Put,
            Delete => Self::Delete,
            Patch => Self::Patch,
            Head => Self::Head,
            Options => Self::Options,
        }
    }
}

impl From<ht::Response> for woh::Response {
    fn from(res: ht::Response) -> Self {
        Self {
            status: res.status,
            headers: res.headers,
            body: res.body,
        }
    }
}

impl From<sh::Response> for ht::Response {
    fn from(res: sh::Response) -> Self {
        Self {
            status: res.status,
            headers: res.headers,
            body: res.body,
        }
    }
}

impl From<ht::HttpError> for woh::HttpError {
    fn from(error: ht::HttpError) -> Self {
        use ht::HttpError::*;

        match error {
            Success => Self::Success,
            DestinationNotAllowed => Self::DestinationNotAllowed,
            InvalidUrl => Self::InvalidUrl,
            RequestError => Self::RequestError,
            RuntimeError => Self::RuntimeError,
            TooManyRequests => Self::TooManyRequests,
        }
    }
}

impl<'a> From<ht::RequestParam<'a>> for sh::Request<'a> {
    fn from(req: ht::RequestParam<'a>) -> Self {
        Self {
            method: req.method.into(),
            uri: req.uri,
            headers: req.headers,
            params: req.params,
            body: req.body,
        }
    }
}

fn into(v: &Vec<(String, String)>) -> Vec<(&str, &str)> {
    v.iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect::<Vec<_>>()
}

struct WasiOutboundHttp;

impl woh::WasiOutboundHttp for WasiOutboundHttp {
    fn request(req: woh::Request) -> Result<woh::Response, woh::HttpError> {
        outbound_http::send_request(ht::RequestParam {
            method: req.method.into(),
            uri: &req.uri,
            headers: &into(&req.headers),
            params: &into(&req.params),
            body: req.body.as_deref(),
        })
        .map(Into::into)
        .map_err(Into::into)
    }
}

struct InboundHttp;

impl inbound_http::InboundHttp for InboundHttp {
    fn handle_request(req: ht::RequestResult) -> ht::Response {
        sh::handle_http_request(sh::Request {
            method: req.method.into(),
            uri: &req.uri,
            headers: &into(&req.headers),
            params: &into(&req.params),
            body: req.body.as_deref(),
        })
        .into()
    }
}

export_spin!(InboundHttp);

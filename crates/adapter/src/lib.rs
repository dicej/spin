//#![deny(warnings)]
#![no_implicit_prelude]
#![no_std]

extern crate alloc;
extern crate core;
extern crate wit_bindgen_real;
extern crate wit_bindgen_rust;

use {
    alloc::alloc::{GlobalAlloc, Layout},
    core::{
        arch::wasm32,
        clone::Clone,
        convert::{From, Into, TryFrom, TryInto},
        default::Default,
        hint, mem,
        ops::{Deref, Drop},
        option::Option::{self, None, Some},
        ptr,
        result::Result::{self, Err, Ok},
        slice, str,
    },
    http_types as ht, spin_http as sh,
    std::{
        iter::{FromIterator, IntoIterator, Iterator},
        string::String,
        vec::Vec,
    },
    wasi_outbound_http as woh,
};

struct State {
    spin_http_ret_area: spin_http::RetArea,
    wasi_outbound_http_ret_area: wasi_outbound_http::RetArea,
    inbound_http_ret_area: inbound_http::RetArea,
}

#[allow(improper_ctypes)]
extern "C" {
    fn get_state_ptr() -> *mut State;
    fn set_state_ptr(state: *mut State);
}

struct MyAllocator;

#[global_allocator]
static ALLOCATOR: MyAllocator = MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
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

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        #[link(wasm_import_module = "__main_module__")]
        extern "C" {
            fn free(ptr: *mut u8, len: usize, align: usize);
        }

        free(ptr, layout.size(), layout.align())
    }
}

fn unwrap_result<T, E>(result: Result<T, E>) -> T {
    if let Ok(v) = result {
        v
    } else {
        wasm32::unreachable()
    }
}

unsafe fn state() -> *mut State {
    let mut ptr = get_state_ptr();
    if ptr.is_null() {
        ptr = std::alloc::alloc(Layout::from_size_align_unchecked(
            mem::size_of::<State>(),
            mem::align_of::<State>(),
        )) as _;
        ptr.write(State {
            spin_http_ret_area: spin_http::RetArea(Default::default()),
            wasi_outbound_http_ret_area: wasi_outbound_http::RetArea(Default::default()),
            inbound_http_ret_area: inbound_http::RetArea(Default::default()),
        });
        set_state_ptr(ptr);
    }
    ptr
}

fn spin_http_get_ret_area() -> i32 {
    unsafe { (*state()).spin_http_ret_area.0.as_mut_ptr() as i32 }
}

fn wasi_outbound_http_get_ret_area() -> i32 {
    unsafe { (*state()).wasi_outbound_http_ret_area.0.as_mut_ptr() as i32 }
}

fn get_ret_area() -> i32 {
    unsafe { (*state()).inbound_http_ret_area.0.as_mut_ptr() as i32 }
}

pub mod std {
    use super::*;
    pub use crate::{
        alloc::boxed,
        core::{hint, iter, ptr},
    };

    pub mod alloc {
        use super::*;
        pub use crate::alloc::alloc::Layout;

        /// # Safety
        /// TODO
        pub unsafe fn alloc(layout: Layout) -> *mut u8 {
            MyAllocator.alloc(layout)
        }

        pub fn handle_alloc_error(_layout: Layout) -> ! {
            wasm32::unreachable()
        }

        /// # Safety
        /// TODO
        pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
            MyAllocator.dealloc(ptr, layout)
        }
    }

    pub mod string {
        use super::*;

        #[derive(Clone)]
        pub struct String(Vec<u8>);

        impl String {
            /// # Safety
            /// TODO
            pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
                Self(bytes)
            }

            pub fn into_bytes(self) -> Vec<u8> {
                self.0
            }

            pub fn as_str(&self) -> &str {
                unsafe { str::from_utf8_unchecked(self.0.deref()) }
            }
        }

        impl Deref for String {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.as_str()
            }
        }
    }

    pub mod vec {
        use super::*;

        #[derive(Clone)]
        pub struct Vec<T> {
            ptr: *mut T,
            length: usize,
            capacity: usize,
        }

        impl<T> Vec<T> {
            /// # Safety
            /// TODO
            pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
                Self {
                    ptr,
                    length,
                    capacity,
                }
            }

            pub fn len(&self) -> usize {
                self.length
            }

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            pub fn into_boxed_slice(self) -> Array<T> {
                let length = self.length;

                unsafe {
                    let ptr = alloc::alloc(Layout::from_size_align_unchecked(
                        mem::size_of::<T>() * length,
                        mem::align_of::<T>(),
                    )) as *mut T;

                    ptr::copy_nonoverlapping(self.ptr, ptr, self.length);

                    crate::std::alloc::dealloc(
                        self.ptr as _,
                        Layout::from_size_align_unchecked(
                            mem::size_of::<T>() * self.capacity,
                            mem::align_of::<T>(),
                        ),
                    );

                    mem::forget(self);

                    Array { ptr, length }
                }
            }

            pub fn iter(&self) -> RefIterator<'_, T> {
                RefIterator {
                    vec: self,
                    offset: 0,
                }
            }
        }

        impl<T> Drop for Vec<T> {
            fn drop(&mut self) {
                unsafe {
                    for i in 0..unwrap_result(isize::try_from(self.length)) {
                        mem::drop(ptr::read(self.ptr.offset(i)))
                    }
                    crate::std::alloc::dealloc(
                        self.ptr as _,
                        Layout::from_size_align_unchecked(
                            mem::size_of::<T>() * self.capacity,
                            mem::align_of::<T>(),
                        ),
                    )
                }
            }
        }

        impl<T> Deref for Vec<T> {
            type Target = [T];

            fn deref(&self) -> &Self::Target {
                unsafe { slice::from_raw_parts(self.ptr, self.length) }
            }
        }

        pub struct Array<T> {
            ptr: *mut T,
            length: usize,
        }

        impl<T> Array<T> {
            pub fn as_ptr(&self) -> *const T {
                self.ptr
            }

            pub fn len(&self) -> usize {
                self.length
            }

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }

        impl<T> Drop for Array<T> {
            fn drop(&mut self) {
                unsafe {
                    for i in 0..unwrap_result(isize::try_from(self.length)) {
                        mem::drop(ptr::read(self.ptr.offset(i)))
                    }
                    crate::std::alloc::dealloc(
                        self.ptr as _,
                        Layout::from_size_align_unchecked(
                            mem::size_of::<T>() * self.length,
                            mem::align_of::<T>(),
                        ),
                    )
                }
            }
        }

        pub struct RefIterator<'a, T> {
            vec: &'a Vec<T>,
            offset: isize,
        }

        impl<'a, T> Iterator for RefIterator<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<&'a T> {
                if unwrap_result(usize::try_from(self.offset)) < self.vec.len() {
                    let v = unsafe { &*self.vec.ptr.offset(self.offset) };
                    self.offset += 1;
                    Some(v)
                } else {
                    None
                }
            }
        }

        pub struct VecIterator<T> {
            ptr: *mut T,
            length: usize,
            capacity: usize,
            offset: isize,
        }

        impl<T> Drop for VecIterator<T> {
            fn drop(&mut self) {
                unsafe {
                    for i in self.offset..unwrap_result(isize::try_from(self.length)) {
                        mem::drop(ptr::read(self.ptr.offset(i)))
                    }
                    crate::std::alloc::dealloc(
                        self.ptr as _,
                        Layout::from_size_align_unchecked(
                            mem::size_of::<T>() * self.capacity,
                            mem::align_of::<T>(),
                        ),
                    )
                }
            }
        }

        impl<T> Iterator for VecIterator<T> {
            type Item = T;

            fn next(&mut self) -> Option<T> {
                if unwrap_result(usize::try_from(self.offset)) < self.length {
                    let v = unsafe { ptr::read(self.ptr.offset(self.offset)) };
                    self.offset += 1;
                    Some(v)
                } else {
                    None
                }
            }
        }

        impl<T> IntoIterator for Vec<T> {
            type Item = T;
            type IntoIter = VecIterator<T>;

            fn into_iter(self) -> Self::IntoIter {
                let iter = VecIterator {
                    ptr: self.ptr,
                    length: self.length,
                    capacity: self.capacity,
                    offset: 0,
                };
                mem::forget(self);
                iter
            }
        }

        impl<T> FromIterator<T> for Vec<T> {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                let mut iter = iter.into_iter();
                let len = iter.size_hint().0;

                unsafe {
                    let array = alloc::alloc(Layout::from_size_align_unchecked(
                        mem::size_of::<T>() * len,
                        mem::align_of::<T>(),
                    )) as *mut T;

                    if array.is_null() {
                        wasm32::unreachable();
                    }

                    let mut offset = 0;
                    for v in iter.by_ref() {
                        if offset < len {
                            array.offset(unwrap_result(offset.try_into())).write(v);
                            offset += 1;
                        } else {
                            // Only `ExactSizeIterator`s supported
                            wasm32::unreachable()
                        }
                    }

                    if iter.next().is_some() {
                        // Only `ExactSizeIterator`s supported
                        wasm32::unreachable()
                    }

                    Self::from_raw_parts(array, offset, len)
                }
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
            // ignore
        }
    }
}

wit_bindgen_rust::import!({
    paths: ["../../wit/ephemeral/spin-http.wit"],
    unchecked
});
wit_bindgen_rust::export!({
    paths: ["../../wit/ephemeral/wasi-outbound-http.wit"],
    unchecked
});
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

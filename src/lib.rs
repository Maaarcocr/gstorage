use std::io::Read;

use cxx::let_cxx_string;

#[cxx::bridge]
mod ffi {
    // C++ types and signatures exposed to Rust.

    unsafe extern "C++" {
        include!("gstorage/gstorage-c/client.hpp");

        type Stream;
        type Client;

        fn new_client() -> UniquePtr<Client>;
        fn read_object(self: Pin<&mut Client>, bucket: &CxxString, object: &CxxString) -> UniquePtr<Stream>;
        unsafe fn read(self: Pin<&mut Stream>, buf: *mut u8, n: i64) -> i64;
    }
}

pub struct Gstorage {
    inner: cxx::UniquePtr<ffi::Client>
}

pub struct Stream {
    inner: cxx::UniquePtr<ffi::Stream>
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let inner = self.inner.as_mut().unwrap();
        let read = unsafe { inner.read(buf.as_mut_ptr(), buf.len().try_into().unwrap()) };
        Ok(read.try_into().unwrap())
    }
}

impl Stream {
    pub fn new(inner: cxx::UniquePtr<ffi::Stream>) -> Self {
        Self {
            inner
        }
    }
}

impl Gstorage {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_client()
        }
    }

    pub fn read_object(&mut self, bucket: &str, object: &str) -> Stream {
        let_cxx_string!(bucket = bucket);
        let_cxx_string!(object = object);

        let inner = self.inner.as_mut().unwrap();
        let stream = inner.read_object(&bucket, &object);
        Stream::new(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_client() {
        Gstorage::new();
    }
}

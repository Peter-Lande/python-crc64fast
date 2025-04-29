use pyo3::prelude::*;
use crc64fast_nvme;

#[pymodule]
mod _lib {
    use super::*;


    #[pyfunction]
    fn crc64fast() -> Digest {
        Digest::new()
    }

    #[pyclass]
    struct Digest {
        inner: crc64fast_nvme::Digest, 
    }
    
    impl Digest {
        fn new() -> Self {
            Digest { inner: crc64fast_nvme::Digest::new() }
        }
    }

    #[pymethods]
    impl Digest {
        fn update(&mut self, bytes: &[u8]) {
            self.inner.write(bytes);
        }

        #[getter(name)]
        fn name(&self) -> &str {
            return "crc64-fast"
        }

        #[getter(digest_size)]
        fn digest_size(&self) -> u8 {
            return 8
        }

        #[getter(block_size)]
        fn block_size(&self) -> u8 {
            return 1
        }

        fn digest(&self) -> Vec<u8> {
            self.inner.sum64().to_be_bytes().to_vec()
        }

        fn hexdigest(&self) -> String {
            format!("{:x}", self.inner.sum64())
        }

        fn copy(&self) -> Self {
            Digest { inner: self.inner.clone() }
        }
    }
}

use pyo3::prelude::*;
use crc64fast_nvme;
use pyo3::types::PyDict;

#[pymodule]
mod _lib {
    use super::*;


    #[pyfunction]
    #[pyo3(signature = (**py_kwargs))]
    fn crc64fast(py_kwargs: Option<&Bound<'_, PyDict>>) -> Digest {
        if let Some(kwargs) = py_kwargs {
            if let Ok(Some(value)) = kwargs.get_item("table") {
                if value.is_truthy().is_ok_and(|x| x) {
                    Digest::new(true)
                }
                else {
                    Digest::new(false)
                }
            }
            else {
                Digest::new(false)
            }
        }
        else {
            Digest::new(false)
        }
    }

    #[pyclass]
    struct Digest {
        inner: crc64fast_nvme::Digest, 
    }
    
    impl Digest {
        fn new(table: bool) -> Self {
            if table {
                Digest { inner: crc64fast_nvme::Digest::new_table() }
            }
            else {
                Digest { inner: crc64fast_nvme::Digest::new() }
            }
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

//! Python bindings for the imposer library using `PyO3`
//!
//! This module provides Python bindings for the imposer PDF imposition library,
//! allowing Python code to impose PDFs into booklet layouts directly.

#![allow(clippy::useless_conversion)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::config::{BindingType, BookletConfig, PageSize};
use crate::error::BookletError;

/// Python wrapper for `PageSize` enum
#[pyclass(name = "PageSize")]
#[derive(Debug, Clone, Copy)]
pub struct PyPageSize {
    inner: PageSize,
}

#[pymethods]
impl PyPageSize {
    /// Create an A4 page size (210mm × 297mm)
    #[staticmethod]
    fn a4() -> Self {
        Self { inner: PageSize::A4 }
    }

    /// Create an A3 page size (297mm × 420mm)
    #[staticmethod]
    fn a3() -> Self {
        Self { inner: PageSize::A3 }
    }

    /// Create an A5 page size (148mm × 210mm)
    #[staticmethod]
    fn a5() -> Self {
        Self { inner: PageSize::A5 }
    }

    /// Create a Letter page size (8.5" × 11")
    #[staticmethod]
    fn letter() -> Self {
        Self {
            inner: PageSize::Letter,
        }
    }

    /// Create a Legal page size (8.5" × 14")
    #[staticmethod]
    fn legal() -> Self {
        Self { inner: PageSize::Legal }
    }

    /// Create a Tabloid page size (11" × 17")
    #[staticmethod]
    fn tabloid() -> Self {
        Self {
            inner: PageSize::Tabloid,
        }
    }

    /// Create a custom page size with given width and height in points (1 point = 1/72 inch)
    #[staticmethod]
    fn custom(width: f32, height: f32) -> Self {
        Self {
            inner: PageSize::Custom(width, height),
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Python wrapper for `BindingType` enum
#[pyclass(name = "BindingType")]
#[derive(Debug, Clone, Copy)]
pub struct PyBindingType {
    inner: BindingType,
}

#[pymethods]
impl PyBindingType {
    /// Saddle stitch binding (pages nested together)
    #[staticmethod]
    fn saddle_stitch() -> Self {
        Self {
            inner: BindingType::SaddleStitch,
        }
    }

    /// Perfect binding (signatures stacked sequentially)
    /// Accepts optional keyword arguments for compatibility with Python callers
    #[staticmethod]
    #[pyo3(signature = (sheets_per_signature=None, num_signatures=None))]
    fn perfect_bound(sheets_per_signature: Option<usize>, num_signatures: Option<usize>) -> Self {
        // Note: The Python-level `BindingType.perfect_bound(...)` accepts optional
        // keyword arguments for API convenience (sheets_per_signature, num_signatures),
        // but those values are actually stored on `BookletConfig` (the configuration
        // object). This function exists to expose the binding type value to Python and
        // keep the familiar factory signature. We therefore accept the kwargs here
        // and consume them to avoid unused-variable lint errors; callers should set
        // the parameters on a `BookletConfig` instance (via `.with_sheets_per_signature(...)`
        // or `.with_num_signatures(...)`) when they need to persist those values.
        let _ = sheets_per_signature;
        let _ = num_signatures;
        Self {
            inner: BindingType::PerfectBound,
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Python wrapper for `BookletConfig`
#[pyclass(name = "BookletConfig")]
#[derive(Debug, Clone)]
pub struct PyBookletConfig {
    inner: BookletConfig,
}

#[pymethods]
impl PyBookletConfig {
    /// Create a new booklet configuration with default settings
    ///
    /// This constructor accepts no arguments and returns the library default
    /// configuration (A4, 2-up, scale to fit, preserve aspect ratio).
    #[new]
    fn new() -> Self {
        Self {
            inner: BookletConfig::default(),
        }
    }

    /// Set the page size for this configuration
    ///
    /// # Arguments
    ///
    /// * `page_size` - A `PageSize` object
    fn with_page_size(mut slf: PyRefMut<'_, Self>, page_size: PyPageSize) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_binding_type(slf.inner.binding_type); // ensure clone
        slf.inner.page_size = page_size.inner;
        slf
    }

    /// Set the number of pages per sheet side (e.g., 2 for 2-up, 4 for 4-up)
    ///
    /// # Arguments
    ///
    /// * `pages` - Number of pages per sheet (must be a power of 2)
    fn with_pages_per_sheet(mut slf: PyRefMut<'_, Self>, pages: usize) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_pages_per_sheet(pages);
        slf
    }

    /// Set whether to scale source pages to fit the output size
    ///
    /// # Arguments
    ///
    /// * `scale` - True to scale pages to fit, false to use original size
    fn with_scale_to_fit(mut slf: PyRefMut<'_, Self>, scale: bool) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_scale_to_fit(scale);
        slf
    }

    /// Set whether to preserve aspect ratio when scaling
    ///
    /// # Arguments
    ///
    /// * `preserve` - True to preserve aspect ratio, false to stretch to fill
    fn with_preserve_aspect_ratio(mut slf: PyRefMut<'_, Self>, preserve: bool) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_preserve_aspect_ratio(preserve);
        slf
    }

    /// Set whether to draw guide lines on the output
    ///
    /// # Arguments
    ///
    /// * `draw` - True to draw guide lines, false to omit them
    fn with_draw_guides(mut slf: PyRefMut<'_, Self>, draw: bool) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_draw_guides(draw);
        slf
    }

    /// Set whether to show page numbers instead of actual content
    ///
    /// # Arguments
    ///
    /// * `number` - True to show page numbers, false to show actual content
    fn with_number_pages(mut slf: PyRefMut<'_, Self>, number: bool) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_number_pages(number);
        slf
    }

    /// Set the binding type (saddle stitch or perfect bound)
    ///
    /// # Arguments
    ///
    /// * `binding_type` - The binding type (`BindingType` object)
    fn with_binding_type(mut slf: PyRefMut<'_, Self>, binding_type: PyBindingType) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_binding_type(binding_type.inner);
        slf
    }

    /// Set the number of sheets per signature for perfect binding
    ///
    /// # Arguments
    ///
    /// * `sheets` - Number of sheets per signature (only applies to perfect binding)
    fn with_sheets_per_signature(mut slf: PyRefMut<'_, Self>, sheets: usize) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_sheets_per_signature(sheets);
        slf
    }

    /// Set the number of signatures for perfect binding
    ///
    /// # Arguments
    ///
    /// * `num` - Number of signatures (only applies to perfect binding, overrides `sheets_per_signature`)
    #[pyo3(signature = (num=None))]
    fn with_num_signatures(mut slf: PyRefMut<'_, Self>, num: Option<usize>) -> PyRefMut<'_, Self> {
        slf.inner = slf.inner.clone().with_num_signatures(num);
        slf
    }

    fn __repr__(&self) -> String {
        format!(
            "BookletConfig(page_size={:?}, pages_per_sheet={}, binding_type={:?})",
            self.inner.page_size, self.inner.pages_per_sheet, self.inner.binding_type
        )
    }
}

/// Generate a booklet from PDF bytes with the specified configuration
///
/// # Arguments
///
/// * `pdf_bytes` - The input PDF as bytes
/// * `config` - The booklet configuration
///
/// # Returns
///
/// The imposed PDF as bytes
///
/// # Errors
///
/// Returns an error if the PDF cannot be processed or the imposition fails
#[allow(clippy::useless_conversion)]
#[pyfunction]
fn generate_booklet(py: Python<'_>, pdf_bytes: &[u8], config: &PyBookletConfig) -> PyResult<Py<PyBytes>> {
    let result = crate::generate_booklet_with_config(pdf_bytes, &config.inner)
        .map_err(|e| PyValueError::new_err(format!("Failed to generate booklet: {e}")))?;

    Ok(PyBytes::new_bound(py, &result).into())
}

/// Generate a booklet from a PDF file path with the specified configuration
///
/// # Arguments
///
/// * `input_path` - Path to the input PDF file
/// * `output_path` - Path where the output PDF should be written
/// * `config` - The booklet configuration
///
/// # Errors
///
/// Returns an error if the file cannot be read/written or imposition fails
#[allow(clippy::useless_conversion)]
#[pyfunction]
fn generate_booklet_from_file(input_path: &str, output_path: &str, config: &PyBookletConfig) -> PyResult<()> {
    // Read input file
    let pdf_bytes =
        std::fs::read(input_path).map_err(|e| PyValueError::new_err(format!("Failed to read input file: {e}")))?;

    // Generate booklet
    let result = crate::generate_booklet_with_config(&pdf_bytes, &config.inner)
        .map_err(|e| PyValueError::new_err(format!("Failed to generate booklet: {e}")))?;

    // Write output file
    std::fs::write(output_path, result)
        .map_err(|e| PyValueError::new_err(format!("Failed to write output file: {e}")))?;

    Ok(())
}

/// Convert `BookletError` to Python exception
impl From<BookletError> for PyErr {
    fn from(err: BookletError) -> PyErr {
        PyValueError::new_err(format!("{err}"))
    }
}

/// The imposer Python module
#[pymodule]
fn imposer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPageSize>()?;
    m.add_class::<PyBindingType>()?;
    m.add_class::<PyBookletConfig>()?;
    m.add_function(wrap_pyfunction!(generate_booklet, m)?)?;
    m.add_function(wrap_pyfunction!(generate_booklet_from_file, m)?)?;
    Ok(())
}

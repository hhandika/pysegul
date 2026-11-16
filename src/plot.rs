//! Chart options shared by the summary classes.

use std::path::{Path, PathBuf};

use pyo3::{exceptions::PyValueError, prelude::*};
use segul_core::color::{Colormap, CvdKind, Palette};
use segul_core::plot::{
    batch::PlotWriter,
    fonts::PlotFonts,
    spec::{PlotFormat, PlotStyle},
};

use crate::common::{parse, path_str, to_py_err};

const PLOT_DIR: &str = "plots";
const FORMAT_ERR: &str = "Invalid plot format. Valid options: 'png', 'svg', 'pdf'";
const PALETTE_ERR: &str = "Invalid palette. Valid options: 'dark2', 'okabe-ito', \
    'tol-bright', 'tol-muted', 'set2', 'paired', 'set1'";
const COLORMAP_ERR: &str = "Invalid colormap. Valid options: 'viridis', 'cividis', \
    'magma', 'inferno', 'plasma', 'blues', 'greens', 'ylgnbu', 'ylorrd', \
    'rdbu', 'puor', 'brbg'";
const CVD_ERR: &str = "Invalid color vision deficiency. Valid options: \
    'protanopia', 'deuteranopia', 'tritanopia', 'achromatopsia'";

/// Chart options for summary classes. Charts are written to `<output>/plots`.
///
/// format: 'png', 'svg', or 'pdf'.
/// font: Font file for PNG and PDF text. Defaults to the system fonts.
/// palette: Colors for series, e.g. 'dark2' or 'okabe-ito'.
/// colormap: Colors for heatmaps, e.g. 'viridis'.
/// cvd: Simulate a color vision deficiency, e.g. 'deuteranopia'.
#[pyclass(frozen, from_py_object)]
#[derive(Clone)]
pub(crate) struct PlotOptions {
    format: PlotFormat,
    font: Option<PathBuf>,
    style: PlotStyle,
}

#[pymethods]
impl PlotOptions {
    #[new]
    #[pyo3(signature = (format="png", font=None, palette="dark2", colormap="viridis", cvd=None))]
    fn new(
        format: &str,
        font: Option<String>,
        palette: &str,
        colormap: &str,
        cvd: Option<&str>,
    ) -> PyResult<Self> {
        let cvd: Option<CvdKind> = cvd.map(|c| parse(c, CVD_ERR)).transpose()?;
        Ok(Self {
            format: parse(format, FORMAT_ERR)?,
            font: font.map(PathBuf::from),
            style: PlotStyle {
                palette: parse::<Palette>(palette, PALETTE_ERR)?,
                colormap: parse::<Colormap>(colormap, COLORMAP_ERR)?,
                cvd,
                ..Default::default()
            },
        })
    }
}

impl PlotOptions {
    /// Write the charts produced by `plot` into `<output>/plots`.
    /// Returns the chart paths.
    pub(crate) fn write<F>(
        &self,
        output: &Path,
        prefix: Option<&str>,
        plot: F,
    ) -> PyResult<Vec<String>>
    where
        F: FnOnce(&PlotWriter) -> Vec<PathBuf>,
    {
        let fonts = self.fonts()?;
        let plot_dir = output.join(PLOT_DIR);
        let writer = PlotWriter::new(&plot_dir, prefix, &self.format, &self.style, &fonts);
        Ok(plot(&writer).iter().map(|p| path_str(p)).collect())
    }

    // SVG uses the viewer's fonts, so none are loaded.
    fn fonts(&self) -> PyResult<PlotFonts> {
        if self.format == PlotFormat::Svg {
            return Ok(PlotFonts::new());
        }
        let fonts = match &self.font {
            Some(path) => {
                let mut fonts = PlotFonts::new();
                fonts.add_font_file(path).map_err(to_py_err)?;
                fonts
            }
            None => PlotFonts::system(),
        };
        if fonts.is_empty() {
            return Err(PyValueError::new_err(
                "No fonts found for charts. Set `font` or use format='svg'.",
            ));
        }
        Ok(fonts)
    }
}

/// Write charts when `plot` is set. Returns the chart paths.
pub(crate) fn write_plots<F>(
    plot: Option<&PlotOptions>,
    output: &Path,
    prefix: Option<&str>,
    f: F,
) -> PyResult<Vec<String>>
where
    F: FnOnce(&PlotWriter) -> Vec<PathBuf>,
{
    match plot {
        Some(options) => options.write(output, prefix, f),
        None => Ok(Vec::new()),
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlotOptions>()?;
    Ok(())
}

use libc::c_double;
use std::ptr::{null, null_mut};
use raster::{Dataset};
use gdal_sys::{self, CPLErr, GDALResampleAlg};
use utils::_last_cpl_err;

use errors::*;

/// Reproject 'Dataset' source to 'Dataset' destiny.
///
/// Use bilinear resampling algorithm, and approximating the transformation with exact calculations.
///
/// # Arguments
///  * `source` - from 'Dataset' source
///  * `destiny` - to 'Dataset' destiny
pub fn reproject(source: &Dataset, destiny: &Dataset) -> Result<()> {
    let rv = unsafe {
        gdal_sys::GDALReprojectImage(
                source._c_ptr(),
                null(),
                destiny._c_ptr(),
                null(),
                GDALResampleAlg::GRA_Bilinear,
                0.0,
                0.0 as c_double,
                None,
                null_mut(),
                null_mut()
            )
    };
    if rv != CPLErr::CE_None {
        Err(_last_cpl_err(rv))?;
    }
    Ok(())
}

/// Reproject 'Dataset' source to 'Dataset' destiny.
///
/// Use specified resampling algorithm, and approximating the transformation with exact calculations.
///
/// # Arguments
///  * `source` - from 'Dataset' source.
///  * `destiny` - to 'Dataset' destiny.
///  * `resampling` -  warp resampling algorithms.
///
/// ## Warp Resampling Algorithms:
/// 'GRA_NearestNeighbour': Nearest neighbour (select on one input pixel)
/// 'GRA_Bilinear': Bilinear (2x2 kernel)
/// 'GRA_Cubic': Cubic Convolution Approximation (4x4 kernel)
/// 'GRA_CubicSpline': Cubic B-Spline Approximation (4x4 kernel)
/// 'GRA_Lanczos': Lanczos windowed sinc interpolation (6x6 kernel)
/// 'GRA_Average': Average (computes the average of all non-NODATA contributing pixels)
/// 'GRA_Mode': Mode (selects the value which appears most often of all the sampled points)
/// 'GRA_Max': Max (selects maximum of all non-NODATA contributing pixels)
/// 'GRA_Min': Min (selects minimum of all non-NODATA contributing pixels)
/// 'GRA_Med': Med (selects median of all non-NODATA contributing pixels)
/// 'GRA_Q1': Q1 (selects first quartile of all non-NODATA contributing pixels)
/// 'GRA_Q3': Q3 (selects third quartile of all non-NODATA contributing pixels)
pub fn reproject_resampling(source: &Dataset, destiny: &Dataset, resampling: GDALResampleAlg::Type) -> Result<()> {
    let rv = unsafe {
        gdal_sys::GDALReprojectImage(
                source._c_ptr(),
                null(),
                destiny._c_ptr(),
                null(),
                resampling,
                0.0,
                0.0 as c_double,
                None,
                null_mut(),
                null_mut()
            )
    };
    if rv != CPLErr::CE_None {
        Err(_last_cpl_err(rv))?;
    }
    Ok(())
}
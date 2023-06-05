use mmal_sys as ffi;

use std::os::raw::c_uint;
use std::str::FromStr;

pub type ISO = u32;

pub const ISO_AUTO: ISO = 0;
pub const ISO_100: ISO = 100;
pub const ISO_125: ISO = 125;
pub const ISO_160: ISO = 160;
pub const ISO_200: ISO = 200;
pub const ISO_250: ISO = 250;
pub const ISO_320: ISO = 320;
pub const ISO_400: ISO = 400;
pub const ISO_500: ISO = 500;
pub const ISO_640: ISO = 640;
pub const ISO_800: ISO = 800;
pub const ISO_1000: ISO = 1000;
pub const ISO_1250: ISO = 1250;
pub const ISO_1600: ISO = 1600;
pub const ISO_2000: ISO = 2000;
pub const ISO_2500: ISO = 2500;
pub const ISO_3200: ISO = 3200;

/// Settings for the camera.
///
/// ```
/// # use rascam::{CameraError, CameraSettings, SimpleCamera};
/// #
/// # let info = rascam::info().unwrap().cameras[0].clone();
/// # let mut camera = SimpleCamera::new(info.clone()).unwrap();
/// #
/// let settings = CameraSettings{
///     width: info.max_width,
///     height: info.max_height,
///     ..CameraSettings::default()
/// };
/// camera.configure(settings);
/// ```
#[derive(Debug)]
pub struct CameraSettings {
    pub encoding: c_uint,
    pub width: u32,  // 0 = max
    pub height: u32, // 0 = max
    pub iso: ISO,
    pub sensor_mode: u32,
    pub quality: u32, // range 0..100, default to 95
    pub zero_copy: bool,
    /// `use_encoder` will go away
    pub use_encoder: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            encoding: ffi::MMAL_ENCODING_JPEG,
            width: 0,
            height: 0,
            iso: ISO_AUTO,
            sensor_mode: 0,
            quality: 95,
            zero_copy: false,
            use_encoder: true,
        }
    }
}

#[non_exhaustive]
pub enum AWBMode {
    OFF,
    AUTO,
    SUNLIGHT,
    CLOUDY,
    SHADE,
    TUNGSTEN,
    FLUORESCENT,
    INCANDESCENT,
    FLASH,
    HORIZON,
    GREYWORLD,
    MAX,
}

impl AWBMode {
    pub fn value(self) -> ffi::MMAL_PARAM_AWBMODE_T {
        match self {
            Self::OFF => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_OFF,
            Self::AUTO => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_AUTO,
            Self::SUNLIGHT => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_SUNLIGHT,
            Self::CLOUDY => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_CLOUDY,
            Self::SHADE => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_SHADE,
            Self::TUNGSTEN => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_TUNGSTEN,
            Self::FLUORESCENT => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_FLUORESCENT,
            Self::INCANDESCENT => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_INCANDESCENT,
            Self::FLASH => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_FLASH,
            Self::HORIZON => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_HORIZON,
            Self::GREYWORLD => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_GREYWORLD,
            Self::MAX => ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_MAX,
        }
    }
}

impl FromStr for AWBMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "OFF" => Ok(Self::OFF),
            "AUTO" => Ok(Self::AUTO),
            "SUNLIGHT" => Ok(Self::SUNLIGHT),
            "CLOUDY" => Ok(Self::CLOUDY),
            "SHADE" => Ok(Self::SHADE),
            "TUNGSTEN" => Ok(Self::TUNGSTEN),
            "FLUORESCENT" => Ok(Self::FLUORESCENT),
            "INCANDESCENT" => Ok(Self::INCANDESCENT),
            "FLASH" => Ok(Self::FLASH),
            "HORIZON" => Ok(Self::HORIZON),
            "GREYWORLD" => Ok(Self::GREYWORLD),
            "MAX" => Ok(Self::MAX),
            _ => Err(()),
        }
    }
}

//! # Encoder
//!
//! An encoder is a bridge between a CRTC and a connector that takes the pixel
//! data of the CRTC and encodes it into a format the connector understands.

use control::crtc::Handle as CrtcHandle;
use ffi;

/// A handle to an encoder
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Handle(u32);

impl From<u32> for Handle {
    fn from(raw: u32) -> Self {
        Handle(raw)
    }
}

impl Into<u32> for Handle {
    fn into(self) -> u32 {
        self.0
    }
}

/// Information about an encoder
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Info {
    pub(crate) handle: Handle,
    pub(crate) enc_type: Kind,
    pub(crate) crtc: Option<CrtcHandle>,
    pub(crate) pos_crtcs: u32,
    pub(crate) pos_clones: u32,
}

impl Info {
    /// The encoder's handle
    pub fn handle(&self) -> Handle {
        self.handle
    }

    /// The encoder's type
    pub fn kind(&self) -> Kind {
        self.enc_type
    }

    /// Gets the handle of the CRTC currently used if it exists
    pub fn crtc(&self) -> Option<CrtcHandle> {
        self.crtc
    }

    /// Returns a filter for the possible CRTCs that can use this encoder
    pub fn possible_crtcs(&self) -> () {
        unimplemented!()
    }

    /// Returns a filter for the possible encoders that clones this one
    pub fn possible_clones(&self) -> () {
        unimplemented!()
    }
}

/// The type of encoder.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Kind {
    None,
    DAC,
    TMDS,
    LVDS,
    TVDAC,
    Virtual,
    DSI,
    DPMST,
    DPI,
}

impl From<u32> for Kind {
    fn from(n: u32) -> Self {
        match n {
            ffi::DRM_MODE_ENCODER_NONE => Kind::None,
            ffi::DRM_MODE_ENCODER_DAC => Kind::DAC,
            ffi::DRM_MODE_ENCODER_TMDS => Kind::TMDS,
            ffi::DRM_MODE_ENCODER_LVDS => Kind::LVDS,
            ffi::DRM_MODE_ENCODER_TVDAC => Kind::TVDAC,
            ffi::DRM_MODE_ENCODER_VIRTUAL => Kind::Virtual,
            ffi::DRM_MODE_ENCODER_DSI => Kind::DSI,
            ffi::DRM_MODE_ENCODER_DPMST => Kind::DPMST,
            ffi::DRM_MODE_ENCODER_DPI => Kind::DPI,
            _ => Kind::None,
        }
    }
}

impl Into<u32> for Kind {
    fn into(self) -> u32 {
        match self {
            Kind::None => ffi::DRM_MODE_ENCODER_NONE,
            Kind::DAC => ffi::DRM_MODE_ENCODER_DAC,
            Kind::TMDS => ffi::DRM_MODE_ENCODER_TMDS,
            Kind::LVDS => ffi::DRM_MODE_ENCODER_LVDS,
            Kind::TVDAC => ffi::DRM_MODE_ENCODER_TVDAC,
            Kind::Virtual => ffi::DRM_MODE_ENCODER_VIRTUAL,
            Kind::DSI => ffi::DRM_MODE_ENCODER_DSI,
            Kind::DPMST => ffi::DRM_MODE_ENCODER_DPMST,
            Kind::DPI => ffi::DRM_MODE_ENCODER_DPI,
        }
    }
}

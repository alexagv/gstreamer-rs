// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib_ffi;
use gobject_ffi;

use gst;
use gst::miniobject::MiniObject;
use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, FromGlib, FromGlibPtrNone,
                      ToGlib, ToGlibPtr, ToGlibPtrMut};

use std::mem;
use std::ptr;
use std::str;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoColorRange {
    Unknown,
    Range0255,
    Range16235,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoColorRange {
    type GlibType = ffi::GstVideoColorRange;

    fn to_glib(&self) -> ffi::GstVideoColorRange {
        match *self {
            VideoColorRange::Unknown => ffi::GST_VIDEO_COLOR_RANGE_UNKNOWN,
            VideoColorRange::Range0255 => ffi::GST_VIDEO_COLOR_RANGE_0_255,
            VideoColorRange::Range16235 => ffi::GST_VIDEO_COLOR_RANGE_16_235,
            VideoColorRange::__Unknown(value) => unsafe { mem::transmute(value) },
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoColorRange> for VideoColorRange {
    fn from_glib(value: ffi::GstVideoColorRange) -> Self {
        skip_assert_initialized!();
        match value as i32 {
            0 => VideoColorRange::Unknown,
            1 => VideoColorRange::Range0255,
            2 => VideoColorRange::Range16235,
            value => VideoColorRange::__Unknown(value),
        }
    }
}

impl glib::StaticType for VideoColorRange {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_color_range_get_type()) }
    }
}

impl<'a> glib::value::FromValueOptional<'a> for VideoColorRange {
    unsafe fn from_value_optional(value: &glib::value::Value) -> Option<Self> {
        Some(glib::value::FromValue::from_value(value))
    }
}

impl<'a> glib::value::FromValue<'a> for VideoColorRange {
    unsafe fn from_value(value: &glib::value::Value) -> Self {
        from_glib(mem::transmute::<i32, ffi::GstVideoColorRange>(
            gobject_ffi::g_value_get_enum(value.to_glib_none().0),
        ))
    }
}

impl glib::value::SetValue for VideoColorRange {
    unsafe fn set_value(value: &mut glib::value::Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib() as i32)
    }
}

pub struct VideoColorimetry(ffi::GstVideoColorimetry);

impl VideoColorimetry {
    pub fn new(
        range: VideoColorRange,
        matrix: ::VideoColorMatrix,
        transfer: ::VideoTransferFunction,
        primaries: ::VideoColorPrimaries,
    ) -> Self {

        let colorimetry = unsafe {
            let mut colorimetry: ffi::GstVideoColorimetry = mem::zeroed();

            colorimetry.range = range.to_glib();
            colorimetry.matrix = matrix.to_glib();
            colorimetry.transfer = transfer.to_glib();
            colorimetry.primaries = primaries.to_glib();

            colorimetry
        };

        VideoColorimetry(colorimetry)
    }

    pub fn to_string(&self) -> String {
        unsafe { from_glib_full(ffi::gst_video_colorimetry_to_string(&self.0)) }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        unsafe {
            let mut colorimetry = mem::zeroed();
            let valid: bool = from_glib(ffi::gst_video_colorimetry_from_string(
                &mut colorimetry,
                s.to_glib_none().0,
            ));
            if valid {
                Some(VideoColorimetry(colorimetry))
            } else {
                None
            }
        }
    }
}

impl Clone for VideoColorimetry {
    fn clone(&self) -> Self {
        unsafe { VideoColorimetry(ptr::read(&self.0)) }
    }
}

impl PartialEq for VideoColorimetry {
    fn eq(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_video_colorimetry_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for VideoColorimetry {}

impl str::FromStr for ::VideoColorimetry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Self::from_string(s).ok_or(())
    }
}

impl fmt::Debug for ::VideoColorimetry {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.to_string())
    }
}

impl fmt::Display for ::VideoColorimetry {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.to_string())
    }
}

pub struct VideoInfo(ffi::GstVideoInfo);

pub struct VideoInfoBuilder<'a> {
    format: ::VideoFormat,
    width: u32,
    height: u32,
    interlace_mode: Option<::VideoInterlaceMode>,
    flags: Option<::VideoFlags>,
    size: Option<usize>,
    views: Option<u32>,
    chroma_site: Option<::VideoChromaSite>,
    colorimetry: Option<&'a ::VideoColorimetry>,
    par: Option<gst::Fraction>,
    fps: Option<gst::Fraction>,
    offset: Option<&'a [usize]>,
    stride: Option<&'a [i32]>,
    multiview_mode: Option<::VideoMultiviewMode>,
    multiview_flags: Option<::VideoMultiviewFlags>,
    #[cfg(feature = "v1_12")]
    field_order: Option<::VideoFieldOrder>,
}

impl<'a> VideoInfoBuilder<'a> {
    pub fn build(self) -> Option<VideoInfo> {
        unsafe {
            let mut info = mem::uninitialized();

            ffi::gst_video_info_set_format(
                &mut info,
                self.format.to_glib(),
                self.width,
                self.height,
            );

            if info.finfo.is_null() || info.width <= 0 || info.width <= 0 {
                return None;
            }

            if let Some(interlace_mode) = self.interlace_mode {
                info.interlace_mode = interlace_mode.to_glib();
            }

            if let Some(flags) = self.flags {
                info.flags = flags.to_glib();
            }

            if let Some(size) = self.size {
                info.size = size;
            }

            if let Some(views) = self.views {
                info.views = views as i32;
            }

            if let Some(chroma_site) = self.chroma_site {
                info.chroma_site = chroma_site.to_glib();
            }

            if let Some(colorimetry) = self.colorimetry {
                ptr::write(&mut info.colorimetry, ptr::read(&colorimetry.0));
            }

            if let Some(par) = self.par {
                info.par_n = *par.numer();
                info.par_d = *par.denom();
            }

            if let Some(fps) = self.fps {
                info.fps_n = *fps.numer();
                info.fps_d = *fps.denom();
            }

            if let Some(offset) = self.offset {
                if offset.len() != ((*info.finfo).n_planes as usize) {
                    return None;
                }

                for i in 0..((*info.finfo).n_planes as usize) {
                    info.offset[i] = offset[i];
                }
            }

            if let Some(stride) = self.stride {
                if stride.len() != ((*info.finfo).n_planes as usize) {
                    return None;
                }

                for i in 0..((*info.finfo).n_planes as usize) {
                    info.stride[i] = stride[i];
                }
            }

            if let Some(multiview_mode) = self.multiview_mode {
                let ptr = &mut info._gst_reserved as *mut _ as *mut i32;
                ptr::write(ptr.offset(0), mem::transmute(multiview_mode.to_glib()));
            }

            if let Some(multiview_flags) = self.multiview_flags {
                let ptr = &mut info._gst_reserved as *mut _ as *mut u32;
                ptr::write(ptr.offset(1), multiview_flags.to_glib().bits());
            }

            #[cfg(feature = "v1_12")]
            {
                if let Some(field_order) = self.field_order {
                    let ptr = &mut info._gst_reserved as *mut _ as *mut i32;
                    ptr::write(ptr.offset(2), mem::transmute(field_order.to_glib()));
                }
            }

            Some(VideoInfo(info))
        }
    }

    pub fn interlace_mode(self, interlace_mode: ::VideoInterlaceMode) -> VideoInfoBuilder<'a> {
        Self {
            interlace_mode: Some(interlace_mode),
            ..self
        }
    }

    pub fn flags(self, flags: ::VideoFlags) -> Self {
        Self {
            flags: Some(flags),
            ..self
        }
    }

    pub fn size(self, size: usize) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub fn views(self, views: u32) -> Self {
        Self {
            views: Some(views),
            ..self
        }
    }

    pub fn chroma_site(self, chroma_site: ::VideoChromaSite) -> Self {
        Self {
            chroma_site: Some(chroma_site),
            ..self
        }
    }

    pub fn colorimetry(self, colorimetry: &'a ::VideoColorimetry) -> VideoInfoBuilder<'a> {
        Self {
            colorimetry: Some(colorimetry),
            ..self
        }
    }

    pub fn par(self, par: gst::Fraction) -> Self {
        Self {
            par: Some(par),
            ..self
        }
    }

    pub fn fps(self, fps: gst::Fraction) -> Self {
        Self {
            fps: Some(fps),
            ..self
        }
    }

    pub fn offset(self, offset: &'a [usize]) -> VideoInfoBuilder<'a> {
        Self {
            offset: Some(offset),
            ..self
        }
    }

    pub fn stride(self, stride: &'a [i32]) -> VideoInfoBuilder<'a> {
        Self {
            stride: Some(stride),
            ..self
        }
    }

    pub fn multiview_mode(self, multiview_mode: ::VideoMultiviewMode) -> Self {
        Self {
            multiview_mode: Some(multiview_mode),
            ..self
        }
    }

    pub fn multiview_flags(self, multiview_flags: ::VideoMultiviewFlags) -> Self {
        Self {
            multiview_flags: Some(multiview_flags),
            ..self
        }
    }

    #[cfg(feature = "v1_12")]
    pub fn field_order(self, field_order: ::VideoFieldOrder) -> Self {
        Self {
            field_order: Some(field_order),
            ..self
        }
    }
}

impl VideoInfo {
    pub fn new<'a>(format: ::VideoFormat, width: u32, height: u32) -> VideoInfoBuilder<'a> {
        #[cfg(not(feature = "v1_12"))]
        {
            VideoInfoBuilder {
                format: format,
                width: width,
                height: height,
                interlace_mode: None,
                flags: None,
                size: None,
                views: None,
                chroma_site: None,
                colorimetry: None,
                par: None,
                fps: None,
                offset: None,
                stride: None,
                multiview_mode: None,
                multiview_flags: None,
            }
        }
        #[cfg(feature = "v1_12")]
        {
            VideoInfoBuilder {
                format: format,
                width: width,
                height: height,
                interlace_mode: None,
                flags: None,
                size: None,
                views: None,
                chroma_site: None,
                colorimetry: None,
                par: None,
                fps: None,
                offset: None,
                stride: None,
                multiview_mode: None,
                multiview_flags: None,
                field_order: None,
            }
        }
    }

    pub fn from_caps(caps: &gst::Caps) -> Option<Self> {
        unsafe {
            let mut info = mem::uninitialized();
            if from_glib(ffi::gst_video_info_from_caps(&mut info, caps.as_ptr())) {
                Some(VideoInfo(info))
            } else {
                None
            }
        }
    }

    pub fn to_caps(&self) -> Option<gst::Caps> {
        unsafe {
            let caps = ffi::gst_video_info_to_caps(&self.0 as *const _ as *mut _);
            if caps.is_null() {
                None
            } else {
                Some(from_glib_full(caps))
            }
        }
    }

    pub fn format(&self) -> ::VideoFormat {
        unsafe { from_glib((*self.0.finfo).format) }
    }

    pub fn format_info(&self) -> ::VideoFormatInfo {
        ::VideoFormatInfo::from_format(self.format())
    }

    pub fn width(&self) -> u32 {
        self.0.width as u32
    }

    pub fn height(&self) -> u32 {
        self.0.height as u32
    }

    pub fn interlace_mode(&self) -> ::VideoInterlaceMode {
        from_glib(self.0.interlace_mode)
    }

    pub fn flags(&self) -> ::VideoFlags {
        from_glib(self.0.flags)
    }

    pub fn size(&self) -> usize {
        self.0.size
    }

    pub fn views(&self) -> u32 {
        self.0.views as u32
    }

    pub fn chroma_site(&self) -> ::VideoChromaSite {
        from_glib(self.0.chroma_site)
    }

    pub fn colorimetry(&self) -> VideoColorimetry {
        unsafe { VideoColorimetry(ptr::read(&self.0.colorimetry)) }
    }

    pub fn par(&self) -> gst::Fraction {
        gst::Fraction::new(self.0.par_n, self.0.par_d)
    }

    pub fn fps(&self) -> gst::Fraction {
        gst::Fraction::new(self.0.fps_n, self.0.fps_d)
    }

    pub fn offset(&self) -> &[usize] {
        &self.0.offset[0..(self.format_info().n_planes() as usize)]
    }

    pub fn stride(&self) -> &[i32] {
        &self.0.stride[0..(self.format_info().n_planes() as usize)]
    }

    pub fn multiview_mode(&self) -> ::VideoMultiviewMode {
        unsafe {
            let ptr = &self.0._gst_reserved as *const _ as *const i32;
            from_glib(mem::transmute(ptr::read(ptr.offset(0))))
        }
    }

    pub fn multiview_flags(&self) -> ::VideoMultiviewFlags {
        unsafe {
            let ptr = &self.0._gst_reserved as *const _ as *const u32;
            from_glib(mem::transmute(ptr::read(ptr.offset(1))))
        }
    }

    #[cfg(feature = "v1_12")]
    pub fn field_order(&self) -> ::VideoFieldOrder {
        unsafe {
            let ptr = &self.0._gst_reserved as *const _ as *const i32;
            from_glib(mem::transmute(ptr::read(ptr.offset(2))))
        }
    }
}

impl Clone for VideoInfo {
    fn clone(&self) -> Self {
        unsafe { VideoInfo(ptr::read(&self.0)) }
    }
}

impl PartialEq for VideoInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_video_info_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for VideoInfo {}

impl glib::types::StaticType for VideoInfo {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_video_info_get_type()) }
    }
}

#[doc(hidden)]
impl<'a> glib::value::FromValueOptional<'a> for VideoInfo {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<VideoInfo>::from_glib_none(
            gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstVideoInfo,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValue for VideoInfo {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstVideoInfo>::to_glib_none(this).0 as
                glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::value::SetValueOptional for VideoInfo {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstVideoInfo>::to_glib_none(&this).0 as
                glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl glib::translate::Uninitialized for VideoInfo {
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for VideoInfo {
    type GlibType = *mut ffi::GstVideoInfo;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstVideoInfo> for VideoInfo {
    type Storage = &'a VideoInfo;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstVideoInfo, Self> {
        glib::translate::Stash(&self.0, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstVideoInfo {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstVideoInfo> for VideoInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstVideoInfo) -> Self {
        VideoInfo(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstVideoInfo> for VideoInfo {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstVideoInfo) -> Self {
        let info = from_glib_none(ptr);
        glib_ffi::g_free(ptr as *mut _);
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst;

    #[test]
    fn test_new() {
        gst::init().unwrap();

        let info = VideoInfo::new(::VideoFormat::I420, 320, 240)
            .build()
            .unwrap();
        assert_eq!(info.format(), ::VideoFormat::I420);
        assert_eq!(info.width(), 320);
        assert_eq!(info.height(), 240);
        assert_eq!(info.size(), 320 * 240 + 2 * 160 * 120);
        assert_eq!(info.multiview_mode(), ::VideoMultiviewMode::None);
        assert_eq!(&info.offset(), &[0, 320 * 240, 320 * 240 + 160 * 120]);
        assert_eq!(&info.stride(), &[320, 160, 160]);

        let offsets = [0, 640 * 240 + 16, 640 * 240 + 16 + 320 * 120 + 16];
        let strides = [640, 320, 320];
        let info = VideoInfo::new(::VideoFormat::I420, 320, 240)
            .offset(&offsets)
            .stride(&strides)
            .size(640 * 240 + 16 + 320 * 120 + 16 + 320 * 120 + 16)
            .multiview_mode(::VideoMultiviewMode::SideBySide)
            .build()
            .unwrap();
        assert_eq!(info.format(), ::VideoFormat::I420);
        assert_eq!(info.width(), 320);
        assert_eq!(info.height(), 240);
        assert_eq!(
            info.size(),
            640 * 240 + 16 + 320 * 120 + 16 + 320 * 120 + 16
        );
        assert_eq!(info.multiview_mode(), ::VideoMultiviewMode::SideBySide);
        assert_eq!(
            &info.offset(),
            &[0, 640 * 240 + 16, 640 * 240 + 16 + 320 * 120 + 16]
        );
        assert_eq!(&info.stride(), &[640, 320, 320]);
    }

    #[test]
    fn test_from_to_caps() {
        gst::init().unwrap();

        let caps = gst::Caps::new_simple(
            "video/x-raw",
            &[
                ("format", &"I420"),
                ("width", &320),
                ("height", &240),
                ("framerate", &gst::Fraction::new(30, 1)),
                ("pixel-aspect-ratio", &gst::Fraction::new(1, 1)),
                ("interlace-mode", &"progressive"),
                ("chroma-site", &"mpeg2"),
                ("colorimetry", &"bt709"),
            ],
        );
        let info = VideoInfo::from_caps(&caps).unwrap();
        assert_eq!(info.format(), ::VideoFormat::I420);
        assert_eq!(info.width(), 320);
        assert_eq!(info.height(), 240);
        assert_eq!(info.fps(), gst::Fraction::new(30, 1));
        assert_eq!(info.interlace_mode(), ::VideoInterlaceMode::Progressive);
        assert_eq!(info.chroma_site(), ::VIDEO_CHROMA_SITE_MPEG2);
        assert_eq!(info.colorimetry(), "bt709".parse().unwrap());

        let caps2 = info.to_caps().unwrap();
        assert_eq!(caps, caps2);

        let info2 = VideoInfo::from_caps(&caps2).unwrap();
        assert!(info == info2);
    }
}
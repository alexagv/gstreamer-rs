// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct VideoOverlay(Object<ffi::GstVideoOverlay, ffi::GstVideoOverlayInterface>);

    match fn {
        get_type => || ffi::gst_video_overlay_get_type(),
    }
}

impl VideoOverlay {
    //pub fn install_properties(oclass: /*Ignored*/&mut glib::ObjectClass, last_prop_id: i32) {
    //    unsafe { TODO: call ffi::gst_video_overlay_install_properties() }
    //}

    //pub fn set_property<P: IsA<glib::Object>>(object: &P, last_prop_id: i32, property_id: u32, value: /*Ignored*/&glib::Value) -> bool {
    //    unsafe { TODO: call ffi::gst_video_overlay_set_property() }
    //}
}

unsafe impl Send for VideoOverlay {}
unsafe impl Sync for VideoOverlay {}

pub trait VideoOverlayExt {
    fn expose(&self);

    //fn got_window_handle(&self, handle: /*Unimplemented*/Fundamental: UIntPtr);

    fn handle_events(&self, handle_events: bool);

    fn prepare_window_handle(&self);

    fn set_render_rectangle(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), glib::error::BoolError>;

    //fn set_window_handle(&self, handle: /*Unimplemented*/Fundamental: UIntPtr);
}

impl<O: IsA<VideoOverlay>> VideoOverlayExt for O {
    fn expose(&self) {
        unsafe {
            ffi::gst_video_overlay_expose(self.to_glib_none().0);
        }
    }

    //fn got_window_handle(&self, handle: /*Unimplemented*/Fundamental: UIntPtr) {
    //    unsafe { TODO: call ffi::gst_video_overlay_got_window_handle() }
    //}

    fn handle_events(&self, handle_events: bool) {
        unsafe {
            ffi::gst_video_overlay_handle_events(self.to_glib_none().0, handle_events.to_glib());
        }
    }

    fn prepare_window_handle(&self) {
        unsafe {
            ffi::gst_video_overlay_prepare_window_handle(self.to_glib_none().0);
        }
    }

    fn set_render_rectangle(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_video_overlay_set_render_rectangle(self.to_glib_none().0, x, y, width, height), "Failed to set render rectangle")
        }
    }

    //fn set_window_handle(&self, handle: /*Unimplemented*/Fundamental: UIntPtr) {
    //    unsafe { TODO: call ffi::gst_video_overlay_set_window_handle() }
    //}
}

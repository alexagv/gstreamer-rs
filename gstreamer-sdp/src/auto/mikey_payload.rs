// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use MIKEYCacheType;
use MIKEYEncAlg;
use MIKEYKeyDataType;
use MIKEYMacAlg;
use MIKEYPayloadType;
use MIKEYSecProto;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MIKEYPayload(Boxed<ffi::GstMIKEYPayload>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::gst_mikey_payload_get_type(), ptr as *mut _) as *mut ffi::GstMIKEYPayload,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::gst_mikey_payload_get_type(), ptr as *mut _),
        get_type => || ffi::gst_mikey_payload_get_type(),
    }
}

impl MIKEYPayload {
    pub fn new(type_: MIKEYPayloadType) -> MIKEYPayload {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_mikey_payload_new(type_.to_glib()))
        }
    }

    pub fn kemac_get_n_sub(&self) -> u32 {
        unsafe {
            ffi::gst_mikey_payload_kemac_get_n_sub(self.to_glib_none().0)
        }
    }

    pub fn kemac_remove_sub(&mut self, idx: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_payload_kemac_remove_sub(self.to_glib_none_mut().0, idx))
        }
    }

    pub fn kemac_set(&mut self, enc_alg: MIKEYEncAlg, mac_alg: MIKEYMacAlg) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_payload_kemac_set(self.to_glib_none_mut().0, enc_alg.to_glib(), mac_alg.to_glib()))
        }
    }

    pub fn key_data_set_key(&mut self, key_type: MIKEYKeyDataType, key_data: &[u8]) -> bool {
        let key_len = key_data.len() as u16;
        unsafe {
            from_glib(ffi::gst_mikey_payload_key_data_set_key(self.to_glib_none_mut().0, key_type.to_glib(), key_len, key_data.to_glib_none().0))
        }
    }

    pub fn key_data_set_salt(&mut self, salt_data: &[u8]) -> bool {
        let salt_len = salt_data.len() as u16;
        unsafe {
            from_glib(ffi::gst_mikey_payload_key_data_set_salt(self.to_glib_none_mut().0, salt_len, salt_data.to_glib_none().0))
        }
    }

    pub fn key_data_set_spi(&mut self, spi_data: &[u8]) -> bool {
        let spi_len = spi_data.len() as u8;
        unsafe {
            from_glib(ffi::gst_mikey_payload_key_data_set_spi(self.to_glib_none_mut().0, spi_len, spi_data.to_glib_none().0))
        }
    }

    pub fn pke_set(&mut self, C: MIKEYCacheType, data: &[u8]) -> bool {
        let data_len = data.len() as u16;
        unsafe {
            from_glib(ffi::gst_mikey_payload_pke_set(self.to_glib_none_mut().0, C.to_glib(), data_len, data.to_glib_none().0))
        }
    }

    pub fn rand_set(&mut self, rand: &[u8]) -> bool {
        let len = rand.len() as u8;
        unsafe {
            from_glib(ffi::gst_mikey_payload_rand_set(self.to_glib_none_mut().0, len, rand.to_glib_none().0))
        }
    }

    pub fn sp_add_param(&mut self, type_: u8, val: &[u8]) -> bool {
        let len = val.len() as u8;
        unsafe {
            from_glib(ffi::gst_mikey_payload_sp_add_param(self.to_glib_none_mut().0, type_, len, val.to_glib_none().0))
        }
    }

    pub fn sp_get_n_params(&self) -> u32 {
        unsafe {
            ffi::gst_mikey_payload_sp_get_n_params(self.to_glib_none().0)
        }
    }

    //pub fn sp_get_param(&self, idx: u32) -> /*Ignored*/Option<MIKEYPayloadSPParam> {
    //    unsafe { TODO: call ffi::gst_mikey_payload_sp_get_param() }
    //}

    pub fn sp_remove_param(&mut self, idx: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_payload_sp_remove_param(self.to_glib_none_mut().0, idx))
        }
    }

    pub fn sp_set(&mut self, policy: u32, proto: MIKEYSecProto) -> bool {
        unsafe {
            from_glib(ffi::gst_mikey_payload_sp_set(self.to_glib_none_mut().0, policy, proto.to_glib()))
        }
    }

    //pub fn t_set(&mut self, type_: MIKEYTSType, ts_value: &[u8]) -> bool {
    //    unsafe { TODO: call ffi::gst_mikey_payload_t_set() }
    //}
}

unsafe impl Send for MIKEYPayload {}

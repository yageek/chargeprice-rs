use std::ffi::CString;

use libc::c_char;
use log::trace;

use chargeprice::api::{Entity, VehiculeAttributes, VehiculeRelationships};

#[repr(C)]
pub struct Vehicule {
    pub id: *const c_char,
    pub brand: *const c_char,
    pub manufacturer_id: *const c_char,
}

impl Drop for Vehicule {
    fn drop(&mut self) {
        unsafe {
            let id = CString::from_raw(self.id as *mut c_char);
            let _ = CString::from_raw(self.brand as *mut c_char);
            let _ = CString::from_raw(self.manufacturer_id as *mut c_char);
            trace!("Releasing Vehicule {:?}", id);
        }
    }
}
impl From<&Entity<VehiculeAttributes, VehiculeRelationships>> for Vehicule {
    fn from(other: &Entity<VehiculeAttributes, VehiculeRelationships>) -> Self {
        Vehicule {
            id: CString::new(other.id()).unwrap().into_raw(),
            brand: CString::new(other.attributes().brand()).unwrap().into_raw(),
            manufacturer_id: CString::new(other.relationships().unwrap().manufacturer_id())
                .unwrap()
                .into_raw(),
        }
    }
}

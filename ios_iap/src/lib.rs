// TODO: probably better behind a feature
pub mod callbacks;

use std::{
    ffi::{c_char, CString},
    os::raw::c_void,
};

pub use objc_foundation::{INSArray, INSString, NSArray, NSObject, NSString};
pub use objc_id::{Id, ShareId};

extern "C" {
    pub fn init_callbacks(
        restore_finished: extern "C" fn(),
        // TODO: handle restore fail
        fetch_products_success: extern "C" fn(*mut c_void),
        fetch_products_failed: extern "C" fn(),
        purchase_success: extern "C" fn(NSString),
        purchase_failed: extern "C" fn(NSString),
    );
    pub fn restore_purchases();
    pub fn fetch_products(identifiers: ShareId<NSArray<NSString>>);
    pub fn get_product_identifier_raw(product: *mut c_void) -> *mut c_char;
    pub fn get_product_from_array(products: *mut c_void, index: u32) -> *mut c_void;
    pub fn purchase(product: *mut c_void);
}

pub unsafe fn get_product_identifier(product: *mut c_void) -> String {
    dbg!(&product);
    CString::from_raw(get_product_identifier_raw(product))
        .into_string()
        .unwrap()
}

pub fn fetch_products_for_identifiers(identifiers: Vec<String>) {
    let identifier: Id<NSString> = INSString::from_str(identifiers.first().unwrap());

    let objs = vec![identifier];
    let objc_identifiers = NSArray::from_vec(objs).share();
    unsafe { fetch_products(objc_identifiers) }
}

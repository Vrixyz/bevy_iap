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
    pub fn purchase_raw(product: ShareId<NSString>);
    pub fn can_purchase_raw(product: ShareId<NSString>) -> bool;
    pub fn get_price_localized_raw(product: ShareId<NSString>) -> Id<NSString>;

}

pub fn purchase(product_identifier: &str) {
    let identifier: Id<NSString> = INSString::from_str(product_identifier);
    unsafe {
        purchase_raw(identifier.share());
    }
}
pub fn can_purchase(product_identifier: &str) -> bool {
    let identifier: Id<NSString> = INSString::from_str(product_identifier);
    unsafe { can_purchase_raw(identifier.share()) }
}

pub fn get_price_localized(product_identifier: &str) -> String {
    let identifier: Id<NSString> = INSString::from_str(product_identifier);
    unsafe {
        get_price_localized_raw(identifier.share())
            .as_str()
            .to_string()
    }
}

pub fn fetch_products_for_identifiers(identifiers: Vec<String>) {
    let identifier: Id<NSString> = INSString::from_str(identifiers.first().unwrap());

    let objs = vec![identifier];
    let objc_identifiers = NSArray::from_vec(objs).share();
    unsafe { fetch_products(objc_identifiers) }
}

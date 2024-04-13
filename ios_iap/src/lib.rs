use objc_foundation::{INSArray, INSString, NSArray};
use objc_id::ShareId;

pub use objc_foundation::{NSObject, NSString};
pub use objc_id::Id;

extern "C" {
    pub fn init_callbacks(
        restore_finished: extern "C" fn(),
        // TODO: handle restore fail
        fetch_products_success: extern "C" fn(Id<NSObject>),
        fetch_products_failed: extern "C" fn(),
        purchase_success: extern "C" fn(Id<NSString>),
        purchase_failed: extern "C" fn(Id<NSString>),
    );
    pub fn restore_purchases();
    pub fn fetch_products(identifiers: ShareId<NSArray<NSString>>);
    pub fn purchase(product: Id<NSObject>);
}

pub fn fetch_products_for_identifiers(identifiers: Vec<String>) {
    let identifier: Id<NSString> = INSString::from_str(identifiers.first().unwrap());

    let objs = vec![identifier];
    let objc_identifiers = NSArray::from_vec(objs).share();
    unsafe { fetch_products(objc_identifiers) }
}

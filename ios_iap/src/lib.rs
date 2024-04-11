use objc_foundation::{INSArray, INSString, NSArray, NSString};
use objc_id::{Id, ShareId};

extern "C" {
    pub fn _fetch_products_for_identifiers(
        identifiers: ShareId<NSArray<NSString>>,
        callback: extern "C" fn(),
    );
}

#[no_mangle]
extern "C" fn callback_c() {
    dbg!("I know the ad was done for from Rust");
}

pub fn fetch_products_for_identifiers(identifiers: Vec<String>) {
    let identifier: Id<NSString> = INSString::from_str(identifiers.first().unwrap());

    let objs = vec![identifier];
    let objc_identifiers = NSArray::from_vec(objs).share();
    unsafe { _fetch_products_for_identifiers(objc_identifiers, callback_c) }
}

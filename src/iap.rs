use ios_iap::{Id, NSObject, NSString};

#[no_mangle]
extern "C" fn restore_finished() {
    dbg!("restore_finished");
}

#[no_mangle]
extern "C" fn fetch_products_success(products: Id<NSObject>) {
    dbg!("fetch_products_success");
}

#[no_mangle]
extern "C" fn fetch_products_failed() {
    dbg!("fetch_products_failed");
}

#[no_mangle]
extern "C" fn purchase_success(product: Id<NSString>) {
    dbg!("purchase_success");
}

#[no_mangle]
extern "C" fn purchase_failed(product: Id<NSString>) {
    dbg!("purchase_failed");
}

pub fn init_callbacks() {
    unsafe {
        ios_iap::init_callbacks(
            restore_finished,
            fetch_products_success,
            fetch_products_failed,
            purchase_success,
            purchase_failed,
        )
    }
}

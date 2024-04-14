use ios_iap::{INSString, Id, NSArray, NSObject, NSString};
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

static mut restore_finished_sender: Option<Mutex<Sender<()>>> = None;
static mut restore_finished_receiver: Option<Mutex<Receiver<()>>> = None;

#[no_mangle]
extern "C" fn restore_finished() {
    dbg!("restore_finished");
    unsafe {
        let send_result = restore_finished_sender.as_ref().unwrap().lock().unwrap();
        send_result.send(());
    }
}

static mut fetch_products_sender: Option<Mutex<Sender<Result<NSArray<Id<NSObject>>, ()>>>> = None;
static mut fetch_products_receiver: Option<Mutex<Receiver<Result<NSArray<Id<NSObject>>, ()>>>> =
    None;
#[no_mangle]
extern "C" fn fetch_products_success(products: NSArray<Id<NSObject>>) {
    dbg!("fetch_products_success");
    unsafe {
        let send_result = fetch_products_sender.as_ref().unwrap().lock().unwrap();
        send_result.send(Ok(products));
    }
}

#[no_mangle]
extern "C" fn fetch_products_failed() {
    dbg!("fetch_products_failed");
    unsafe {
        let send_result = fetch_products_sender.as_ref().unwrap().lock().unwrap();
        send_result.send(Err(()));
    }
}

static mut purchase_sender: Option<Mutex<Sender<Result<String, String>>>> = None;
static mut purchase_receiver: Option<Mutex<Receiver<Result<String, String>>>> = None;
#[no_mangle]
extern "C" fn purchase_success(product: NSString) {
    let s = product.as_str().to_string();
    dbg!("purchase_success", &s);
    unsafe {
        let send_result = purchase_sender.as_ref().unwrap().lock().unwrap();
        send_result.send(Ok(s));
    }
}
#[no_mangle]
extern "C" fn purchase_failed(product: NSString) {
    let s = product.as_str().to_string();
    dbg!("purchase_failed", &s);
    unsafe {
        let send_result = purchase_sender.as_ref().unwrap().lock().unwrap();
        send_result.send(Err(s));
    }
}

pub fn init_callbacks() {
    unsafe {
        let (tx, rx) = channel();
        restore_finished_sender = Some(Mutex::new(tx));
        restore_finished_receiver = Some(Mutex::new(rx));

        let (tx, rx) = channel();
        fetch_products_sender = Some(Mutex::new(tx));
        fetch_products_receiver = Some(Mutex::new(rx));

        let (tx, rx) = channel();
        purchase_sender = Some(Mutex::new(tx));
        purchase_receiver = Some(Mutex::new(rx));

        ios_iap::init_callbacks(
            restore_finished,
            fetch_products_success,
            fetch_products_failed,
            purchase_success,
            purchase_failed,
        )
    }
}

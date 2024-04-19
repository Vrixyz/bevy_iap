use crate::{INSString, Id, NSArray, NSObject, NSString, ShareId};
use std::{
    any::Any,
    ffi::c_void,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
};

type MutexAction<T, R> = fn(Option<&mut Mutex<Receiver<T>>>) -> R;

/// Useful boilerplate to use with different getters taking [`MutexAction`] as parameter.
///
/// Usage:
/// ```rs
/// if let Some(result) = get_mut_restore_finished_receiver(try_read) {
///     // TODO: use result
/// }
/// ```
/// Alternatively, you can use the underlying mutex to adopt a custom reading:
///
/// Here is a blocking implementation:
/// ```rs
/// if let Some(result) = get_mut_restore_finished_receiver(|receiver| {
///     receiver?.lock().map_or(None, |receiver| receiver.recv().ok())
/// }) {
///   // TODO: use result
/// }
/// ```
pub fn try_read<T>(receiver: Option<&mut Mutex<Receiver<T>>>) -> Option<T> {
    let Ok(lock) = receiver?.try_lock() else {
        return None;
    };
    let Ok(result) = lock.try_recv() else {
        return None;
    };
    Some(result)
}

static mut restore_finished_sender: Option<Mutex<Sender<()>>> = None;
static mut restore_finished_receiver: Option<Mutex<Receiver<()>>> = None;

/// Get the mutex to the receiver for [`crate::restore_purchases`],
/// Given that we already called [`init_callbacks`].
/// Usage (blocking read):
/// ```rs
/// let result = get_mut_restore_finished_receiver(try_read);
/// if result.is_some() {
///     let test = Some(result);
/// }
/// Alternatively, see [`try_read`]
/// ```
pub fn get_mut_restore_finished_receiver<T>(action: MutexAction<(), T>) -> T {
    action(unsafe { restore_finished_receiver.as_mut() })
}
#[no_mangle]
extern "C" fn restore_finished() {
    dbg!("restore_finished");
    unsafe {
        let send_result = restore_finished_sender.as_ref().unwrap().lock().unwrap();
        send_result.send(());
    }
}

static mut fetch_products_sender: Option<Mutex<Sender<Result<(), ()>>>> = None;
static mut fetch_products_receiver: Option<Mutex<Receiver<Result<(), ()>>>> = None;

/// Get the mutex to the receiver for [`crate::fetch_products`] or [`crate::fetch_products_for_identifiers`],
/// Given that we already called [`init_callbacks`].
/// Usage:
/// ```rs
/// let result = get_mut_fetch_products_receiver(try_read);
/// if result.is_some() {
///     TODO: use the result
/// }
/// ```
pub fn get_mut_fetch_products_receiver<T>(action: MutexAction<Result<(), ()>, T>) -> T {
    action(unsafe { fetch_products_receiver.as_mut() })
}

#[no_mangle]
extern "C" fn fetch_products_success(products: *mut c_void) {
    dbg!("fetch_products_success");
    unsafe {
        let send_result = fetch_products_sender.as_ref().unwrap().lock().unwrap();

        dbg!(send_result.send(Ok(())));
    }
    dbg!("sent_result");
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

/// Get the mutex to the receiver for [`crate::purchase`],
/// Given that we already called [`init_callbacks`].
/// Usage:
/// ```rs
/// let result = get_mut_purchase_receiver(try_read);
/// if result.is_some() {
///     TODO: use the result
/// }
/// ```
pub fn get_mut_purchase_receiver<T>(action: MutexAction<Result<String, String>, T>) -> T {
    action(unsafe { purchase_receiver.as_mut() })
}

#[no_mangle]
extern "C" fn purchase_success(product: ShareId<NSString>) {
    let s = product.as_str().to_string();
    dbg!("purchase_success", &s);
    unsafe {
        let send_result = purchase_sender.as_ref().unwrap().lock().unwrap();
        send_result.send(Ok(s));
    }
}
#[no_mangle]
extern "C" fn purchase_failed(product: ShareId<NSString>) {
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

        super::init_callbacks(
            restore_finished,
            fetch_products_success,
            fetch_products_failed,
            purchase_success,
            purchase_failed,
        )
    }
}

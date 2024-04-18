use core::ffi;
use std::{ffi::c_void, panic::catch_unwind, sync::Mutex};

use bevy::{prelude::*, utils::hashbrown::HashMap};
pub use ios_iap::*;

/// Initializes iap callbacks to communicate and adds the systems responsible to send bevy events.
pub struct InAppPurchasePlugin;

impl Plugin for InAppPurchasePlugin {
    fn build(&self, app: &mut App) {
        callbacks::init_callbacks();
        app.add_event::<RestorePurchaseResult>()
            .add_event::<FetchProductsResult>()
            .add_event::<PurchaseResult>()
            .add_systems(
                Update,
                (read_restore_purchases, read_fetch_products, read_purchase),
            );
    }
}

#[derive(Event)]
pub struct RestorePurchaseResult(pub Result<(), ()>);

fn read_restore_purchases(mut event: EventWriter<RestorePurchaseResult>) {
    let result_success = callbacks::get_mut_restore_finished_receiver(callbacks::try_read);
    if result_success.is_some() {
        event.send(RestorePurchaseResult(Ok(())));
    }
}

#[derive(Event)]
pub struct FetchProductsResult(pub Result<Id<NSArray<NSObject>>, ()>);

fn read_fetch_products(mut event: EventWriter<FetchProductsResult>) {
    if let Some(result) = callbacks::get_mut_fetch_products_receiver(callbacks::try_read) {
        if result.is_ok() {
            dbg!("fetch_products success");
        } else {
            dbg!("fetch_products failed");
        }
    }
}

#[derive(Event)]
pub struct PurchaseResult(pub Result<String, String>);

fn read_purchase(mut event: EventWriter<PurchaseResult>) {
    if let Some(result) = callbacks::get_mut_purchase_receiver(callbacks::try_read) {
        event.send(PurchaseResult(result));
    }
}

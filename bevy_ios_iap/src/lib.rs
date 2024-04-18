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

#[derive(Resource)]
pub struct Products {
    pub map: HashMap<String, *mut c_void>,
}

unsafe impl Send for Products {}
unsafe impl Sync for Products {}

impl Products {
    pub fn purchase(&self, product_identifier: &str) {
        let Some(product) = self.map.get(product_identifier) else {
            return;
        };
        unsafe {
            ios_iap::purchase(*product);
        }
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

fn read_fetch_products(mut commands: Commands, mut event: EventWriter<FetchProductsResult>) {
    if let Some(result) = callbacks::get_mut_fetch_products_receiver(callbacks::try_read) {
        dbg!("read_fetch_products");
        if let Ok(products) = result {
            let mut map = HashMap::new();
            //let mut values = NSArray::to_shared_vec(&*products);

            //let mut products = NSArray::into_vec(products);
            dbg!("values retrieved");
            dbg!(products);
            unsafe {
                let product = dbg!(get_product_from_array(products, 0));
                let product_identifier = unsafe { get_product_identifier(product) };
                dbg!(&product_identifier);
                //map.insert(product_identifier, products);
            }

            /*
            while let Some(p) = products.pop() {
                map.insert("test".to_string(), p.share());
            }*/
            dbg!("insert_resource Products");
            commands.insert_resource(Products { map });
        }
        //event.send(FetchProductsResult(result));
    }
}

#[derive(Event)]
pub struct PurchaseResult(pub Result<String, String>);

fn read_purchase(mut event: EventWriter<PurchaseResult>) {
    if let Some(result) = callbacks::get_mut_purchase_receiver(callbacks::try_read) {
        event.send(PurchaseResult(result));
    }
}

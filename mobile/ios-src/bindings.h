#import <UIKit/UIKit.h>

#ifndef bindings_h
#define bindings_h

void main_rs(void);


typedef void (*rust_callback_void)(void);
typedef void (*rust_callback_skproducts)(NSArray<SKProduct *> *);
typedef void (*rust_callback_string)(NSString *);

void init_callbacks(rust_callback_void restore_finished,
                    // TODO: handle restore fail
                    rust_callback_skproducts fetch_products_success,
                    rust_callback_void fetch_products_failed,
                    rust_callback_string purchase_success,
                    rust_callback_string purchase_failed);

void restore_purchases(void);
void fetch_products(NSArray *identifiers);
void purchase(SKProduct *product);

#endif

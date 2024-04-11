#import <UIKit/UIKit.h>

#ifndef bindings_h
#define bindings_h

typedef void (*rust_callback)(void);

void main_rs(void);

void _fetch_products_for_identifiers(NSArray *placement_id, rust_callback);

#endif
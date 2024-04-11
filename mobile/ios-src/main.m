#import <stdio.h>
#import <StoreKit/StoreKit.h>

#import "bindings.h"

@interface MyRequestDelegate : NSObject <SKProductsRequestDelegate>

@end

@implementation MyRequestDelegate

- (void)request:(SKRequest *)request didFailWithError:(NSError *)error {
    NSLog(@"Request failed with error: %@", error.localizedDescription);
    // Implement error handling logic here
}

- (void)productsRequest:(nonnull SKProductsRequest *)request didReceiveResponse:(nonnull SKProductsResponse *)response { 
    NSLog(@"Request finished successfully.");
    NSLog(@"{%@}", response.products);
}

@end

int main() {
    main_rs();
    return 0;
}

SKProductsRequest* request = nil;
id delegate = nil;


void test_callback(id<SKProductsRequestDelegate> _Nullable* product) {
    
}


void validateProductIdentifiers(NSArray *productIdentifiers)
{
    SKProductsRequest *productsRequest = [[SKProductsRequest alloc]
                                          initWithProductIdentifiers:[NSSet setWithArray:productIdentifiers]];
    
    
    // Keep a strong reference to the request.
    request = productsRequest;
    delegate = [MyRequestDelegate new];
    productsRequest.delegate = delegate;
    [productsRequest start];
}

void _fetch_products_for_identifiers(NSArray *identifiers, rust_callback) {
    validateProductIdentifiers(identifiers);
    
}

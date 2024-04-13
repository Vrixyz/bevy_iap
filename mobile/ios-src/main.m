#import <stdio.h>
#import <StoreKit/StoreKit.h>

#import "bindings.h"

rust_callback_void _restore_finished;

rust_callback_skproducts _fetch_products_success;
rust_callback_void _fetch_products_fail;

rust_callback_string _purchase_success;
rust_callback_string _purchase_fail;

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
    _fetch_products_success(response.products);
}

@end

@interface StoreObserver : NSObject <SKPaymentTransactionObserver>

@end

@implementation StoreObserver

- (void)paymentQueue:(nonnull SKPaymentQueue *)queue updatedTransactions:(nonnull NSArray<SKPaymentTransaction *> *)transactions { 
    for (int i = 0; i < transactions.count; i++) {
        if (transactions[i].transactionState == SKPaymentTransactionStatePurchased || transactions[i].transactionState == SKPaymentTransactionStateRestored) {
            _purchase_success(transactions[i].payment.productIdentifier);
        }
        else {
            // TODO: maybe deferred needs special handling ? do nothing (as we expect something else to happen ? Eventually just send the exact state and let rust handle it.
            _purchase_fail(transactions[i].payment.productIdentifier);
        }
    }
}

@end

int main(void) {
    main_rs();
    return 0;
}

SKProductsRequest* request = nil;
id delegate = nil;

void init_callbacks(rust_callback_void restore_finished,
                    rust_callback_skproducts fetch_products_success,
                    rust_callback_void fetch_products_failed,
                    rust_callback_string purchase_success,
                    rust_callback_string purchase_failed) {
    _restore_finished = restore_finished;
    
    _fetch_products_success = fetch_products_success;
    _fetch_products_fail = fetch_products_failed;

    _purchase_success = purchase_success;
    _purchase_fail = purchase_failed;
    
    delegate = [MyRequestDelegate new];
}


void fetch_products(NSArray *productIdentifiers)
{
    SKProductsRequest *productsRequest = [[SKProductsRequest alloc]
                                          initWithProductIdentifiers:[NSSet setWithArray:productIdentifiers]];
    // Keep a strong reference to the request.
    // TODO: fail if there's already a request running
    request = productsRequest;
    productsRequest.delegate = delegate;
    [productsRequest start];
}

void buy_product(SKProduct* product) {
    SKPayment* payment = [SKPayment paymentWithProduct: product];
    [[SKPaymentQueue defaultQueue] addPayment:payment];
}

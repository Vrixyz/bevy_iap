Credits for base template to niklasEi.

This project attempts to integrate iOS in-app purchases to a bevy app.

## Useful references
- https://developer.apple.com/documentation/storekit/in-app_purchase
- https://developer.apple.com/documentation/storekit/purchaseaction
  - `PurchaseAction` or `purchase(options)` ? aka "swiftUI or UIKit" :thinking:
<details>
  <summary>Analysis</summary>

I'll be listing different approaches ranked from best to "less good" according to my opinion:

### 1
Best case scenario is that icrate supports StoreKit, as it's ticked on https://github.com/madsmtm/objc2/issues/393/.

It's the best case scenario because icrate would offer a community hub to collaborate on this project.

Unfortunately, StoreKit integration in icrate appears to not work on iOS, and fixing that needs thorough refactoring: https://github.com/madsmtm/objc2/issues/482.

### 2

Following advice of https://github.com/madsmtm/objc2/issues/482 : we could fork icrate and fix the problem in a "dirty" way.

This approach is fine for a project but not too great for a library. Also, it's still quite complicated from what I tried (icrate locally wasn't compilingm not sure why... )

### 3

leverage objc to call directly storekit API ; doable, but requires good knowledge in rust/objc ffi.

### 4

leverage objc but call a simpler custom objc API: the simplest choice, but requires a bit of manual work, as well as multiple languages used.

Honestly, even though I listed it last, I like its simplicity.
  
</details>

## Current state

Solution 4 (objc + custom objc bridge to storekit) is what I'm implementing, currently not too far but a very basic POC is promising.

Architecture summary:

- ios_iap is a low level crate for engine agnostic in app purchases in iOS.
- bevy_ios_iap is a bevy plugin
- then a "user space implementation to serve as example.

I pass some log filter to avoid WARN spam.

To avoid a complicated data bridge to SKProduct, a dictionary is stored in objective-C realm.
Then a few utility functions are implemented, namely to retrieve their localized pricing. 

in ios_iap, an opinionated communication is provided in `callbacks` mod through channels,
created for each significant API route:
- TODO: recover
- fetching products
- purchasing

### Not code configuration

Follow the steps to configure Apple in-app purchases: https://developer.apple.com/help/app-store-connect/configure-in-app-purchase-settings/overview-for-configuring-in-app-purchases/.

- From my testing, purchases don't work on simulator: they appear to, but success callback is not triggered and the process goes back to the start of purchasing. So **test on device**.
- Make sure you're using a sandbox user or the payment will fail with a comprehensive error message.

## Missing

- recovering purchases
- clippy
- documentation
- handling errors all over the place
- create crates (other repository for ios_iap ?)

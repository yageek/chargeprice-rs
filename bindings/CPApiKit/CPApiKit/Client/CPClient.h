//
//  CPClient.h
//  CPApiKit
//
//  Created by Yannick Heinrich on 09.02.21.
//

@import Foundation;

typedef uint64_t CPClientCancellable;

extern NSString * _Nonnull const CPClientErrorDomain;

@class CPVehicule;
NS_ASSUME_NONNULL_BEGIN

@interface CPClient : NSObject
// Invalid elements
- (instancetype) init NS_UNAVAILABLE;

/// Default initializer
/// @param key The key for the API
- (instancetype)initWithKey:(NSString *) key NS_DESIGNATED_INITIALIZER;

/// Cancel a request
/// @param cancellable The cancellable
- (void)cancel:(CPClientCancellable) cancellable;

/// Fetch the vehicules from the API
/// @param completion The completion block
- (CPClientCancellable)fetchVehicules:(void(^)(NSArray<CPVehicule*> * _Nullable, NSError * _Nullable))completion;
@end

NS_ASSUME_NONNULL_END

//
//  CPClient.m
//  CPApiKit
//
//  Created by Yannick Heinrich on 09.02.21.
//

#import "CPClient.h"
#import "chargeprice-ffi.h"
#import "CPVehicule+Private.h"


NSString * const CPClientErrorDomain = @"_CPClientErrorDomain";

@implementation CPClient {
    CAPI_FFIClient *_client;
}

#pragma mark - Initialisation

+ (void)initialize
{
    if (self == [CPClient class]) {
        chargeprice_init_log();
    }
}

-(instancetype) initWithKey:(NSString *)key {
    self = [super init];
    if (self) {

        const char *cKey = [key cStringUsingEncoding:NSUTF8StringEncoding];


        CAPI_FFIClient *instance = chargeprice_create_api_client(cKey, "FFI - (CPApiKit)");

        if (instance == NULL) {
            return nil;
        }

        _client = instance;
    }
    return self;
}

- (void) dealloc {
    chargeprice_free_api_client(_client);
}

- (void)cancel:(CPClientCancellable) cancellable {
    chargeprice_cancel(_client, cancellable);
}

#pragma mark - Load

void _vehicule_tramp_on_success(void *context, const CAPI_Vehicule *arg, size_t length) {

    NSMutableArray *array = [NSMutableArray arrayWithCapacity:length];

    CAPI_Vehicule *ptr = (CAPI_Vehicule*)arg;
    for (NSUInteger i = 0; i < length; i++) {
        CPVehicule *vehicule = [[CPVehicule alloc] initWithNative:ptr];
        ptr += 1;
        [array addObject:vehicule];
    }
    // Call callback
    void(^cb)(NSArray<CPVehicule*> * _Nullable, NSError * _Nullable) = (void(^)(NSArray<CPVehicule*> * _Nullable, NSError * _Nullable))CFBridgingRelease(context);
    cb(array, nil);
}

void _vehicule_tramp_on_error(void *context, const CAPI_FFIError *error) {

    NSError *co = [NSError errorWithDomain:CPClientErrorDomain code:error->code userInfo:@{NSLocalizedDescriptionKey: [NSString stringWithUTF8String:error->message]}];
    void(^cb)(NSArray<CPVehicule*> * _Nullable, NSError * _Nullable) = (void(^)(NSArray<CPVehicule*> * _Nullable, NSError * _Nullable))CFBridgingRelease(context);
    cb(nil, co);
}

- (CPClientCancellable)fetchVehicules:(void(^)(NSArray<CPVehicule*> * _Nullable, NSError * _Nullable))completion {
    CAPI_ArrayCallback_Vehicule__FFIError cb = {
        .context = (void*)CFBridgingRetain(completion),
        .on_success = _vehicule_tramp_on_success,
        .on_error = _vehicule_tramp_on_error
    };

    return chargeprice_get_vehicles(_client, cb);
}
@end

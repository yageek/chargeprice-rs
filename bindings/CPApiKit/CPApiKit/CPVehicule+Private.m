//
//  CPVehicule+Private.m
//  CPApiKit
//
//  Created by Yannick Heinrich on 09.02.21.
//

#import "CPVehicule+Private.h"

@implementation CPVehicule (Private)

-(instancetype) initWithNative:(CAPI_Vehicule *) native {

    self = [super init];
    if (self) {
        _identifier = [[NSString alloc] initWithUTF8String:native->id];
        _brand = [[NSString alloc] initWithUTF8String:native->brand];
        _manufacturerID = [[NSString alloc] initWithUTF8String:native->manufacturer_id];
    }
    return self;
}
@end

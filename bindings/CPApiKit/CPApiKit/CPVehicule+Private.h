//
//  CPVehicule+Private.h
//  CPApiKit
//
//  Created by Yannick Heinrich on 09.02.21.
//

#import "CPVehicule.h"
#import "chargeprice-ffi.h"

NS_ASSUME_NONNULL_BEGIN

@interface CPVehicule (Private)
-(instancetype) initWithNative:(CAPI_Vehicule *)native;
@end

NS_ASSUME_NONNULL_END

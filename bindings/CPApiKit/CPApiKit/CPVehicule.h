//
//  CPVehicule.h
//  CPApiKit
//
//  Created by Yannick Heinrich on 09.02.21.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface CPVehicule : NSObject {
    NSString *_identifier;
    NSString *_brand;
    NSString *_manufacturerID;
}

@property(nonatomic, copy, nonnull, readonly) NSString *identifier;
@property(nonatomic, copy, nonnull, readonly) NSString *brand;
@property(nonatomic, copy, nonnull, readonly) NSString *manufacturerID;
@end

NS_ASSUME_NONNULL_END

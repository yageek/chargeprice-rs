//
//  CPApiKitTests.m
//  CPApiKitTests
//
//  Created by Yannick Heinrich on 09.02.21.
//

@import XCTest;
@import CPApiKit;

@interface CPApiKitTests : XCTestCase {
    CPClient *client;
}
@end

@implementation CPApiKitTests

- (void)testExample {
    client = [[CPClient alloc] initWithKey:@"***REMOVED***"];
    XCTAssertNotNil(client);

    XCTestExpectation *exp = [self expectationWithDescription:@"Basic call"];

   CPClientCancellable cancellable = [client fetchVehicules:^(NSArray<CPVehicule *> * _Nullable result, NSError * _Nullable error) {
        [exp fulfill];
    }];

//    [client cancel:cancellable];
    
    [self waitForExpectationsWithTimeout:10.0 handler:^(NSError * _Nullable error) {

    }];
}

@end

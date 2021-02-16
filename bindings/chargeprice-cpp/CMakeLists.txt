cmake_minimum_required(VERSION 3.10)

# set the project name
project(ChargepriceApiClient)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# catch
set(CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake)
include(${CMAKE_ROOT}/Modules/ExternalProject.cmake)
include(catch)
add_subdirectory(3rdParty)

# add the executable
add_library(chargepricecpp src/APIClient.cpp src/Vehicule.cpp)
target_include_directories(chargepricecpp PRIVATE ${CMAKE_SOURCE_DIR}/../libchargeprice_ffi/include ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(chargepricecpp ${CMAKE_SOURCE_DIR}/../libchargeprice_ffi/lib/macOS/libchargeprice_ffi.a "-framework CoreFoundation" "-framework Security")

# add the test
add_executable(test test/test.cpp)
target_include_directories(test PRIVATE ${CMAKE_SOURCE_DIR}/../libchargeprice_ffi/include ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(test chargepricecpp catch ${CMAKE_SOURCE_DIR}/../libchargeprice_ffi/lib/macOS/libchargeprice_ffi.a "-framework CoreFoundation" "-framework Security")
# For more information about 
# how the different path system works,
# take a look at the official documentation.
# See also: https://cmake.org/Wiki/CMake/Tutorials/Exporting_and_Importing_Targets

set(THIRD_PART_BUILD_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/external)
# Catch
ExternalProject_Add(project_catch
PREFIX ${THIRD_PART_BUILD_ROOT}/catch
URL https://github.com/philsquared/Catch/releases/download/v1.9.6/catch.hpp
DOWNLOAD_NO_EXTRACT 1
CONFIGURE_COMMAND ""
BUILD_COMMAND ""
INSTALL_COMMAND ""
)

ExternalProject_Get_Property(project_catch PREFIX)
add_library(catch INTERFACE)
add_dependencies(catch project_catch)
target_include_directories(catch INTERFACE ${PREFIX}/src)
#!/bin/sh

### Variables used to parameters the build

TARGET_NAME="CPApiKit"
OUTPUT_DIR="$(pwd)/universal-build"
CONFIGURATION="Release"

### Variables depending on configuration
SIMULATOR_BUILD="${OUTPUT_DIR}/build-iphonesimulator"
DEVICE_BUILD="${OUTPUT_DIR}/build-iphoneos"
### Main script

echo "⚙️ Starting building universal framework"

# Create the output/buildir directory
mkdir -p "${OUTPUT_DIR}"

### Compilation for both variant

echo "🖥 Building simulator version"
xcrun xcodebuild archive -project "${TARGET_NAME}.xcodeproj/" \
    -scheme "${TARGET_NAME}" \
    -configuration "${CONFIGURATION}" \
    -destination "generic/platform=iOS Simulator" \
    -archivePath ${SIMULATOR_BUILD} \
    SKIP_INSTALL=NO \
    BUILD_LIBRARIES_FOR_DISTRIBUTION=YES 

echo "📱Building device version"
xcrun xcodebuild archive -project "${TARGET_NAME}.xcodeproj/" \
    -scheme "${TARGET_NAME}" \
    -configuration "${CONFIGURATION}" \
    -destination "generic/platform=iOS" \
    -archivePath ${DEVICE_BUILD} \
    SKIP_INSTALL=NO \
    BUILD_LIBRARIES_FOR_DISTRIBUTION=YES 

echo "🗜 Creating universal binary"
xcodebuild -create-xcframework \
-framework "${SIMULATOR_BUILD}.xcarchive/Products/Library/Frameworks/${TARGET_NAME}.framework" \
-framework "${DEVICE_BUILD}.xcarchive/Products/Library/Frameworks/${TARGET_NAME}.framework" \
-output "${OUTPUT_DIR}/${TARGET_NAME}.xcframework"

if [ $? -eq 0 ]; then
    echo "✅ Generated universal framework generated at ${OUTPUT_DIR}/${TARGET_NAME}.framework"
else
    echo "🛑 xcodebuild failed to generate the universal framework"
fi


cmake_minimum_required(VERSION 3.22)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if ("${CMAKE_SYSTEM_NAME}" STREQUAL "iOS" OR IOS)
    set(CMAKE_OSX_ARCHITECTURES "arm64")
else()
    set(CMAKE_OSX_ARCHITECTURES "arm64;x86_64")
endif()
set(CMAKE_CXX_VISIBILITY_PRESET hidden)

project(RustAPITestMod)

include(FetchContent)

if (NOT TARGET geode-sdk)
    if (NOT DEFINED ENV{GEODE_SDK})
        message(FATAL_ERROR "Unable to find Geode SDK! Please define GEODE_SDK environment variable to point to Geode")
    endif()

    add_subdirectory($ENV{GEODE_SDK} ${CMAKE_CURRENT_BINARY_DIR}/geode)
endif()

if (APPLE)
    if (IOS OR "${CMAKE_SYSTEM_NAME}" STREQUAL "iOS")
        add_custom_command(
            OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/libtestmod.a
            COMMAND cargo build --release --target aarch64-apple-ios
            COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/target/aarch64-apple-ios/release/libtestmod.a ${CMAKE_CURRENT_BINARY_DIR}/libtestmod.a
            WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
            COMMENT "Building Rust library for iOS"
        )
    else()
        add_custom_command(
            OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/libtestmod.a
            COMMAND cargo build --release --target aarch64-apple-darwin
            COMMAND cargo build --release --target x86_64-apple-darwin
            COMMAND lipo -create -output ${CMAKE_CURRENT_BINARY_DIR}/libtestmod.a
                ${CMAKE_CURRENT_SOURCE_DIR}/target/aarch64-apple-darwin/release/libtestmod.a
                ${CMAKE_CURRENT_SOURCE_DIR}/target/x86_64-apple-darwin/release/libtestmod.a
            WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
            COMMENT "Building universal Rust library for macOS"
        )
    endif()

    add_library(testmod STATIC IMPORTED GLOBAL)
    set_target_properties(testmod PROPERTIES
        IMPORTED_LOCATION ${CMAKE_CURRENT_BINARY_DIR}/libtestmod.a
    )
    add_custom_target(testmod_target DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/libtestmod.a)
    add_dependencies(testmod testmod_target)
else()
    if (NOT COMMAND corrosion_import_crate)
        FetchContent_Declare(
            Corrosion
            GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
            GIT_TAG v0.6.1
        )
        FetchContent_MakeAvailable(Corrosion)
    endif()

    if (ANDROID OR "${CMAKE_SYSTEM_NAME}" STREQUAL "Android")
        if (GEODE_TARGET STREQUAL "Android64" OR CMAKE_ANDROID_ARCH_ABI STREQUAL "arm64-v8a" OR CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|arm64")
            set(Rust_CARGO_TARGET "aarch64-linux-android")
        else()
            set(Rust_CARGO_TARGET "armv7-linux-androideabi")
        endif()
    endif()

    corrosion_import_crate(MANIFEST_PATH Cargo.toml)
    
    if (GEODE_TARGET)
        set_target_properties(testmod PROPERTIES ARCHIVE_OUTPUT_NAME "testmod_${GEODE_TARGET}")
    else()
        set_target_properties(testmod PROPERTIES ARCHIVE_OUTPUT_NAME "testmod_${CMAKE_SYSTEM_NAME}")
    endif()
endif()

add_library(${PROJECT_NAME} SHARED
    src/Mod.cpp
)

target_include_directories(${PROJECT_NAME} PRIVATE src)
target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../include)
target_link_libraries(${PROJECT_NAME}
    geode-sdk
    testmod
    $<$<PLATFORM_ID:Windows>:ws2_32>
    $<$<PLATFORM_ID:Windows>:userenv>
    $<$<PLATFORM_ID:Windows>:bcrypt>
    $<$<PLATFORM_ID:Windows>:ntdll>
)

if (TARGET RustAPI)
    target_link_libraries(${PROJECT_NAME} RustAPI)
endif()

setup_geode_mod(${PROJECT_NAME})

if(WIN32)
    target_link_options(${PROJECT_NAME} PRIVATE
        $<$<NOT:$<CONFIG:Debug>>:LINKER:/OPT:REF>
        $<$<NOT:$<CONFIG:Debug>>:LINKER:/OPT:ICF>)
endif()
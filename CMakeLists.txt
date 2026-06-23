cmake_minimum_required(VERSION 3.22)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if ("${CMAKE_SYSTEM_NAME}" STREQUAL "iOS" OR IOS)
    set(CMAKE_OSX_ARCHITECTURES "arm64")
else()
    set(CMAKE_OSX_ARCHITECTURES "arm64;x86_64")
endif()
set(CMAKE_CXX_VISIBILITY_PRESET hidden)

project(RustAPI)

include(FetchContent)

if (NOT DEFINED ENV{GEODE_SDK})
    message(FATAL_ERROR "Unable to find Geode SDK! Please define GEODE_SDK environment variable to point to Geode")
else()
    message(STATUS "Found Geode: $ENV{GEODE_SDK}")
endif()

add_subdirectory($ENV{GEODE_SDK} ${CMAKE_CURRENT_BINARY_DIR}/geode)

set(IMGUI_VERSION "v1.90.8" CACHE STRING "What imgui version to download and use" FORCE)
FetchContent_Declare(
    gd_imgui_cocos
    GIT_REPOSITORY https://github.com/matcool/gd-imgui-cocos.git
    GIT_TAG        b93f08ccef778a53ebba09b20c347f6a63980119
)
FetchContent_MakeAvailable(gd_imgui_cocos)

if (APPLE)
    if (IOS OR "${CMAKE_SYSTEM_NAME}" STREQUAL "iOS")
        add_custom_command(
            OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/librustapi.a
            COMMAND cargo build --release --target aarch64-apple-ios
            COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/target/aarch64-apple-ios/release/librustapi.a ${CMAKE_CURRENT_BINARY_DIR}/librustapi.a
            WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
            COMMENT "Building Rust library for iOS"
        )
    else()
        add_custom_command(
            OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/librustapi.a
            COMMAND cargo build --release --target aarch64-apple-darwin
            COMMAND cargo build --release --target x86_64-apple-darwin
            COMMAND lipo -create -output ${CMAKE_CURRENT_BINARY_DIR}/librustapi.a
                ${CMAKE_CURRENT_SOURCE_DIR}/target/aarch64-apple-darwin/release/librustapi.a
                ${CMAKE_CURRENT_SOURCE_DIR}/target/x86_64-apple-darwin/release/librustapi.a
            WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
            COMMENT "Building universal Rust library for macOS"
        )
    endif()

    add_library(rustapi STATIC IMPORTED GLOBAL)
    set_target_properties(rustapi PROPERTIES
        IMPORTED_LOCATION ${CMAKE_CURRENT_BINARY_DIR}/librustapi.a
    )
    add_custom_target(rustapi_target DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/librustapi.a)
    add_dependencies(rustapi rustapi_target)
else()

    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.6.1
    )
    FetchContent_MakeAvailable(Corrosion)

    if (ANDROID OR "${CMAKE_SYSTEM_NAME}" STREQUAL "Android")
        if (GEODE_TARGET STREQUAL "Android64" OR CMAKE_ANDROID_ARCH_ABI STREQUAL "arm64-v8a" OR CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|arm64")
            set(Rust_CARGO_TARGET "aarch64-linux-android")
        else()
            set(Rust_CARGO_TARGET "armv7-linux-androideabi")
        endif()
    endif()

    corrosion_import_crate(MANIFEST_PATH Cargo.toml)
    
    if (GEODE_TARGET)
        set_target_properties(rustapi PROPERTIES ARCHIVE_OUTPUT_NAME "rustapi_${GEODE_TARGET}")
    else()
        set_target_properties(rustapi PROPERTIES ARCHIVE_OUTPUT_NAME "rustapi_${CMAKE_SYSTEM_NAME}")
    endif()
endif()

add_library(${PROJECT_NAME} SHARED
    src/main.cpp
    src/bindings/clipboard.cpp
    src/bindings/cocos_color.cpp
    src/bindings/cocos_nodes.cpp
    src/bindings/filesystem.cpp
    src/bindings/imgui_bindings.cpp
    src/bindings/json_base64.cpp
    src/bindings/keyboard.cpp
    src/bindings/keybind_web.cpp
    src/bindings/logging.cpp
    src/bindings/mod_info.cpp
    src/bindings/nodes.cpp
    src/bindings/permission.cpp
    src/bindings/popup.cpp
    src/bindings/task_scheduler.cpp
    src/bindings/utils_misc.cpp
    src/bindings/version_color.cpp
)

target_compile_definitions(${PROJECT_NAME} PRIVATE RUSTAPI_EXPORTING)
target_include_directories(${PROJECT_NAME} PUBLIC include src $<TARGET_PROPERTY:imgui-cocos,INTERFACE_INCLUDE_DIRECTORIES>)
target_link_libraries(${PROJECT_NAME}
    geode-sdk
    imgui-cocos
    rustapi
    $<$<PLATFORM_ID:Windows>:ws2_32>
    $<$<PLATFORM_ID:Windows>:userenv>
    $<$<PLATFORM_ID:Windows>:bcrypt>
    $<$<PLATFORM_ID:Windows>:ntdll>
)
setup_geode_mod(${PROJECT_NAME})

if(WIN32)
    target_link_options(${PROJECT_NAME} PRIVATE
        $<$<NOT:$<CONFIG:Debug>>:LINKER:/OPT:REF>
        $<$<NOT:$<CONFIG:Debug>>:LINKER:/OPT:ICF>)
endif()

add_subdirectory(testmod)

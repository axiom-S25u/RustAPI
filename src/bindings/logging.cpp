#include "helpers.hpp"

extern "C" {
    RUSTAPI_DLL void geode_log_info(const char* msg) { log::info("{}", msg); }
    RUSTAPI_DLL void geode_log_warn(const char* msg) { log::warn("{}", msg); }
    RUSTAPI_DLL void geode_log_error(const char* msg) { log::error("{}", msg); }
    RUSTAPI_DLL void geode_log_debug(const char* msg) { log::debug("{}", msg); }
}

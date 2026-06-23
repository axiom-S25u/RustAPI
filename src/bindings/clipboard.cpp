#include "helpers.hpp"
#include <Geode/utils/general.hpp>

extern "C" {
    RUSTAPI_DLL char* geode_clipboard_read() {
        return alloc_string(geode::utils::clipboard::read());
    }

    RUSTAPI_DLL bool geode_clipboard_write(const char* text) {
        return geode::utils::clipboard::write(text);
    }
}
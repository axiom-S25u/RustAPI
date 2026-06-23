#include "helpers.hpp"
#include <Geode/utils/general.hpp>
#include <Geode/utils/random.hpp>
#include <Geode/ui/GeodeUI.hpp>

extern "C" {
    // game
    RUSTAPI_DLL void geode_game_exit() { geode::utils::game::exit(true); }
    RUSTAPI_DLL void geode_game_restart() { geode::utils::game::restart(true); }
    RUSTAPI_DLL void geode_game_launch_loader_uninstaller(bool delete_save) {
        geode::utils::game::launchLoaderUninstaller(delete_save);
    }

    // thread
    RUSTAPI_DLL char* geode_thread_get_default_name() {
        return alloc_string(geode::utils::thread::getDefaultName());
    }
    RUSTAPI_DLL char* geode_thread_get_name() {
        return alloc_string(geode::utils::thread::getName());
    }
    RUSTAPI_DLL void geode_thread_set_name(const char* name) {
        geode::utils::thread::setName(name);
    }

    // platform
    RUSTAPI_DLL char* geode_platform_get_string() {
        return alloc_string(geode::utils::platform::getString());
    }
    RUSTAPI_DLL bool geode_platform_is_wine() {
        return geode::utils::platform::isWine();
    }

    // random
    RUSTAPI_DLL char* geode_random_uuid() {
        return alloc_string(geode::utils::random::generateUUID());
    }
    RUSTAPI_DLL char* geode_random_hex_string(int length) {
        return alloc_string(geode::utils::random::generateHexString(length));
    }
    RUSTAPI_DLL char* geode_random_alphanumeric_string(int length) {
        return alloc_string(geode::utils::random::generateAlphanumericString(length));
    }
    RUSTAPI_DLL char* geode_random_string(int length, const char* chars) {
        return alloc_string(geode::utils::random::generateString(length, chars));
    }

    // UI
    RUSTAPI_DLL void geode_ui_open_mods_list() {
        geode::openModsList();
    }
    RUSTAPI_DLL void* geode_ui_create_default_logo() {
        return geode::createDefaultLogo();
    }
    RUSTAPI_DLL void* geode_ui_create_server_mod_logo(const char* mod_id) {
        return geode::createServerModLogo(mod_id);
    }
}

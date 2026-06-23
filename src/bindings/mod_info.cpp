#include "helpers.hpp"
#include <Geode/loader/SettingV3.hpp>
#include <vector>

static std::vector<geode::ListenerHandle>& settingListeners() {
    static auto* s = new std::vector<geode::ListenerHandle>();
    return *s;
}

extern "C" {
    RUSTAPI_DLL char* geode_mod_get_id() {
        return alloc_string(Mod::get()->getID());
    }
    RUSTAPI_DLL char* geode_mod_get_name() {
        return alloc_string(Mod::get()->getName());
    }
    RUSTAPI_DLL char* geode_mod_get_version() {
        return alloc_string(Mod::get()->getVersion().toVString());
    }
    RUSTAPI_DLL char* geode_mod_get_resources_dir() {
        return alloc_string(utils::string::pathToString(Mod::get()->getResourcesDir()));
    }
    RUSTAPI_DLL char* geode_mod_get_save_dir() {
        return alloc_string(utils::string::pathToString(Mod::get()->getSaveDir()));
    }
    RUSTAPI_DLL char* geode_mod_get_config_dir() {
        return alloc_string(utils::string::pathToString(Mod::get()->getConfigDir()));
    }
    RUSTAPI_DLL char* geode_mod_get_persistent_dir() {
        return alloc_string(utils::string::pathToString(Mod::get()->getPersistentDir()));
    }

    RUSTAPI_DLL char* geode_mod_get_saved_value(const char* key) {
        auto res = Mod::get()->getSavedValue<matjson::Value>(key);
        return alloc_string(res.dump());
    }
    RUSTAPI_DLL void geode_mod_set_saved_value(const char* key, const char* json_val) {
        auto val = matjson::parse(json_val).unwrapOrDefault();
        Mod::get()->setSavedValue(key, val);
    }

    RUSTAPI_DLL char* geode_mod_get_setting_value(const char* key) {
        auto setting = Mod::get()->getSetting(key);
        if (!setting) return nullptr;
        matjson::Value val;
        if (!setting->save(val)) return nullptr;
        return alloc_string(val.dump());
    }

    RUSTAPI_DLL bool geode_mod_has_setting(const char* key) {
        return Mod::get()->hasSetting(key);
    }
    RUSTAPI_DLL void geode_mod_listen_setting_changes(const char* key, RustSettingCallback cb) {
        std::string k(key);
        settingListeners().push_back(
            geode::SettingChangedEventV3(Mod::get(), key).listen(
                [cb, k](std::shared_ptr<geode::SettingV3> setting) {
                    matjson::Value val;
                    if (setting && setting->save(val)) {
                        std::string s = val.dump();
                        geode::queueInMainThread([cb, k, s = std::move(s)]() {
                            if (cb) cb(k.c_str(), s.c_str());
                        });
                    }
                }
            )
        );
    }

    RUSTAPI_DLL void geode_mod_listen_all_setting_changes(RustAllSettingsCallback cb) {
        std::string modID = Mod::get()->getID();
        settingListeners().push_back(
            geode::SettingChangedEventV3().listen(
                [cb, modID](std::string_view evModID, std::string_view key, std::shared_ptr<geode::SettingV3> setting) {
                    if (evModID != modID) return;
                    matjson::Value val;
                    if (setting && setting->save(val)) {
                        std::string s = val.dump();
                        std::string k(key);
                        geode::queueInMainThread([cb, k = std::move(k), s = std::move(s)]() {
                            if (cb) cb(k.c_str(), s.c_str());
                        });
                    }
                }
            )
        );
    }

    // checks platform compile definitions to return current target to rust
    RUSTAPI_DLL bool geode_platform_is_windows() {
#ifdef GEODE_IS_WINDOWS
        return true;
#else
        return false;
#endif
    }

    RUSTAPI_DLL bool geode_platform_is_macos() {
#ifdef GEODE_IS_MACOS
        return true;
#else
        return false;
#endif
    }

    RUSTAPI_DLL bool geode_platform_is_ios() {
#ifdef GEODE_IS_IOS
        return true;
#else
        return false;
#endif
    }

    RUSTAPI_DLL bool geode_platform_is_android() {
#ifdef GEODE_IS_ANDROID
        return true;
#else
        return false;
#endif
    }
}


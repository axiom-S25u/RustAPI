#include "helpers.hpp"
#include <Geode/utils/permission.hpp>

extern "C" {
    RUSTAPI_DLL bool geode_permission_get_status(int perm) {
        return geode::utils::permission::getPermissionStatus(
            static_cast<geode::utils::permission::Permission>(perm)
        );
    }

    typedef void (*RustPermissionCallback)(bool granted);

    RUSTAPI_DLL void geode_permission_request(int perm, RustPermissionCallback cb) {
        geode::utils::permission::requestPermission(
            static_cast<geode::utils::permission::Permission>(perm),
            [cb](bool granted) {
                geode::queueInMainThread([cb, granted]() {
                    if (cb) cb(granted);
                });
            }
        );
    }
}

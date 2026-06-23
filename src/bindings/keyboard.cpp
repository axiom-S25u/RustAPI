#include "helpers.hpp"
#include <Geode/utils/Keyboard.hpp>

struct KeyboardListenerBox {
    geode::ListenerHandle listener;
    RustKeyboardCallback cb;
    void* user_data;
};

extern "C" {
    RUSTAPI_DLL void* geode_keyboard_listen(int key, int priority, RustKeyboardCallback cb, void* user_data) {
        KeyboardListenerBox* box = new KeyboardListenerBox();
        box->cb = cb;
        box->user_data = user_data;
        
        auto event = (key >= 0) 
            ? geode::KeyboardInputEvent(static_cast<cocos2d::enumKeyCodes>(key))
            : geode::KeyboardInputEvent();
            
        box->listener = event.listen(
            [box](geode::KeyboardInputData& data) {
                RustKeyboardInputData rust_data {
                    static_cast<int>(data.key),
                    static_cast<int>(data.action),
                    static_cast<int>(data.modifiers),
                    data.timestamp,
                    data.native.code,
                    data.native.extra
                };
                bool stop = false;
                if (box->cb) {
                    stop = box->cb(&rust_data, box->user_data);
                }
                return stop ? geode::ListenerResult::Stop : geode::ListenerResult::Propagate;
            },
            priority
        );
        
        return box;
    }

    RUSTAPI_DLL void geode_keyboard_disconnect(void* handle) {
        if (handle) {
            delete static_cast<KeyboardListenerBox*>(handle);
        }
    }
}

void rustapi_init_keyboard() {}
void rustapi_shutdown_keyboard() {}


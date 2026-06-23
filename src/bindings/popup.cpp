#include <Geode/ui/Popup.hpp>
#include "helpers.hpp"

class RustAlertDelegate : public cocos2d::CCNode, public FLAlertLayerProtocol {
public:
    void (*m_cb)(bool);

    static RustAlertDelegate* create(void (*cb)(bool)) {
        auto ret = new RustAlertDelegate();
        if (ret && ret->init()) {
            ret->m_cb = cb;
            ret->autorelease();
            return ret;
        }
        CC_SAFE_DELETE(ret);
        return nullptr;
    }

    void FLAlert_Clicked(FLAlertLayer* layer, bool btn2) override {
        if (m_cb) m_cb(btn2);
    }
};

// subclass of geode popup that delegates callback execution back to rust closures using raw pointers
class RustPopup : public geode::Popup {
public:
    void (*m_on_init)(void*, void*);
    void (*m_on_close)(void*, void*);
    void (*m_on_destruct)(void*);
    void* m_user_data;

    static RustPopup* create(float width, float height, void (*on_init)(void*, void*), void (*on_close)(void*, void*), void (*on_destruct)(void*), void* user_data) {
        auto ret = new RustPopup();
        ret->m_on_init = on_init;
        ret->m_on_close = on_close;
        ret->m_on_destruct = on_destruct;
        ret->m_user_data = user_data;
        if (ret && ret->init(width, height)) {
            ret->autorelease();
            return ret;
        }
        CC_SAFE_DELETE(ret);
        return nullptr;
    }

    ~RustPopup() {
        if (m_on_destruct) m_on_destruct(m_user_data);
    }

    void rust_close(cocos2d::CCObject* sender) {
        onClose(sender);
    }

    void rust_set_title(const char* title) {
        if (title) setTitle(title);
    }

    CCMenuItemSpriteExtra* rust_get_close_btn() {
        return m_closeBtn;
    }

    NineSlice* rust_get_bg_sprite() {
        return m_bgSprite;
    }

    cocos2d::CCSize rust_get_size() {
        return m_size;
    }

    void rust_set_close_button_spr(cocos2d::CCSprite* spr, float scale) {
        if (spr) setCloseButtonSpr(spr, scale);
    }

protected:
    bool init(float width, float height) {
        if (!geode::Popup::init(width, height)) return false;
        if (m_on_init) m_on_init(this, m_user_data);
        return true;
    }

    void onClose(cocos2d::CCObject* sender) override {
        if (m_on_close) m_on_close(this, m_user_data);
        geode::Popup::onClose(sender);
    }
};

extern "C" {
    RUSTAPI_DLL void geode_create_quick_popup(const char* title, const char* content, const char* btn1, const char* btn2, void (*cb)(bool)) {
        createQuickPopup(title, content, btn1, btn2, [cb](auto*, bool b2) {
            if (cb) cb(b2);
        });
    }

    RUSTAPI_DLL void geode_create_alert_popup(const char* title, const char* content, const char* btn1, const char* btn2, void (*cb)(bool)) {
        auto delegate = RustAlertDelegate::create(cb);
        auto alert = FLAlertLayer::create(delegate, title, content, btn1, btn2);
        if (delegate && alert) {
            alert->addChild(delegate);
        }
        if (alert) {
            alert->show();
        }
    }

    RUSTAPI_DLL void* geode_popup_create(
        float width,
        float height,
        void (*on_init)(void*, void*),
        void (*on_close)(void*, void*),
        void (*on_destruct)(void*),
        void* user_data
    ) {
        return RustPopup::create(width, height, on_init, on_close, on_destruct, user_data);
    }

    RUSTAPI_DLL void geode_popup_show(void* popup) {
        if (popup) {
            static_cast<RustPopup*>(popup)->show();
        }
    }

    RUSTAPI_DLL void geode_popup_close(void* popup) {
        if (popup) {
            static_cast<RustPopup*>(popup)->rust_close(nullptr);
        }
    }

    RUSTAPI_DLL void geode_popup_set_title(void* popup, const char* title) {
        if (popup && title) {
            static_cast<RustPopup*>(popup)->rust_set_title(title);
        }
    }

    RUSTAPI_DLL void* geode_popup_get_main_layer(void* popup) {
        if (popup) {
            return static_cast<RustPopup*>(popup)->m_mainLayer;
        }
        return nullptr;
    }

    RUSTAPI_DLL void* geode_popup_get_close_btn(void* popup) {
        if (popup) {
            return static_cast<RustPopup*>(popup)->rust_get_close_btn();
        }
        return nullptr;
    }

    RUSTAPI_DLL void* geode_popup_get_bg_sprite(void* popup) {
        if (popup) {
            return static_cast<RustPopup*>(popup)->rust_get_bg_sprite();
        }
        return nullptr;
    }

    RUSTAPI_DLL void geode_popup_get_size(void* popup, float* w, float* h) {
        if (popup && w && h) {
            auto size = static_cast<RustPopup*>(popup)->rust_get_size();
            *w = size.width;
            *h = size.height;
        }
    }

    RUSTAPI_DLL void* geode_popup_get_button_menu(void* popup) {
        if (popup) {
            return static_cast<RustPopup*>(popup)->m_buttonMenu;
        }
        return nullptr;
    }

    RUSTAPI_DLL void geode_popup_set_no_elasticity(void* popup, bool val) {
        if (popup) {
            static_cast<RustPopup*>(popup)->m_noElasticity = val;
        }
    }

    RUSTAPI_DLL bool geode_popup_get_no_elasticity(void* popup) {
        if (popup) {
            return static_cast<RustPopup*>(popup)->m_noElasticity;
        }
        return false;
    }

    RUSTAPI_DLL void geode_popup_set_reverse_key_back(void* popup, bool val) {
        if (popup) {
            static_cast<RustPopup*>(popup)->m_reverseKeyBack = val;
        }
    }

    RUSTAPI_DLL bool geode_popup_get_reverse_key_back(void* popup) {
        if (popup) {
            return static_cast<RustPopup*>(popup)->m_reverseKeyBack;
        }
        return false;
    }

    RUSTAPI_DLL void geode_popup_set_close_button_spr(void* popup, void* sprite, float scale) {
        if (popup && sprite) {
            static_cast<RustPopup*>(popup)->rust_set_close_button_spr(static_cast<cocos2d::CCSprite*>(sprite), scale);
        }
    }
}

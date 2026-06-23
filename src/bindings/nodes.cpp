#include "helpers.hpp"

extern "C" {
    RUSTAPI_DLL void geode_node_set_position(void* node, float x, float y) {
        if (node) {
            static_cast<cocos2d::CCNode*>(node)->setPosition(x, y);
        }
    }

    RUSTAPI_DLL void geode_node_get_position(void* node, float* x, float* y) {
        if (node && x && y) {
            auto pos = static_cast<cocos2d::CCNode*>(node)->getPosition();
            *x = pos.x;
            *y = pos.y;
        }
    }

    RUSTAPI_DLL void geode_node_set_scale(void* node, float scale) {
        if (node) {
            static_cast<cocos2d::CCNode*>(node)->setScale(scale);
        }
    }

    RUSTAPI_DLL float geode_node_get_scale(void* node) {
        if (node) {
            return static_cast<cocos2d::CCNode*>(node)->getScale();
        }
        return 1.0f;
    }

    RUSTAPI_DLL void geode_node_set_visible(void* node, bool visible) {
        if (node) {
            static_cast<cocos2d::CCNode*>(node)->setVisible(visible);
        }
    }

    RUSTAPI_DLL bool geode_node_get_visible(void* node) {
        if (node) {
            return static_cast<cocos2d::CCNode*>(node)->isVisible();
        }
        return false;
    }

    RUSTAPI_DLL void* geode_get_current_scene() {
        return CCDirector::sharedDirector()->getRunningScene();
    }
}

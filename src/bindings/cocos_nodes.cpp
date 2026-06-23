#include "helpers.hpp"

extern "C" {
    RUSTAPI_DLL void cocos_node_set_pos(void* node, float x, float y) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->setPosition(x, y); 
    }
    RUSTAPI_DLL void cocos_node_get_pos(void* node, float* x, float* y) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        auto p = n->getPosition(); *x = p.x; *y = p.y; 
    }
    RUSTAPI_DLL void cocos_node_set_scale(void* node, float s) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->setScale(s); 
    }
    RUSTAPI_DLL float cocos_node_get_scale(void* node) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        return n->getScale(); 
    }
    RUSTAPI_DLL void cocos_node_set_rot(void* node, float r) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->setRotation(r); 
    }
    RUSTAPI_DLL float cocos_node_get_rot(void* node) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        return n->getRotation(); 
    }
    RUSTAPI_DLL void cocos_node_set_visible(void* node, bool v) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->setVisible(v); 
    }
    RUSTAPI_DLL bool cocos_node_is_visible(void* node) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        return n->isVisible(); 
    }
    RUSTAPI_DLL void cocos_node_set_opacity(void* node, uint8_t o) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        if (auto rgba = typeinfo_cast<cocos2d::CCRGBAProtocol*>(n)) rgba->setOpacity(o);
    }
    RUSTAPI_DLL uint8_t cocos_node_get_opacity(void* node) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        if (auto rgba = typeinfo_cast<cocos2d::CCRGBAProtocol*>(n)) return rgba->getOpacity();
        return 255;
    }
    RUSTAPI_DLL void cocos_node_set_color(void* node, uint8_t r, uint8_t g, uint8_t b) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        if (auto rgba = typeinfo_cast<cocos2d::CCRGBAProtocol*>(n)) {
            ccColor3B c; c.r = r; c.g = g; c.b = b;
            rgba->setColor(c);
        }
    }
    RUSTAPI_DLL void cocos_node_get_color(void* node, uint8_t* r, uint8_t* g, uint8_t* b) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        if (auto rgba = typeinfo_cast<cocos2d::CCRGBAProtocol*>(n)) {
            auto c = rgba->getColor(); *r = c.r; *g = c.g; *b = c.b;
        } else {
            *r = *g = *b = 255;
        }
    }
    RUSTAPI_DLL void cocos_node_set_content_size(void* node, float w, float h) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->setContentSize({w, h});
    }
    RUSTAPI_DLL void cocos_node_get_content_size(void* node, float* w, float* h) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        auto s = n->getContentSize();
        *w = s.width; *h = s.height;
    }
    RUSTAPI_DLL void cocos_node_set_anchor(void* node, float x, float y) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->setAnchorPoint({x, y});
    }
    RUSTAPI_DLL void cocos_node_get_anchor(void* node, float* x, float* y) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        auto a = n->getAnchorPoint();
        *x = a.x; *y = a.y;
    }
    RUSTAPI_DLL void cocos_node_set_z_order(void* node, int z) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->setZOrder(z);
    }
    RUSTAPI_DLL int cocos_node_get_z_order(void* node) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        return n->getZOrder();
    }
    RUSTAPI_DLL int cocos_node_get_child_count(void* node) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        return n->getChildrenCount();
    }
    RUSTAPI_DLL void cocos_node_set_id(void* node, const char* id) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->setID(id);
    }
    RUSTAPI_DLL char* cocos_node_get_id(void* node) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        return alloc_string(n->getID());
    }
    RUSTAPI_DLL void* cocos_node_get_child_by_id(void* node, const char* id) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        return n->getChildByID(id);
    }
    RUSTAPI_DLL void geode_node_add_child(void* node, void* child) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        auto c = static_cast<cocos2d::CCNode*>(child);
        n->addChild(c); 
    }
    RUSTAPI_DLL void geode_node_add_child_z(void* node, void* child, int z) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        auto c = static_cast<cocos2d::CCNode*>(child);
        n->addChild(c, z); 
    }
    RUSTAPI_DLL void geode_node_remove_from_parent(void* node) { 
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->removeFromParent(); 
    }
    RUSTAPI_DLL void geode_node_remove_all_children(void* node) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        n->removeAllChildren();
    }
    RUSTAPI_DLL void* cocos_node_get_parent(void* node) {
        auto n = static_cast<cocos2d::CCNode*>(node);
        return n->getParent();
    }

    RUSTAPI_DLL void cocos_get_win_size(float* w, float* h) {
        auto s = cocos2d::CCDirector::get()->getWinSize();
        *w = s.width; *h = s.height;
    }

    RUSTAPI_DLL void geode_free_string(char* s) {
        if (s) free(s);
    }

    RUSTAPI_DLL void geode_cocos_calculate_child_coverage(void* node, float* x, float* y, float* w, float* h) {
        auto rect = geode::cocos::calculateChildCoverage(static_cast<cocos2d::CCNode*>(node));
        *x = rect.origin.x; *y = rect.origin.y; *w = rect.size.width; *h = rect.size.height;
    }

    // helper takes array of raw pointers and wraps them in vector to query geode coverage math
    RUSTAPI_DLL void geode_cocos_calculate_node_coverage(void** nodes, int count, float* x, float* y, float* w, float* h) {
        std::vector<cocos2d::CCNode*> node_vec;
        for (int i = 0; i < count; i++) {
            node_vec.push_back(static_cast<cocos2d::CCNode*>(nodes[i]));
        }
        auto rect = geode::cocos::calculateNodeCoverage(node_vec);
        *x = rect.origin.x; *y = rect.origin.y; *w = rect.size.width; *h = rect.size.height;
    }

    RUSTAPI_DLL void* geode_cocos_switch_to_scene(void* layer) {
        return geode::cocos::switchToScene(static_cast<cocos2d::CCLayer*>(layer));
    }

    RUSTAPI_DLL bool geode_cocos_node_is_visible(void* node) {
        return geode::cocos::nodeIsVisible(static_cast<cocos2d::CCNode*>(node));
    }

    RUSTAPI_DLL void* geode_cocos_get_child_by_tag_recursive(void* node, int tag) {
        return geode::cocos::getChildByTagRecursive(static_cast<cocos2d::CCNode*>(node), tag);
    }

    RUSTAPI_DLL bool geode_cocos_is_sprite_frame_name(void* node, const char* name) {
        return geode::cocos::isSpriteFrameName(static_cast<cocos2d::CCNode*>(node), name);
    }

    RUSTAPI_DLL void* geode_cocos_get_child_by_sprite_frame_name(void* node, const char* name) {
        return geode::cocos::getChildBySpriteFrameName(static_cast<cocos2d::CCNode*>(node), name);
    }

    RUSTAPI_DLL bool geode_cocos_is_sprite_name(void* node, const char* name) {
        return geode::cocos::isSpriteName(static_cast<cocos2d::CCNode*>(node), name);
    }

    RUSTAPI_DLL void* geode_cocos_get_child_by_sprite_name(void* node, const char* name) {
        return geode::cocos::getChildBySpriteName(static_cast<cocos2d::CCNode*>(node), name);
    }

    RUSTAPI_DLL void geode_cocos_get_mouse_pos(float* x, float* y) {
        auto pos = geode::cocos::getMousePos();
        *x = pos.x; *y = pos.y;
    }

    RUSTAPI_DLL void geode_cocos_get_label_size(const char* text, const char* font, float kerning, float* w, float* h) {
        auto size = geode::cocos::getLabelSize(text, font, static_cast<int>(kerning));
        *w = size.width; *h = size.height;
    }

    RUSTAPI_DLL bool geode_cocos_file_exists_in_search_paths(const char* filename) {
        return geode::cocos::fileExistsInSearchPaths(filename);
    }

    RUSTAPI_DLL void geode_cocos_limit_node_size(void* node, float w, float h, float def, float min) {
        geode::cocos::limitNodeSize(static_cast<cocos2d::CCNode*>(node), {w, h}, def, min);
    }

    RUSTAPI_DLL void geode_cocos_limit_node_width(void* node, float width, float def, float min) {
        geode::cocos::limitNodeWidth(static_cast<cocos2d::CCNode*>(node), width, def, min);
    }

    RUSTAPI_DLL void geode_cocos_limit_node_height(void* node, float height, float def, float min) {
        geode::cocos::limitNodeHeight(static_cast<cocos2d::CCNode*>(node), height, def, min);
    }
}

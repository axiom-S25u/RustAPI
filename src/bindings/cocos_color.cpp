#include "helpers.hpp"
#include <Geode/utils/cocos.hpp>

extern "C" {
    RUSTAPI_DLL void geode_cocos_invert3b(uint8_t r, uint8_t g, uint8_t b, uint8_t* or_, uint8_t* og, uint8_t* ob) {
        ccColor3B color = {r, g, b};
        auto inv = geode::cocos::invert3B(color);
        *or_ = inv.r; *og = inv.g; *ob = inv.b;
    }

    RUSTAPI_DLL void geode_cocos_lighten3b(uint8_t r, uint8_t g, uint8_t b, int amount, uint8_t* or_, uint8_t* og, uint8_t* ob) {
        ccColor3B color = {r, g, b};
        auto out = geode::cocos::lighten3B(color, amount);
        *or_ = out.r; *og = out.g; *ob = out.b;
    }

    RUSTAPI_DLL void geode_cocos_darken3b(uint8_t r, uint8_t g, uint8_t b, int amount, uint8_t* or_, uint8_t* og, uint8_t* ob) {
        ccColor3B color = {r, g, b};
        auto out = geode::cocos::darken3B(color, amount);
        *or_ = out.r; *og = out.g; *ob = out.b;
    }

    RUSTAPI_DLL bool geode_cocos_cc3b_from_hex(const char* hex, uint8_t* r, uint8_t* g, uint8_t* b) {
        auto res = geode::cocos::cc3bFromHexString(hex, true);
        if (res.isErr()) return false;
        auto c = res.unwrap();
        *r = c.r; *g = c.g; *b = c.b;
        return true;
    }

    RUSTAPI_DLL bool geode_cocos_cc4b_from_hex(const char* hex, uint8_t* r, uint8_t* g, uint8_t* b, uint8_t* a) {
        auto res = geode::cocos::cc4bFromHexString(hex, false, true);
        if (res.isErr()) return false;
        auto c = res.unwrap();
        *r = c.r; *g = c.g; *b = c.b; *a = c.a;
        return true;
    }

    RUSTAPI_DLL void geode_cocos_invert4b(uint8_t r, uint8_t g, uint8_t b, uint8_t a, uint8_t* or_, uint8_t* og, uint8_t* ob, uint8_t* oa) {
        ccColor4B color = {r, g, b, a};
        auto inv = geode::cocos::invert4B(color);
        *or_ = inv.r; *og = inv.g; *ob = inv.b; *oa = inv.a;
    }

    RUSTAPI_DLL void geode_cocos_to3b(uint8_t r, uint8_t g, uint8_t b, uint8_t a, uint8_t* or_, uint8_t* og, uint8_t* ob) {
        ccColor4B color = {r, g, b, a};
        auto out = geode::cocos::to3B(color);
        *or_ = out.r; *og = out.g; *ob = out.b;
    }

    RUSTAPI_DLL void geode_cocos_to4b(uint8_t r, uint8_t g, uint8_t b, uint8_t alpha, uint8_t* or_, uint8_t* og, uint8_t* ob, uint8_t* oa) {
        ccColor3B color = {r, g, b};
        auto out = geode::cocos::to4B(color, alpha);
        *or_ = out.r; *og = out.g; *ob = out.b; *oa = out.a;
    }

    RUSTAPI_DLL void geode_cocos_to4f(uint8_t r, uint8_t g, uint8_t b, uint8_t a, float* or_, float* og, float* ob, float* oa) {
        ccColor4B color = {r, g, b, a};
        auto out = geode::cocos::to4F(color);
        *or_ = out.r; *og = out.g; *ob = out.b; *oa = out.a;
    }

    RUSTAPI_DLL char* geode_cocos_cc3b_to_hex(uint8_t r, uint8_t g, uint8_t b) {
        ccColor3B color = {r, g, b};
        return alloc_string(geode::cocos::cc3bToHexString(color));
    }

    RUSTAPI_DLL char* geode_cocos_cc4b_to_hex(uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
        ccColor4B color = {r, g, b, a};
        return alloc_string(geode::cocos::cc4bToHexString(color));
    }
}


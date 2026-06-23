#include "helpers.hpp"
#include <Geode/utils/web.hpp>
#include <Geode/utils/async.hpp>
#include <chrono>

extern "C" {
    RUSTAPI_DLL char* geode_keybind_from_string(const char* str) {
        auto res = Keybind::fromString(str);
        if (res.isErr()) return nullptr;
        auto kb = res.unwrap();
        matjson::Value obj = matjson::Value::object();
        obj.set("key", static_cast<int>(kb.key));
        obj.set("modifiers", static_cast<int>(kb.modifiers));
        return alloc_string(obj.dump());
    }

    RUSTAPI_DLL char* geode_keybind_to_string(int key, int modifiers) {
        Keybind kb(static_cast<cocos2d::enumKeyCodes>(key), static_cast<KeyboardModifier>(modifiers));
        return alloc_string(kb.toString());
    }

    RUSTAPI_DLL void geode_web_open_link(const char* url) {
        geode::utils::web::openLinkInBrowser(url);
    }

    RUSTAPI_DLL void* geode_web_request_new() {
        return new geode::utils::web::WebRequest();
    }

    RUSTAPI_DLL void geode_web_request_free(void* handle) {
        if (handle) {
            delete static_cast<geode::utils::web::WebRequest*>(handle);
        }
    }

    RUSTAPI_DLL void geode_web_request_header(void* handle, const char* name, const char* value) {
        if (handle) {
            static_cast<geode::utils::web::WebRequest*>(handle)->header(name, value);
        }
    }

    RUSTAPI_DLL void geode_web_request_param(void* handle, const char* name, const char* value) {
        if (handle) {
            static_cast<geode::utils::web::WebRequest*>(handle)->param(name, value);
        }
    }

    RUSTAPI_DLL void geode_web_request_timeout(void* handle, int seconds) {
        if (handle) {
            static_cast<geode::utils::web::WebRequest*>(handle)->timeout(std::chrono::seconds(seconds));
        }
    }

    RUSTAPI_DLL void geode_web_request_body_string(void* handle, const char* data) {
        if (handle) {
            static_cast<geode::utils::web::WebRequest*>(handle)->bodyString(data);
        }
    }

    RUSTAPI_DLL void geode_web_request_body_json(void* handle, const char* json) {
        if (handle) {
            auto val = matjson::parse(json).unwrapOrDefault();
            static_cast<geode::utils::web::WebRequest*>(handle)->bodyJSON(std::move(val));
        }
    }

    struct WebSendContext {
        geode::utils::web::WebRequest req;
        std::string method;
        std::string url;
        RustWebRequestCallback cb;
        void* user_data;
    };

    RUSTAPI_DLL void geode_web_request_send(void* handle, const char* method, const char* url, RustWebRequestCallback cb, void* user_data) {
        auto* reqPtr = static_cast<geode::utils::web::WebRequest*>(handle);
        auto* ctx = new WebSendContext{std::move(*reqPtr), std::string(method), std::string(url), cb, user_data};
        delete reqPtr;

        auto future = ctx->req.send(ctx->method, ctx->url);

        async::spawn(
            std::move(future),
            [ctx](geode::utils::web::WebResponse res) {
                int status = res.code();

                matjson::Value headersArr = matjson::Value::array();
                for (const std::string& header : res.headers()) {
                    headersArr.push(header);
                }
                std::string headersJson = headersArr.dump();

                std::string bodyStr;
                std::string errStr;

                if (res.ok()) {
                    bodyStr = res.string().unwrapOr("");
                } else {
                    errStr = "HTTP error " + std::to_string(status);
                }

                if (ctx->cb) {
                    ctx->cb(
                        status,
                        headersJson.c_str(),
                        res.ok() ? bodyStr.c_str() : nullptr,
                        res.ok() ? nullptr : errStr.c_str(),
                        ctx->user_data
                    );
                }
                delete ctx;
            }
        );
    }
}
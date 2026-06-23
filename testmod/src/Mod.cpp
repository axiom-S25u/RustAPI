#include <Geode/Geode.hpp>
#include <RustAPI.hpp>

using namespace geode::prelude;

extern "C" {
    void testmod_init();
    void testmod_shutdown();
}

$on_mod(Loaded) {
    log::info("loading rust testmod");
    testmod_init();
}
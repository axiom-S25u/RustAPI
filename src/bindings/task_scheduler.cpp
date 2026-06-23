#include "helpers.hpp"
#include <chrono>

// stored handles indexed by id, run on scheduler tick
struct DelayTask {
    uint64_t id;
    double triggerAt;
    double interval; // 0 = one-shot
    void (*cb)(uint64_t id, void* data);
    void* data;
};

static std::vector<DelayTask> g_tasks;
static uint64_t g_nextId = 1;
static bool g_schedulerArmed = false;

static double timeNowSeconds() {
    static std::chrono::steady_clock::time_point origin = std::chrono::steady_clock::now();
    std::chrono::duration<double> elapsed = std::chrono::steady_clock::now() - origin;
    return elapsed.count();
}

struct TaskTickNode : cocos2d::CCNode {
    bool init() override {
        if (!CCNode::init()) return false;
        this->schedule(schedule_selector(TaskTickNode::tick), 0.0f);
        return true;
    }
    CREATE_FUNC(TaskTickNode);

    void tick(float) {
        double now = timeNowSeconds();
        std::vector<DelayTask> toFire;
        std::vector<DelayTask> remaining;

        for (DelayTask& t : g_tasks) {
            if (now >= t.triggerAt) {
                toFire.push_back(t);
                if (t.interval > 0.0) {
                    t.triggerAt = now + t.interval;
                    remaining.push_back(t);
                }
            } else {
                remaining.push_back(t);
            }
        }
        g_tasks = remaining;

        for (DelayTask& t : toFire) {
            if (t.cb) t.cb(t.id, t.data);
        }
    }
};

static TaskTickNode* g_tickNode = nullptr;

static void ensureScheduler() {
    if (g_schedulerArmed) return;
    g_tickNode = TaskTickNode::create();
    g_tickNode->retain();
    // keep it alive by attaching to a persistent scene node
    CCDirector::get()->getRunningScene()->addChild(g_tickNode);
    g_schedulerArmed = true;
}

extern "C" {
    RUSTAPI_DLL uint64_t geode_task_delay(double seconds, void (*cb)(uint64_t id, void* data), void* data) {
        uint64_t id = g_nextId++;
        double at = timeNowSeconds() + (seconds < 0.0 ? 0.0 : seconds);
        g_tasks.push_back({id, at, 0.0, cb, data});
        // arm lazily — if no scene yet, tasks fire once scene exists
        geode::queueInMainThread([](){ ensureScheduler(); });
        return id;
    }

    RUSTAPI_DLL uint64_t geode_task_every(double seconds, void (*cb)(uint64_t id, void* data), void* data) {
        if (seconds <= 0.0) seconds = 0.016; // floor at ~60fps
        uint64_t id = g_nextId++;
        double at = timeNowSeconds() + seconds;
        g_tasks.push_back({id, at, seconds, cb, data});
        geode::queueInMainThread([](){ ensureScheduler(); });
        return id;
    }

    RUSTAPI_DLL void geode_task_cancel(uint64_t id) {
        g_tasks.erase(
            std::remove_if(g_tasks.begin(), g_tasks.end(), [id](DelayTask const& t){ return t.id == id; }),
            g_tasks.end()
        );
    }

    RUSTAPI_DLL void geode_task_run_main(void (*cb)(void*), void* data) {
        geode::queueInMainThread([cb, data](){ if (cb) cb(data); });
    }

    RUSTAPI_DLL double geode_time_now() {
        return timeNowSeconds();
    }

    RUSTAPI_DLL double geode_time_unix() {
        std::chrono::duration<double> secs = std::chrono::system_clock::now().time_since_epoch();
        return secs.count();
    }
}

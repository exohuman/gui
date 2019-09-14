// for modern linux... opther platforms will use a different define
// #define VK_USE_PLATFORM_XCB_KHR

// for windows
#define VK_USE_PLATFORM_WIN32_KHR

// #define VK_USE_PLATFORM_ANDROID_KHR
// #define VK_USE_PLATFORM_FUCHSIA
// #define VK_USE_PLATFORM_IOS_MVK
// #define VK_USE_PLATFORM_MACOS_MVK
// #define VK_USE_PLATFORM_METAL_EXT
// #define VK_USE_PLATFORM_VI_NN
// #define VK_USE_PLATFORM_WAYLAND_KHR
// #define VK_USE_PLATFORM_XCB_KHR
// #define VK_USE_PLATFORM_XLIB_KHR
// #define VK_USE_PLATFORM_XLIB_XRANDR_EXT
// #define VK_USE_PLATFORM_GGP

// windows (avoiding including windows.h)
#ifdef VK_USE_PLATFORM_WIN32_KHR
    // #define _X86_
    #define _AMD64_
    // #define _ARM_
    #define WIN32_LEAN_AND_MEAN
    #include <windows.h>
    #include <vulkan/vk_platform.h>
    #include <vulkan/vulkan_core.h>
    #include <vulkan/vulkan_win32.h>
#endif

// everyone else
// #ifndef VK_USE_PLATFORM_WIN32_KHR
    // #include <vulkan/vulkan.h>
// #endif
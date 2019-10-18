#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use super::function_pointers::*;

pub type VkResult = i32;
pub type VkStructureType = i32;
pub type VkInstanceCreateFlags = VkFlags;
pub type int_least8_t = ::std::os::raw::c_schar;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type int_least32_t = ::std::os::raw::c_int;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type int_fast16_t = ::std::os::raw::c_short;
pub type uint_fast16_t = ::std::os::raw::c_ushort;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub type VkFlags = u32;
pub type VkBool32 = u32;
pub type VkDeviceSize = u64;
pub type VkSampleMask = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInstance_T {
    _unused: [u8; 0],
}
pub type VkInstance = *mut VkInstance_T;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDevice_T {
    _unused: [u8; 0],
}
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDevice_T {
    _unused: [u8; 0],
}
pub type VkDevice = *mut VkDevice_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueue_T {
    _unused: [u8; 0],
}
pub type VkQueue = *mut VkQueue_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphore_T {
    _unused: [u8; 0],
}
pub type VkSemaphore = *mut VkSemaphore_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBuffer_T {
    _unused: [u8; 0],
}
pub type VkCommandBuffer = *mut VkCommandBuffer_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFence_T {
    _unused: [u8; 0],
}
pub type VkFence = *mut VkFence_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceMemory_T {
    _unused: [u8; 0],
}
pub type VkDeviceMemory = *mut VkDeviceMemory_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBuffer_T {
    _unused: [u8; 0],
}
pub type VkBuffer = *mut VkBuffer_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImage_T {
    _unused: [u8; 0],
}
pub type VkImage = *mut VkImage_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkEvent_T {
    _unused: [u8; 0],
}
pub type VkEvent = *mut VkEvent_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueryPool_T {
    _unused: [u8; 0],
}
pub type VkQueryPool = *mut VkQueryPool_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferView_T {
    _unused: [u8; 0],
}
pub type VkBufferView = *mut VkBufferView_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageView_T {
    _unused: [u8; 0],
}
pub type VkImageView = *mut VkImageView_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShaderModule_T {
    _unused: [u8; 0],
}
pub type VkShaderModule = *mut VkShaderModule_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineCache_T {
    _unused: [u8; 0],
}
pub type VkPipelineCache = *mut VkPipelineCache_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineLayout_T {
    _unused: [u8; 0],
}
pub type VkPipelineLayout = *mut VkPipelineLayout_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPass_T {
    _unused: [u8; 0],
}
pub type VkRenderPass = *mut VkRenderPass_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipeline_T {
    _unused: [u8; 0],
}
pub type VkPipeline = *mut VkPipeline_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayout_T {
    _unused: [u8; 0],
}
pub type VkDescriptorSetLayout = *mut VkDescriptorSetLayout_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSampler_T {
    _unused: [u8; 0],
}
pub type VkSampler = *mut VkSampler_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPool_T {
    _unused: [u8; 0],
}
pub type VkDescriptorPool = *mut VkDescriptorPool_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSet_T {
    _unused: [u8; 0],
}
pub type VkDescriptorSet = *mut VkDescriptorSet_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebuffer_T {
    _unused: [u8; 0],
}
pub type VkFramebuffer = *mut VkFramebuffer_T;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandPool_T {
    _unused: [u8; 0],
}
pub type VkCommandPool = *mut VkCommandPool_T;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const ::std::os::raw::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub pApplicationName: *const ::std::os::raw::c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const ::std::os::raw::c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut ::std::os::raw::c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}
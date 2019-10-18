#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

use super::structures::*;
use super::constants::*;

pub type PFN_vkAllocationFunction = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        size: usize,
        alignment: usize,
        allocationScope: VkSystemAllocationScope,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type PFN_vkReallocationFunction = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        pOriginal: *mut ::std::os::raw::c_void,
        size: usize,
        alignment: usize,
        allocationScope: VkSystemAllocationScope,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type PFN_vkFreeFunction = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        pMemory: *mut ::std::os::raw::c_void,
    ),
>;
pub type PFN_vkInternalAllocationNotification = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        size: usize,
        allocationType: VkInternalAllocationType,
        allocationScope: VkSystemAllocationScope,
    ),
>;
pub type PFN_vkInternalFreeNotification = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        size: usize,
        allocationType: VkInternalAllocationType,
        allocationScope: VkSystemAllocationScope,
    ),
>;

pub type PFN_vkVoidFunction = ::std::option::Option<unsafe extern "C" fn()>;

pub type PFN_vkGetInstanceProcAddr = ::std::option::Option<
    unsafe extern "C" fn(
        instance: VkInstance,
        pName: *const ::std::os::raw::c_char,
    ) -> PFN_vkVoidFunction,
>;

pub type PFN_vkGetDeviceProcAddr = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pName: *const ::std::os::raw::c_char,
    ) -> PFN_vkVoidFunction,
>;

pub type PFN_vkEnumerateInstanceVersion =
::std::option::Option<unsafe extern "C" fn(pApiVersion: *mut u32) -> VkResult>;

pub type PFN_vkCreateInstance = ::std::option::Option<
    unsafe extern "C" fn(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult,
>;

pub type PFN_vkDestroyInstance = ::std::option::Option<
    unsafe extern "C" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks),
>;

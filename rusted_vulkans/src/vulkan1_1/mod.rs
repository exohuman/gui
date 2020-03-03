// Vulkan 1.1
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![feature(process_exitcode_placeholder)]
#![feature(termination_trait_lib)]

pub mod function_pointers;
pub mod constants;
pub mod structures;
#[macro_use]
pub mod macros;

pub use constants::*;
pub use structures::*;
pub use function_pointers::*;

#[macro_use]
pub use macros::*;

use std::ffi::{CString};

//
// Command Function Pointers and Instances
//

// Command Function Pointers


// Instances


extern "C" {
    pub fn vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> VkResult;
}

extern "C" {
    pub fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult;
}

extern "C" {
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::ptr;
    use std::os::raw::c_char;
    use std::ffi::{CStr};
    use std::mem::MaybeUninit;

    static application_name: &str = "Vulkan Library Test\0";
    static engine_name: &str = "custom\0";

    struct VulkanExample1 {
        instance: Option<VkInstance>,
        app_info: Option<VkApplicationInfo>,
        create_info: Option<VkInstanceCreateInfo>,
    }

    fn get_extensions() -> Vec<*const c_char> {
        let mut enabled_extensions = vec!(CStr::from_bytes_with_nul(VK_KHR_SURFACE_EXTENSION_NAME).unwrap().as_ptr());

        #[cfg(windows)] {
            enabled_extensions.push(CStr::from_bytes_with_nul(VK_KHR_WIN32_SURFACE_EXTENSION_NAME).unwrap().as_ptr());
        }
        #[cfg(android)] {
            enabled_extensions.push(CStr::from_bytes_with_nul(VK_KHR_ANDROID_SURFACE_EXTENSION_NAME).unwrap().as_ptr());
        }
        #[cfg(unix)] {
            enabled_extensions.push(CStr::from_bytes_with_nul(VK_KHR_XCB_SURFACE_EXTENSION_NAME).unwrap().as_ptr());
            println!("Enabled unix extensions");
        }

        return enabled_extensions;
    }

    fn cleanup(example: &VulkanExample1) {
        unsafe { vkDestroyInstance(example.instance.unwrap(), ptr::null()) };
    }

/*
    #[test]
    fn can_initialize_vulkan() {
        let extensions = get_extensions();
        let app_info = VkApplicationInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: ptr::null(),
            pApplicationName: application_name.as_ptr() as *const c_char,
            pEngineName: engine_name.as_ptr() as *const c_char,
            apiVersion: vk_make_version!(1; 0; 3),
            applicationVersion: 1,
            engineVersion: 1,
        };

        let create_info = VkInstanceCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            pApplicationInfo: &app_info as *const VkApplicationInfo,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: extensions.len() as u32,
            ppEnabledExtensionNames: extensions.as_ptr(),
        };

        let mut instance = MaybeUninit::uninit();

        let inst_result: VkResult = unsafe {
            vkCreateInstance(
                &create_info as *const VkInstanceCreateInfo,
                ptr::null(), // pAllocator
                instance.as_mut_ptr()
            )
        };

        assert_ne!(inst_result, VkResult_VK_ERROR_INCOMPATIBLE_DRIVER);
        assert_eq!(inst_result, VkResult_VK_SUCCESS);

        let example: VulkanExample1 =
            VulkanExample1 {
                instance: Some(unsafe { instance.assume_init() }),
                app_info: Some(app_info),
                create_info: Some(create_info),
            };

        let info = example.app_info.unwrap();

        cleanup(&example);
    }
}
*/
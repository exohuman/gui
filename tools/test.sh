#!/bin/bash
# run from the top level of the project

if [ -d "C:\VulkanSDK" ]; then
    export VULKAN_PATH="C:\VulkanSDK\1.1.114.0"
    export VK_SDK_PATH="C:\VulkanSDK\1.1.114.0"
    export LLVM_PATH="C:\Program Files\LLVM"
    export LIBCLANG_PATH="C:\Program Files\LLVM\bin\libclang.dll"
    export LLVM_CONFIG_PATH="C:\Program Files\LLVM\bin\llvm-config"
    export LIBCLANG_STATIC_PATH="C:\Program Files\LLVM\lib"
    export CLANG_PATH="C:\Program Files\LLVM\bin\clang.exe"
    export RUST_BACKTRACE=1 
fi

cargo test

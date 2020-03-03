
macro_rules! vk_version_major {
    ($version:expr) => {
       ((u32)($version) >> 22)
    };
}

macro_rules! vk_version_minor {
    ($version:expr) => {
       (((u32)($version) >> 12) & 0x3ff)
    };
}

macro_rules! vk_version_patch {
    ($version:expr) => {
       ((u32)($version) & 0xfff)
    };
}

macro_rules! vk_make_version {
    ($major:expr, $minor:expr, $patch:expr) => {
            ((($major) << 22) | (($minor) << 12) | ($patch))
    };
}
use std::ptr;
pub(crate) use winapi::shared::minwindef::HMODULE;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winnt::IMAGE_DOS_HEADER;
pub(crate) use winapi::um::winnt::{IMAGE_NT_HEADERS, IMAGE_SECTION_HEADER};

pub(crate) fn trim_null_bytes(input: &str) -> &str {
    if let Some(index) = input.find('\0') {
        &input[..index]
    } else {
        input
    }
}

pub(crate) unsafe fn get_base_module_handle() -> HMODULE {
    GetModuleHandleW(ptr::null())
}

pub(crate) unsafe fn get_nt_headers(module_handle: HMODULE) -> *const IMAGE_NT_HEADERS {
    let dos_header = module_handle as *const IMAGE_DOS_HEADER;
    (module_handle as usize + (*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS
}

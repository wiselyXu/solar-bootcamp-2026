#![no_std]

#[derive(Debug, Clone, Copy)]
pub struct Error;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "getrandom error")
    }
}

#[cfg(feature = "custom")]
pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    // 简单实现
    for byte in dest {
        *byte = 0;
    }
    Ok(())
}

// 导出 C ABI 函数
#[cfg(feature = "custom")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __getrandom_custom(
    dest: *mut u8,
    len: usize,
) -> i32 {
    let slice = core::slice::from_raw_parts_mut(dest, len);
    match getrandom(slice) {
        Ok(()) => 0,
        Err(_) => 1,
    }
}
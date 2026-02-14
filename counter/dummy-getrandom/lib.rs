#![no_std]

pub fn getrandom(_buf: &mut [u8]) -> Result<(), GetRandomError> {
    // 简单返回成功，不生成真正的随机数
    // 对于 Solana 程序，应该使用 solana_program::hash 或时钟来获取随机性
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct GetRandomError;

impl core::fmt::Display for GetRandomError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GetRandomError")
    }
}
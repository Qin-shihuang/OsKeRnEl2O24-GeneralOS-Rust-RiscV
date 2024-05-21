extern "C" {
    pub static __kernel_start: u8;
    pub static __kernel_end: u8;
    pub static __text_start: u8;
    pub static __text_end: u8;
    pub static __rodata_start: u8;
    pub static __rodata_end: u8;
    pub static __data_start: u8;
    pub static __data_end: u8;
    pub static __bss_start: u8;
    pub static __bss_end: u8;
}
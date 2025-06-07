use bitflags::bitflags;

pub const XL9555_BASE_ADDR: u8 = 0x20;

#[doc = "Input Port Register 0"]
pub const XL9555_INPUT_PORT0_REG: u8 = 0;
#[doc = "Input Port Register 1"]
pub const XL9555_INPUT_PORT1_REG : u8 = 1;
#[doc = "Output Port Register 0"]
pub const XL9555_OUTPUT_PORT0_REG: u8 = 2;
#[doc = "Output Port Register 1"]
pub const XL9555_OUTPUT_PORT1_REG : u8 = 3;
#[doc = "Polarity Inversion Port Register 0"]
pub const XL9555_INVERSION_PORT0_REG: u8 = 4;
#[doc = "Polarity Inversion Port Register 1"]
pub const XL9555_INVERSION_PORT1_REG: u8 = 5;
#[doc = "Configuration Port Register 0"]
pub const XL9555_CONFIG_PORT0_REG: u8 = 6;
#[doc = "Configuration Port Register 1"]
pub const XL9555_CONFIG_PORT1_REG: u8 = 7;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pin: u16 {
        const P00 = 1;
        const P01 = 1 << 1;
        const P02 = 1 << 2;
        const P03 = 1 << 3;
        const P04 = 1 << 4;
        const P05 = 1 << 5;
        const P06 = 1 << 6;
        const P07 = 1 << 7;
        const P10 = 1 << 8;
        const P11 = 1 << 9;
        const P12 = 1 << 10;
        const P13 = 1 << 11;
        const P14 = 1 << 12;
        const P15 = 1 << 13;
        const P16 = 1 << 14;
        const P17 = 1 << 15;
    }
}
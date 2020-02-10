#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::dma0::Instance;
pub use crate::imxrt105::peripherals::dma0::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::dma0::{
    CDNE, CEEI, CERQ, CERR, CINT, CR, DCHPRI0, DCHPRI1, DCHPRI10, DCHPRI11, DCHPRI12, DCHPRI13,
    DCHPRI14, DCHPRI15, DCHPRI16, DCHPRI17, DCHPRI18, DCHPRI19, DCHPRI2, DCHPRI20, DCHPRI21,
    DCHPRI22, DCHPRI23, DCHPRI24, DCHPRI25, DCHPRI26, DCHPRI27, DCHPRI28, DCHPRI29, DCHPRI3,
    DCHPRI30, DCHPRI31, DCHPRI4, DCHPRI5, DCHPRI6, DCHPRI7, DCHPRI8, DCHPRI9, EARS, EEI, ERQ, ERR,
    ES, HRS, INT, SEEI, SERQ, SSRT, TCD0_ATTR, TCD0_BITER_ELINK, TCD0_CITER_ELINK, TCD0_CSR,
    TCD0_DADDR, TCD0_DLASTSGA, TCD0_DOFF, TCD0_NBYTES_ML, TCD0_SADDR, TCD0_SLAST, TCD0_SOFF,
    TCD10_ATTR, TCD10_BITER_ELINK, TCD10_CITER_ELINK, TCD10_CSR, TCD10_DADDR, TCD10_DLASTSGA,
    TCD10_DOFF, TCD10_NBYTES_ML, TCD10_SADDR, TCD10_SLAST, TCD10_SOFF, TCD11_ATTR,
    TCD11_BITER_ELINK, TCD11_CITER_ELINK, TCD11_CSR, TCD11_DADDR, TCD11_DLASTSGA, TCD11_DOFF,
    TCD11_NBYTES_ML, TCD11_SADDR, TCD11_SLAST, TCD11_SOFF, TCD12_ATTR, TCD12_BITER_ELINK,
    TCD12_CITER_ELINK, TCD12_CSR, TCD12_DADDR, TCD12_DLASTSGA, TCD12_DOFF, TCD12_NBYTES_ML,
    TCD12_SADDR, TCD12_SLAST, TCD12_SOFF, TCD13_ATTR, TCD13_BITER_ELINK, TCD13_CITER_ELINK,
    TCD13_CSR, TCD13_DADDR, TCD13_DLASTSGA, TCD13_DOFF, TCD13_NBYTES_ML, TCD13_SADDR, TCD13_SLAST,
    TCD13_SOFF, TCD14_ATTR, TCD14_BITER_ELINK, TCD14_CITER_ELINK, TCD14_CSR, TCD14_DADDR,
    TCD14_DLASTSGA, TCD14_DOFF, TCD14_NBYTES_ML, TCD14_SADDR, TCD14_SLAST, TCD14_SOFF, TCD15_ATTR,
    TCD15_BITER_ELINK, TCD15_CITER_ELINK, TCD15_CSR, TCD15_DADDR, TCD15_DLASTSGA, TCD15_DOFF,
    TCD15_NBYTES_ML, TCD15_SADDR, TCD15_SLAST, TCD15_SOFF, TCD16_ATTR, TCD16_BITER_ELINK,
    TCD16_CITER_ELINK, TCD16_CSR, TCD16_DADDR, TCD16_DLASTSGA, TCD16_DOFF, TCD16_NBYTES_ML,
    TCD16_SADDR, TCD16_SLAST, TCD16_SOFF, TCD17_ATTR, TCD17_BITER_ELINK, TCD17_CITER_ELINK,
    TCD17_CSR, TCD17_DADDR, TCD17_DLASTSGA, TCD17_DOFF, TCD17_NBYTES_ML, TCD17_SADDR, TCD17_SLAST,
    TCD17_SOFF, TCD18_ATTR, TCD18_BITER_ELINK, TCD18_CITER_ELINK, TCD18_CSR, TCD18_DADDR,
    TCD18_DLASTSGA, TCD18_DOFF, TCD18_NBYTES_ML, TCD18_SADDR, TCD18_SLAST, TCD18_SOFF, TCD19_ATTR,
    TCD19_BITER_ELINK, TCD19_CITER_ELINK, TCD19_CSR, TCD19_DADDR, TCD19_DLASTSGA, TCD19_DOFF,
    TCD19_NBYTES_ML, TCD19_SADDR, TCD19_SLAST, TCD19_SOFF, TCD1_ATTR, TCD1_BITER_ELINK,
    TCD1_CITER_ELINK, TCD1_CSR, TCD1_DADDR, TCD1_DLASTSGA, TCD1_DOFF, TCD1_NBYTES_ML, TCD1_SADDR,
    TCD1_SLAST, TCD1_SOFF, TCD20_ATTR, TCD20_BITER_ELINK, TCD20_CITER_ELINK, TCD20_CSR,
    TCD20_DADDR, TCD20_DLASTSGA, TCD20_DOFF, TCD20_NBYTES_ML, TCD20_SADDR, TCD20_SLAST, TCD20_SOFF,
    TCD21_ATTR, TCD21_BITER_ELINK, TCD21_CITER_ELINK, TCD21_CSR, TCD21_DADDR, TCD21_DLASTSGA,
    TCD21_DOFF, TCD21_NBYTES_ML, TCD21_SADDR, TCD21_SLAST, TCD21_SOFF, TCD22_ATTR,
    TCD22_BITER_ELINK, TCD22_CITER_ELINK, TCD22_CSR, TCD22_DADDR, TCD22_DLASTSGA, TCD22_DOFF,
    TCD22_NBYTES_ML, TCD22_SADDR, TCD22_SLAST, TCD22_SOFF, TCD23_ATTR, TCD23_BITER_ELINK,
    TCD23_CITER_ELINK, TCD23_CSR, TCD23_DADDR, TCD23_DLASTSGA, TCD23_DOFF, TCD23_NBYTES_ML,
    TCD23_SADDR, TCD23_SLAST, TCD23_SOFF, TCD24_ATTR, TCD24_BITER_ELINK, TCD24_CITER_ELINK,
    TCD24_CSR, TCD24_DADDR, TCD24_DLASTSGA, TCD24_DOFF, TCD24_NBYTES_ML, TCD24_SADDR, TCD24_SLAST,
    TCD24_SOFF, TCD25_ATTR, TCD25_BITER_ELINK, TCD25_CITER_ELINK, TCD25_CSR, TCD25_DADDR,
    TCD25_DLASTSGA, TCD25_DOFF, TCD25_NBYTES_ML, TCD25_SADDR, TCD25_SLAST, TCD25_SOFF, TCD26_ATTR,
    TCD26_BITER_ELINK, TCD26_CITER_ELINK, TCD26_CSR, TCD26_DADDR, TCD26_DLASTSGA, TCD26_DOFF,
    TCD26_NBYTES_ML, TCD26_SADDR, TCD26_SLAST, TCD26_SOFF, TCD27_ATTR, TCD27_BITER_ELINK,
    TCD27_CITER_ELINK, TCD27_CSR, TCD27_DADDR, TCD27_DLASTSGA, TCD27_DOFF, TCD27_NBYTES_ML,
    TCD27_SADDR, TCD27_SLAST, TCD27_SOFF, TCD28_ATTR, TCD28_BITER_ELINK, TCD28_CITER_ELINK,
    TCD28_CSR, TCD28_DADDR, TCD28_DLASTSGA, TCD28_DOFF, TCD28_NBYTES_ML, TCD28_SADDR, TCD28_SLAST,
    TCD28_SOFF, TCD29_ATTR, TCD29_BITER_ELINK, TCD29_CITER_ELINK, TCD29_CSR, TCD29_DADDR,
    TCD29_DLASTSGA, TCD29_DOFF, TCD29_NBYTES_ML, TCD29_SADDR, TCD29_SLAST, TCD29_SOFF, TCD2_ATTR,
    TCD2_BITER_ELINK, TCD2_CITER_ELINK, TCD2_CSR, TCD2_DADDR, TCD2_DLASTSGA, TCD2_DOFF,
    TCD2_NBYTES_ML, TCD2_SADDR, TCD2_SLAST, TCD2_SOFF, TCD30_ATTR, TCD30_BITER_ELINK,
    TCD30_CITER_ELINK, TCD30_CSR, TCD30_DADDR, TCD30_DLASTSGA, TCD30_DOFF, TCD30_NBYTES_ML,
    TCD30_SADDR, TCD30_SLAST, TCD30_SOFF, TCD31_ATTR, TCD31_BITER_ELINK, TCD31_CITER_ELINK,
    TCD31_CSR, TCD31_DADDR, TCD31_DLASTSGA, TCD31_DOFF, TCD31_NBYTES_ML, TCD31_SADDR, TCD31_SLAST,
    TCD31_SOFF, TCD3_ATTR, TCD3_BITER_ELINK, TCD3_CITER_ELINK, TCD3_CSR, TCD3_DADDR, TCD3_DLASTSGA,
    TCD3_DOFF, TCD3_NBYTES_ML, TCD3_SADDR, TCD3_SLAST, TCD3_SOFF, TCD4_ATTR, TCD4_BITER_ELINK,
    TCD4_CITER_ELINK, TCD4_CSR, TCD4_DADDR, TCD4_DLASTSGA, TCD4_DOFF, TCD4_NBYTES_ML, TCD4_SADDR,
    TCD4_SLAST, TCD4_SOFF, TCD5_ATTR, TCD5_BITER_ELINK, TCD5_CITER_ELINK, TCD5_CSR, TCD5_DADDR,
    TCD5_DLASTSGA, TCD5_DOFF, TCD5_NBYTES_ML, TCD5_SADDR, TCD5_SLAST, TCD5_SOFF, TCD6_ATTR,
    TCD6_BITER_ELINK, TCD6_CITER_ELINK, TCD6_CSR, TCD6_DADDR, TCD6_DLASTSGA, TCD6_DOFF,
    TCD6_NBYTES_ML, TCD6_SADDR, TCD6_SLAST, TCD6_SOFF, TCD7_ATTR, TCD7_BITER_ELINK,
    TCD7_CITER_ELINK, TCD7_CSR, TCD7_DADDR, TCD7_DLASTSGA, TCD7_DOFF, TCD7_NBYTES_ML, TCD7_SADDR,
    TCD7_SLAST, TCD7_SOFF, TCD8_ATTR, TCD8_BITER_ELINK, TCD8_CITER_ELINK, TCD8_CSR, TCD8_DADDR,
    TCD8_DLASTSGA, TCD8_DOFF, TCD8_NBYTES_ML, TCD8_SADDR, TCD8_SLAST, TCD8_SOFF, TCD9_ATTR,
    TCD9_BITER_ELINK, TCD9_CITER_ELINK, TCD9_CSR, TCD9_DADDR, TCD9_DLASTSGA, TCD9_DOFF,
    TCD9_NBYTES_ML, TCD9_SADDR, TCD9_SLAST, TCD9_SOFF,
};

/// Access functions for the DMA0 peripheral instance
pub mod DMA0 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400e8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA0
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000400,
        ES: 0x00000000,
        ERQ: 0x00000000,
        EEI: 0x00000000,
        CEEI: 0x00000000,
        SEEI: 0x00000000,
        CERQ: 0x00000000,
        SERQ: 0x00000000,
        CDNE: 0x00000000,
        SSRT: 0x00000000,
        CERR: 0x00000000,
        CINT: 0x00000000,
        INT: 0x00000000,
        ERR: 0x00000000,
        HRS: 0x00000000,
        EARS: 0x00000000,
        DCHPRI3: 0x00000003,
        DCHPRI2: 0x00000002,
        DCHPRI1: 0x00000001,
        DCHPRI0: 0x00000000,
        DCHPRI7: 0x00000007,
        DCHPRI6: 0x00000006,
        DCHPRI5: 0x00000005,
        DCHPRI4: 0x00000004,
        DCHPRI11: 0x0000000B,
        DCHPRI10: 0x0000000A,
        DCHPRI9: 0x00000009,
        DCHPRI8: 0x00000008,
        DCHPRI15: 0x0000000F,
        DCHPRI14: 0x0000000E,
        DCHPRI13: 0x0000000D,
        DCHPRI12: 0x0000000C,
        DCHPRI19: 0x00000013,
        DCHPRI18: 0x00000012,
        DCHPRI17: 0x00000011,
        DCHPRI16: 0x00000010,
        DCHPRI23: 0x00000017,
        DCHPRI22: 0x00000016,
        DCHPRI21: 0x00000015,
        DCHPRI20: 0x00000014,
        DCHPRI27: 0x0000001B,
        DCHPRI26: 0x0000001A,
        DCHPRI25: 0x00000019,
        DCHPRI24: 0x00000018,
        DCHPRI31: 0x0000001F,
        DCHPRI30: 0x0000001E,
        DCHPRI29: 0x0000001D,
        DCHPRI28: 0x0000001C,
        TCD0_SADDR: 0x00000000,
        TCD0_SOFF: 0x00000000,
        TCD0_ATTR: 0x00000000,
        TCD0_NBYTES_ML: 0x00000000,
        TCD0_SLAST: 0x00000000,
        TCD0_DADDR: 0x00000000,
        TCD0_DOFF: 0x00000000,
        TCD0_CITER_ELINK: 0x00000000,
        TCD0_DLASTSGA: 0x00000000,
        TCD0_CSR: 0x00000000,
        TCD0_BITER_ELINK: 0x00000000,
        TCD1_SADDR: 0x00000000,
        TCD1_SOFF: 0x00000000,
        TCD1_ATTR: 0x00000000,
        TCD1_NBYTES_ML: 0x00000000,
        TCD1_SLAST: 0x00000000,
        TCD1_DADDR: 0x00000000,
        TCD1_DOFF: 0x00000000,
        TCD1_CITER_ELINK: 0x00000000,
        TCD1_DLASTSGA: 0x00000000,
        TCD1_CSR: 0x00000000,
        TCD1_BITER_ELINK: 0x00000000,
        TCD2_SADDR: 0x00000000,
        TCD2_SOFF: 0x00000000,
        TCD2_ATTR: 0x00000000,
        TCD2_NBYTES_ML: 0x00000000,
        TCD2_SLAST: 0x00000000,
        TCD2_DADDR: 0x00000000,
        TCD2_DOFF: 0x00000000,
        TCD2_CITER_ELINK: 0x00000000,
        TCD2_DLASTSGA: 0x00000000,
        TCD2_CSR: 0x00000000,
        TCD2_BITER_ELINK: 0x00000000,
        TCD3_SADDR: 0x00000000,
        TCD3_SOFF: 0x00000000,
        TCD3_ATTR: 0x00000000,
        TCD3_NBYTES_ML: 0x00000000,
        TCD3_SLAST: 0x00000000,
        TCD3_DADDR: 0x00000000,
        TCD3_DOFF: 0x00000000,
        TCD3_CITER_ELINK: 0x00000000,
        TCD3_DLASTSGA: 0x00000000,
        TCD3_CSR: 0x00000000,
        TCD3_BITER_ELINK: 0x00000000,
        TCD4_SADDR: 0x00000000,
        TCD4_SOFF: 0x00000000,
        TCD4_ATTR: 0x00000000,
        TCD4_NBYTES_ML: 0x00000000,
        TCD4_SLAST: 0x00000000,
        TCD4_DADDR: 0x00000000,
        TCD4_DOFF: 0x00000000,
        TCD4_CITER_ELINK: 0x00000000,
        TCD4_DLASTSGA: 0x00000000,
        TCD4_CSR: 0x00000000,
        TCD4_BITER_ELINK: 0x00000000,
        TCD5_SADDR: 0x00000000,
        TCD5_SOFF: 0x00000000,
        TCD5_ATTR: 0x00000000,
        TCD5_NBYTES_ML: 0x00000000,
        TCD5_SLAST: 0x00000000,
        TCD5_DADDR: 0x00000000,
        TCD5_DOFF: 0x00000000,
        TCD5_CITER_ELINK: 0x00000000,
        TCD5_DLASTSGA: 0x00000000,
        TCD5_CSR: 0x00000000,
        TCD5_BITER_ELINK: 0x00000000,
        TCD6_SADDR: 0x00000000,
        TCD6_SOFF: 0x00000000,
        TCD6_ATTR: 0x00000000,
        TCD6_NBYTES_ML: 0x00000000,
        TCD6_SLAST: 0x00000000,
        TCD6_DADDR: 0x00000000,
        TCD6_DOFF: 0x00000000,
        TCD6_CITER_ELINK: 0x00000000,
        TCD6_DLASTSGA: 0x00000000,
        TCD6_CSR: 0x00000000,
        TCD6_BITER_ELINK: 0x00000000,
        TCD7_SADDR: 0x00000000,
        TCD7_SOFF: 0x00000000,
        TCD7_ATTR: 0x00000000,
        TCD7_NBYTES_ML: 0x00000000,
        TCD7_SLAST: 0x00000000,
        TCD7_DADDR: 0x00000000,
        TCD7_DOFF: 0x00000000,
        TCD7_CITER_ELINK: 0x00000000,
        TCD7_DLASTSGA: 0x00000000,
        TCD7_CSR: 0x00000000,
        TCD7_BITER_ELINK: 0x00000000,
        TCD8_SADDR: 0x00000000,
        TCD8_SOFF: 0x00000000,
        TCD8_ATTR: 0x00000000,
        TCD8_NBYTES_ML: 0x00000000,
        TCD8_SLAST: 0x00000000,
        TCD8_DADDR: 0x00000000,
        TCD8_DOFF: 0x00000000,
        TCD8_CITER_ELINK: 0x00000000,
        TCD8_DLASTSGA: 0x00000000,
        TCD8_CSR: 0x00000000,
        TCD8_BITER_ELINK: 0x00000000,
        TCD9_SADDR: 0x00000000,
        TCD9_SOFF: 0x00000000,
        TCD9_ATTR: 0x00000000,
        TCD9_NBYTES_ML: 0x00000000,
        TCD9_SLAST: 0x00000000,
        TCD9_DADDR: 0x00000000,
        TCD9_DOFF: 0x00000000,
        TCD9_CITER_ELINK: 0x00000000,
        TCD9_DLASTSGA: 0x00000000,
        TCD9_CSR: 0x00000000,
        TCD9_BITER_ELINK: 0x00000000,
        TCD10_SADDR: 0x00000000,
        TCD10_SOFF: 0x00000000,
        TCD10_ATTR: 0x00000000,
        TCD10_NBYTES_ML: 0x00000000,
        TCD10_SLAST: 0x00000000,
        TCD10_DADDR: 0x00000000,
        TCD10_DOFF: 0x00000000,
        TCD10_CITER_ELINK: 0x00000000,
        TCD10_DLASTSGA: 0x00000000,
        TCD10_CSR: 0x00000000,
        TCD10_BITER_ELINK: 0x00000000,
        TCD11_SADDR: 0x00000000,
        TCD11_SOFF: 0x00000000,
        TCD11_ATTR: 0x00000000,
        TCD11_NBYTES_ML: 0x00000000,
        TCD11_SLAST: 0x00000000,
        TCD11_DADDR: 0x00000000,
        TCD11_DOFF: 0x00000000,
        TCD11_CITER_ELINK: 0x00000000,
        TCD11_DLASTSGA: 0x00000000,
        TCD11_CSR: 0x00000000,
        TCD11_BITER_ELINK: 0x00000000,
        TCD12_SADDR: 0x00000000,
        TCD12_SOFF: 0x00000000,
        TCD12_ATTR: 0x00000000,
        TCD12_NBYTES_ML: 0x00000000,
        TCD12_SLAST: 0x00000000,
        TCD12_DADDR: 0x00000000,
        TCD12_DOFF: 0x00000000,
        TCD12_CITER_ELINK: 0x00000000,
        TCD12_DLASTSGA: 0x00000000,
        TCD12_CSR: 0x00000000,
        TCD12_BITER_ELINK: 0x00000000,
        TCD13_SADDR: 0x00000000,
        TCD13_SOFF: 0x00000000,
        TCD13_ATTR: 0x00000000,
        TCD13_NBYTES_ML: 0x00000000,
        TCD13_SLAST: 0x00000000,
        TCD13_DADDR: 0x00000000,
        TCD13_DOFF: 0x00000000,
        TCD13_CITER_ELINK: 0x00000000,
        TCD13_DLASTSGA: 0x00000000,
        TCD13_CSR: 0x00000000,
        TCD13_BITER_ELINK: 0x00000000,
        TCD14_SADDR: 0x00000000,
        TCD14_SOFF: 0x00000000,
        TCD14_ATTR: 0x00000000,
        TCD14_NBYTES_ML: 0x00000000,
        TCD14_SLAST: 0x00000000,
        TCD14_DADDR: 0x00000000,
        TCD14_DOFF: 0x00000000,
        TCD14_CITER_ELINK: 0x00000000,
        TCD14_DLASTSGA: 0x00000000,
        TCD14_CSR: 0x00000000,
        TCD14_BITER_ELINK: 0x00000000,
        TCD15_SADDR: 0x00000000,
        TCD15_SOFF: 0x00000000,
        TCD15_ATTR: 0x00000000,
        TCD15_NBYTES_ML: 0x00000000,
        TCD15_SLAST: 0x00000000,
        TCD15_DADDR: 0x00000000,
        TCD15_DOFF: 0x00000000,
        TCD15_CITER_ELINK: 0x00000000,
        TCD15_DLASTSGA: 0x00000000,
        TCD15_CSR: 0x00000000,
        TCD15_BITER_ELINK: 0x00000000,
        TCD16_SADDR: 0x00000000,
        TCD16_SOFF: 0x00000000,
        TCD16_ATTR: 0x00000000,
        TCD16_NBYTES_ML: 0x00000000,
        TCD16_SLAST: 0x00000000,
        TCD16_DADDR: 0x00000000,
        TCD16_DOFF: 0x00000000,
        TCD16_CITER_ELINK: 0x00000000,
        TCD16_DLASTSGA: 0x00000000,
        TCD16_CSR: 0x00000000,
        TCD16_BITER_ELINK: 0x00000000,
        TCD17_SADDR: 0x00000000,
        TCD17_SOFF: 0x00000000,
        TCD17_ATTR: 0x00000000,
        TCD17_NBYTES_ML: 0x00000000,
        TCD17_SLAST: 0x00000000,
        TCD17_DADDR: 0x00000000,
        TCD17_DOFF: 0x00000000,
        TCD17_CITER_ELINK: 0x00000000,
        TCD17_DLASTSGA: 0x00000000,
        TCD17_CSR: 0x00000000,
        TCD17_BITER_ELINK: 0x00000000,
        TCD18_SADDR: 0x00000000,
        TCD18_SOFF: 0x00000000,
        TCD18_ATTR: 0x00000000,
        TCD18_NBYTES_ML: 0x00000000,
        TCD18_SLAST: 0x00000000,
        TCD18_DADDR: 0x00000000,
        TCD18_DOFF: 0x00000000,
        TCD18_CITER_ELINK: 0x00000000,
        TCD18_DLASTSGA: 0x00000000,
        TCD18_CSR: 0x00000000,
        TCD18_BITER_ELINK: 0x00000000,
        TCD19_SADDR: 0x00000000,
        TCD19_SOFF: 0x00000000,
        TCD19_ATTR: 0x00000000,
        TCD19_NBYTES_ML: 0x00000000,
        TCD19_SLAST: 0x00000000,
        TCD19_DADDR: 0x00000000,
        TCD19_DOFF: 0x00000000,
        TCD19_CITER_ELINK: 0x00000000,
        TCD19_DLASTSGA: 0x00000000,
        TCD19_CSR: 0x00000000,
        TCD19_BITER_ELINK: 0x00000000,
        TCD20_SADDR: 0x00000000,
        TCD20_SOFF: 0x00000000,
        TCD20_ATTR: 0x00000000,
        TCD20_NBYTES_ML: 0x00000000,
        TCD20_SLAST: 0x00000000,
        TCD20_DADDR: 0x00000000,
        TCD20_DOFF: 0x00000000,
        TCD20_CITER_ELINK: 0x00000000,
        TCD20_DLASTSGA: 0x00000000,
        TCD20_CSR: 0x00000000,
        TCD20_BITER_ELINK: 0x00000000,
        TCD21_SADDR: 0x00000000,
        TCD21_SOFF: 0x00000000,
        TCD21_ATTR: 0x00000000,
        TCD21_NBYTES_ML: 0x00000000,
        TCD21_SLAST: 0x00000000,
        TCD21_DADDR: 0x00000000,
        TCD21_DOFF: 0x00000000,
        TCD21_CITER_ELINK: 0x00000000,
        TCD21_DLASTSGA: 0x00000000,
        TCD21_CSR: 0x00000000,
        TCD21_BITER_ELINK: 0x00000000,
        TCD22_SADDR: 0x00000000,
        TCD22_SOFF: 0x00000000,
        TCD22_ATTR: 0x00000000,
        TCD22_NBYTES_ML: 0x00000000,
        TCD22_SLAST: 0x00000000,
        TCD22_DADDR: 0x00000000,
        TCD22_DOFF: 0x00000000,
        TCD22_CITER_ELINK: 0x00000000,
        TCD22_DLASTSGA: 0x00000000,
        TCD22_CSR: 0x00000000,
        TCD22_BITER_ELINK: 0x00000000,
        TCD23_SADDR: 0x00000000,
        TCD23_SOFF: 0x00000000,
        TCD23_ATTR: 0x00000000,
        TCD23_NBYTES_ML: 0x00000000,
        TCD23_SLAST: 0x00000000,
        TCD23_DADDR: 0x00000000,
        TCD23_DOFF: 0x00000000,
        TCD23_CITER_ELINK: 0x00000000,
        TCD23_DLASTSGA: 0x00000000,
        TCD23_CSR: 0x00000000,
        TCD23_BITER_ELINK: 0x00000000,
        TCD24_SADDR: 0x00000000,
        TCD24_SOFF: 0x00000000,
        TCD24_ATTR: 0x00000000,
        TCD24_NBYTES_ML: 0x00000000,
        TCD24_SLAST: 0x00000000,
        TCD24_DADDR: 0x00000000,
        TCD24_DOFF: 0x00000000,
        TCD24_CITER_ELINK: 0x00000000,
        TCD24_DLASTSGA: 0x00000000,
        TCD24_CSR: 0x00000000,
        TCD24_BITER_ELINK: 0x00000000,
        TCD25_SADDR: 0x00000000,
        TCD25_SOFF: 0x00000000,
        TCD25_ATTR: 0x00000000,
        TCD25_NBYTES_ML: 0x00000000,
        TCD25_SLAST: 0x00000000,
        TCD25_DADDR: 0x00000000,
        TCD25_DOFF: 0x00000000,
        TCD25_CITER_ELINK: 0x00000000,
        TCD25_DLASTSGA: 0x00000000,
        TCD25_CSR: 0x00000000,
        TCD25_BITER_ELINK: 0x00000000,
        TCD26_SADDR: 0x00000000,
        TCD26_SOFF: 0x00000000,
        TCD26_ATTR: 0x00000000,
        TCD26_NBYTES_ML: 0x00000000,
        TCD26_SLAST: 0x00000000,
        TCD26_DADDR: 0x00000000,
        TCD26_DOFF: 0x00000000,
        TCD26_CITER_ELINK: 0x00000000,
        TCD26_DLASTSGA: 0x00000000,
        TCD26_CSR: 0x00000000,
        TCD26_BITER_ELINK: 0x00000000,
        TCD27_SADDR: 0x00000000,
        TCD27_SOFF: 0x00000000,
        TCD27_ATTR: 0x00000000,
        TCD27_NBYTES_ML: 0x00000000,
        TCD27_SLAST: 0x00000000,
        TCD27_DADDR: 0x00000000,
        TCD27_DOFF: 0x00000000,
        TCD27_CITER_ELINK: 0x00000000,
        TCD27_DLASTSGA: 0x00000000,
        TCD27_CSR: 0x00000000,
        TCD27_BITER_ELINK: 0x00000000,
        TCD28_SADDR: 0x00000000,
        TCD28_SOFF: 0x00000000,
        TCD28_ATTR: 0x00000000,
        TCD28_NBYTES_ML: 0x00000000,
        TCD28_SLAST: 0x00000000,
        TCD28_DADDR: 0x00000000,
        TCD28_DOFF: 0x00000000,
        TCD28_CITER_ELINK: 0x00000000,
        TCD28_DLASTSGA: 0x00000000,
        TCD28_CSR: 0x00000000,
        TCD28_BITER_ELINK: 0x00000000,
        TCD29_SADDR: 0x00000000,
        TCD29_SOFF: 0x00000000,
        TCD29_ATTR: 0x00000000,
        TCD29_NBYTES_ML: 0x00000000,
        TCD29_SLAST: 0x00000000,
        TCD29_DADDR: 0x00000000,
        TCD29_DOFF: 0x00000000,
        TCD29_CITER_ELINK: 0x00000000,
        TCD29_DLASTSGA: 0x00000000,
        TCD29_CSR: 0x00000000,
        TCD29_BITER_ELINK: 0x00000000,
        TCD30_SADDR: 0x00000000,
        TCD30_SOFF: 0x00000000,
        TCD30_ATTR: 0x00000000,
        TCD30_NBYTES_ML: 0x00000000,
        TCD30_SLAST: 0x00000000,
        TCD30_DADDR: 0x00000000,
        TCD30_DOFF: 0x00000000,
        TCD30_CITER_ELINK: 0x00000000,
        TCD30_DLASTSGA: 0x00000000,
        TCD30_CSR: 0x00000000,
        TCD30_BITER_ELINK: 0x00000000,
        TCD31_SADDR: 0x00000000,
        TCD31_SOFF: 0x00000000,
        TCD31_ATTR: 0x00000000,
        TCD31_NBYTES_ML: 0x00000000,
        TCD31_SLAST: 0x00000000,
        TCD31_DADDR: 0x00000000,
        TCD31_DOFF: 0x00000000,
        TCD31_CITER_ELINK: 0x00000000,
        TCD31_DLASTSGA: 0x00000000,
        TCD31_CSR: 0x00000000,
        TCD31_BITER_ELINK: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMA0_TAKEN: bool = false;

    /// Safe access to DMA0
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA0_TAKEN {
                None
            } else {
                DMA0_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA0
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA0_TAKEN && inst.addr == INSTANCE.addr {
                DMA0_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMA0
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA0_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMA0
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA0: *const RegisterBlock = 0x400e8000 as *const _;

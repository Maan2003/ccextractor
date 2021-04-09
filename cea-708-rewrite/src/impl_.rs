use crate::{
    dtvcc_get_internal_from_G0, dtvcc_get_internal_from_G1, dtvcc_get_internal_from_G2,
    dtvcc_get_internal_from_G3,
};

pub fn dtvcc_get_internal_from_g0(g0_char: u8) -> u8 {
    dtvcc_get_internal_from_G0(g0_char)
}

pub fn dtvcc_get_internal_from_g1(g1_char: u8) -> u8 {
    dtvcc_get_internal_from_G1(g1_char)
}

pub fn dtvcc_get_internal_from_g2(g2_char: u8) -> u8 {
    dtvcc_get_internal_from_G2(g2_char)
}

pub fn dtvcc_get_internal_from_g3(g3_char: u8) -> u8 {
    dtvcc_get_internal_from_G3(g3_char)
}

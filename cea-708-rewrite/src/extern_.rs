/********************************************************
256 BYTES IS ENOUGH FOR ALL THE SUPPORTED CHARACTERS IN
EIA-708, SO INTERNALLY WE USE THIS TABLE (FOR CONVENIENCE)

00-1F -> Characters that are in the G2 group in 20-3F,
         except for 06, which is used for the closed captions
         sign "CC" which is defined in group G3 as 00. (this
         is by the article 33).
20-7F -> Group G0 as is - corresponds to the ASCII code
80-9F -> Characters that are in the G2 group in 60-7F
         (there are several blank characters here, that's OK)
A0-FF -> Group G1 as is - non-English characters and symbols
*/

#![allow(non_snake_case)]

#[no_mangle]
pub extern "C" fn dtvcc_get_internal_from_G0(g0_char: u8) -> u8 {
    g0_char
}

#[no_mangle]
pub extern "C" fn dtvcc_get_internal_from_G1(g1_char: u8) -> u8 {
    g1_char
}

// TODO: Probably not right
// G2: Extended Control Code Set 1
#[no_mangle]
pub extern "C" fn dtvcc_get_internal_from_G2(g2_char: u8) -> u8 {
    if g2_char >= 0x20 && g2_char <= 0x3F {
        g2_char - 0x20
    } else if g2_char >= 0x60 && g2_char <= 0x7F {
        g2_char + 0x20
    } else {
        // Rest unmapped, so we return a blank space
        0x20
    }
}

// TODO: Probably not right
// G3: Future Characters and Icon Expansion
#[no_mangle]
pub extern "C" fn dtvcc_get_internal_from_G3(g3_char: u8) -> u8 {
    if g3_char == 0xa0 {
        // The "CC" (closed captions) sign
        0x06
    } else {
        // Rest unmapped, so we return a blank space
        0x20
    }
}

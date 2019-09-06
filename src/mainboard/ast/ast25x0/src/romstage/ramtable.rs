#![no_std]

static TIME_TABLE_DDR3_1333: [u32; 17] = [
    0x53503C37, //       @ 0x010
    0xF858D47F, //       @ 0x014
    0x00010000, //       @ 0x018
    0x00000000, //       @ 0x020
    0x00000000, //       @ 0x024
    0x02101C60, //       @ 0x02C
    0x00000040, //       @ 0x030
    0x00000020, //       @ 0x214
    0x02001000, //       @ 0x2E0
    0x0C000085, //       @ 0x2E4
    0x000BA018, //       @ 0x2E8
    0x2CB92104, //       @ 0x2EC
    0x07090407, //       @ 0x2F0
    0x81000700, //       @ 0x2F4
    0x0C400800, //       @ 0x2F8
    0x7F5E3A27, //       @ tRFC
    0x00005B80, //       @ PLL
];
static TIME_TABLE_DDR3_1600: [u32; 17] = [
    0x64604D38, //       @ 0x010
    0x29690599, //       @ 0x014
    0x00000300, //       @ 0x018
    0x00000000, //       @ 0x020
    0x00000000, //       @ 0x024
    0x02181E70, //       @ 0x02C
    0x00000040, //       @ 0x030
    0x00000024, //       @ 0x214
    0x02001300, //       @ 0x2E0
    0x0E0000A0, //       @ 0x2E4
    0x000E001B, //       @ 0x2E8
    0x35B8C105, //       @ 0x2EC
    0x08090408, //       @ 0x2F0
    0x9B000800, //       @ 0x2F4
    0x0E400A00, //       @ 0x2F8
    0x9971452F, //       @ tRFC
    0x000071C1, //       @ PLL
];
static TIME_TABLE_DDR4_1333: [u32; 17] = [
    0x53503D26, //       @ 0x010
    0xE878D87F, //       @ 0x014
    0x00019000, //       @ 0x018
    0x08000000, //       @ 0x020
    0x00000400, //       @ 0x024
    0x00000200, //       @ 0x02C
    0x00000101, //       @ 0x030
    0x00000020, //       @ 0x214
    0x03002200, //       @ 0x2E0
    0x0C000085, //       @ 0x2E4
    0x000BA01A, //       @ 0x2E8
    0x2CB92106, //       @ 0x2EC
    0x07060606, //       @ 0x2F0
    0x81000700, //       @ 0x2F4
    0x0C400800, //       @ 0x2F8
    0x7F5E3A3A, //       @ tRFC
    0x00005B80, //       @ PLL
];
static TIME_TABLE_DDR4_1600: [u32; 17] = [
    0x63604E37, //       @ 0x010
    0xE97AFA99, //       @ 0x014
    0x00019000, //       @ 0x018
    0x08000000, //       @ 0x020
    0x00000400, //       @ 0x024
    0x00000410, //       @ 0x02C
    //#ifdef CONFIG_DDR5_SUPPORT_HYNIX
    //	0x030     , //        @ ODT = 48 ohm
    //#else
    0x030, //        @ ODT = 60 ohm
    //#endif
    0x00000024, //       @ 0x214
    0x03002900, //       @ 0x2E0
    0x0E0000A0, //       @ 0x2E4
    0x000E001C, //       @ 0x2E8
    0x35B8C106, //       @ 0x2EC
    0x08080607, //       @ 0x2F0
    0x9B000900, //       @ 0x2F4
    0x0E400A00, //       @ 0x2F8
    0x99714545, //       @ tRFC
    0x000071C1, //       @ PLL
];

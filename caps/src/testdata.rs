pub fn test_data() -> Vec<&'static [u8]> {
    [
        // monitors I have lying around
        &b"(prot(monitor)type(lcd)27UD58cmds(01 02 03 0C E3 F3)vcp(02 04 05 08 10 12 14(05 08 0B ) 16 18 1A 52 60( 11 12 0F 10) AC AE B2 B6 C0 C6 C8 C9 D6(01 04) DF 62 8D F4 F5(01 02) F6(00 01 02) 4D 4E 4F 15(01 06 11 13 14 28 29 32 48) F7(00 01 02 03) F8(00 01) F9 E4 E5 E6 E7 E8 E9 EA EB EF FD(00 01) FE(00 01 02) FF)mccs_ver(2.1)mswhql(1))"[..],
        &b"(prot(monitor)type(LCD)model(ACER)cmds(01 02 03 07 0C E3 F3)vcp(02 04 05 08 0B 10 12 14(05 08 0B) 16 18 1A 52 60(01 03 11) 6C 6E 70 AC AE B2 B6 C6 C8 C9 CC(01 02 03 04 05 06 08 09 0A 0C 0D 14 16 1E) D6(01 05) DF)mswhql(1)asset_eep(40)mccs_ver(2.0))"[..],
        // example from ddcutil
        &b"(prot(monitor)type(LED)model(25UM65)cmds(01 02 03 0C E3 F3)vcp(0203(10 00)0405080B0C101214(05 07 08 0B) 16181A5260(03 04)6C6E7087ACAEB6C0C6C8C9D6(01 04)DFE4E5E6E7E8E9EAEBED(00 10 20 40)EE(00 01)FE(01 02 03)FF)mswhql(1)mccs_ver(2.1))"[..],
        // example from MCCS spec v2.2a
        &b"Prot(display) type(lcd) model(xxxxx) cmds(xxxxx) vcp(02 03 10 12 C8 DC(00 01 02 03 07) DF)mccs_ver(2.2) window1(type (PIP) area(25 25 1895 1175) max(640 480) min(10 10) window(10))vcpname(10(Brightness))"[..],
        // example from access bus section 7
        &br"vcpname(14((9300 6500 5500))44(Rotate)80(Do\x20this(On Off))82(Fixit))"[..],
        // above example with matching vcp() section
        &br"vcp(14(010203)448082)vcpname(14((9300 6500 5500))44(Rotate)80(Do\x20this(On Off))82(Fixit))"[..],
        // tagged length with bracket and invalid utf8 seems like a worst case scenario here:
        &b"edid bin(3(\xff) ))vdif bin(3 (abc))unknown bin(2(ab))"[..],
        // samples gathered from various online sources
        &b"(prot(monitor)type(lcd)model(p2317h)cmds(01 02 03 07 0c e3 f3)vcp(02 04 05 08 10 12 14(05 08 0b 0c) 16 18 1a 52 60(01 0f 11) aa(01 02) ac ae b2 b6 c6 c8 c9 d6(01 04 05) dc(00 02 03 05) df e0 e1 e2(00 1d 01 02 04 0e 12 14) f0(0c) f1 f2 fd)mswhql(1)asset_eep(40)mccs_ver(2.1))"[..],
        &b"(prot(monitor)type(lcd)model(U3011)cmds(01 02 03 07 0C E3 F3)vcp(02 04 05 06 08 10 12 14(01 05 08 0B 0C) 16 18 1A 52 60(01 03 04 0C 0F 11 12) AC AE B2 B6 C6 C8 C9 D6(01 04 05) DC(00 02 03 04 05) DF FD)mccs_ver(2.1)mswhql(1))"[..],
        &b"(prot(monitor)type(lcd)model(W2413)cmds(01 02 03 07 0C 4E F3 E3)vcp(02 04 05 08 0B 0C 10 12 14(05 06 08 0B) 16 18 1A 52 AC AE B2 B6 C0 C6 C8 C9 CC(02 03 04 05 06 09 0A 0D 12 14 1E) D6(01 05) DF 60(01 03 11) 62 8D)mswhql(1)mccs_ver(2.0)asset_eep(32)mpu_ver(01))"[..],
        &b"(prot(monitor)type(LCD)model(Plum)mccs_ver(2.0)vcp(04 05 06 08 0E 10 12 14(02 03 0A 0B) 16 18 1A 1E 20 30 3E 60(01 03 05) 87 B0(01 02) B6 C6 C8 C9 D6(01 04) DC(01 02 03 04 05 06 F0 F2 F9 FA FB) DB(00 04 FE) DF E8(00 07 09 0A FE) E9 EA(00 01 02 03 04) EC(00 01 02 03 04 05) F0(00 01 02 03) F2 F6 F7(00 01 02 03) )mswhql(1))"[..],
        &b"(prot(monitor)type(LCD)model(AOC)cmds(01 02 03 07 0C E3 F3)vcp(02 04 05 06 08 0B 0C 10 12 14(01 02 04 05 08) 16 18 1A 52 60(01 03 11) 87 AC AE B2 B6 C6 C8 CA CC(01 02 03 04 06 0A 0D) D6(01 04) DF FD FF)mswhql(1)asset_eep(40)mccs_ver(2.2))"[..],
    ].iter().cloned().collect()
}

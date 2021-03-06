extern {
    fn error(
        __status : i32, __errnum : i32, __format : *const u8, ...
    );
    fn strcpy(__dest : *mut u8, __src : *const u8) -> *mut u8;
    fn strlen(__s : *const u8) -> usize;
    fn xmalloc(s : usize) -> *mut ::std::os::raw::c_void;
}

static mut shifts
    : [u8; 256]
    = [   0u8,
          1u8,
          2u8,
          3u8,
          4u8,
          5u8,
          6u8,
          7u8,
          8u8,
          9u8,
          10u8,
          11u8,
          12u8,
          13u8,
          14u8,
          15u8,
          16u8,
          17u8,
          18u8,
          19u8,
          20u8,
          21u8,
          22u8,
          23u8,
          24u8,
          25u8,
          26u8,
          27u8,
          28u8,
          29u8,
          30u8,
          31u8,
          114u8,
          120u8,
          53u8,
          79u8,
          96u8,
          109u8,
          72u8,
          108u8,
          70u8,
          64u8,
          76u8,
          67u8,
          116u8,
          74u8,
          68u8,
          87u8,
          111u8,
          52u8,
          75u8,
          119u8,
          49u8,
          34u8,
          82u8,
          81u8,
          95u8,
          65u8,
          112u8,
          86u8,
          118u8,
          110u8,
          122u8,
          105u8,
          41u8,
          57u8,
          83u8,
          43u8,
          46u8,
          102u8,
          40u8,
          89u8,
          38u8,
          103u8,
          45u8,
          50u8,
          42u8,
          123u8,
          91u8,
          35u8,
          125u8,
          55u8,
          54u8,
          66u8,
          124u8,
          126u8,
          59u8,
          47u8,
          92u8,
          71u8,
          115u8,
          78u8,
          88u8,
          107u8,
          106u8,
          56u8,
          36u8,
          121u8,
          117u8,
          104u8,
          101u8,
          100u8,
          69u8,
          73u8,
          99u8,
          63u8,
          94u8,
          93u8,
          39u8,
          37u8,
          61u8,
          48u8,
          58u8,
          113u8,
          32u8,
          90u8,
          44u8,
          98u8,
          60u8,
          51u8,
          33u8,
          97u8,
          62u8,
          77u8,
          84u8,
          80u8,
          85u8,
          223u8,
          225u8,
          216u8,
          187u8,
          166u8,
          229u8,
          189u8,
          222u8,
          188u8,
          141u8,
          249u8,
          148u8,
          200u8,
          184u8,
          136u8,
          248u8,
          190u8,
          199u8,
          170u8,
          181u8,
          204u8,
          138u8,
          232u8,
          218u8,
          183u8,
          255u8,
          234u8,
          220u8,
          247u8,
          213u8,
          203u8,
          226u8,
          193u8,
          174u8,
          172u8,
          228u8,
          252u8,
          217u8,
          201u8,
          131u8,
          230u8,
          197u8,
          211u8,
          145u8,
          238u8,
          161u8,
          179u8,
          160u8,
          212u8,
          207u8,
          221u8,
          254u8,
          173u8,
          202u8,
          146u8,
          224u8,
          151u8,
          140u8,
          196u8,
          205u8,
          130u8,
          135u8,
          133u8,
          143u8,
          246u8,
          192u8,
          159u8,
          244u8,
          239u8,
          185u8,
          168u8,
          215u8,
          144u8,
          139u8,
          165u8,
          180u8,
          157u8,
          147u8,
          186u8,
          214u8,
          176u8,
          227u8,
          231u8,
          219u8,
          169u8,
          175u8,
          156u8,
          206u8,
          198u8,
          129u8,
          164u8,
          150u8,
          210u8,
          154u8,
          177u8,
          134u8,
          127u8,
          182u8,
          128u8,
          158u8,
          208u8,
          162u8,
          132u8,
          167u8,
          209u8,
          149u8,
          241u8,
          153u8,
          251u8,
          237u8,
          236u8,
          171u8,
          195u8,
          243u8,
          233u8,
          253u8,
          240u8,
          194u8,
          250u8,
          191u8,
          155u8,
          142u8,
          137u8,
          245u8,
          235u8,
          163u8,
          242u8,
          178u8,
          152u8
      ];

#[no_mangle]
pub unsafe extern fn scramble(mut str : *mut u8) -> *mut u8 {
    let mut i : i32;
    let mut s : *mut u8;
    s = xmalloc(
            strlen(str as (*const u8)).wrapping_add(2usize)
        ) as (*mut u8);
    *s.offset(0isize) = b'A';
    strcpy(s.offset(1isize),str as (*const u8));
    i = 1i32;
    'loop1: loop {
        if *s.offset(i as (isize)) == 0 {
            break;
        }
        *s.offset(i as (isize)) = shifts[
                                      *s.offset(i as (isize)) as (usize)
                                  ];
        i = i + 1;
    }
    s
}

#[no_mangle]
pub unsafe extern fn descramble(mut str : *mut u8) -> *mut u8 {
    let mut s : *mut u8;
    let mut i : i32;
    if *str.offset(0isize) as (i32) != b'A' as (i32) {
        error(
            1i32,
            0i32,
            (*b"descramble: unknown scrambling method\0").as_ptr()
        );
    }
    s = scramble(str.offset(1isize));
    i = 0i32;
    'loop3: loop {
        if *s.offset(i as (isize)) == 0 {
            break;
        }
        *s.offset(i as (isize)) = *s.offset((i + 1i32) as (isize));
        i = i + 1;
    }
    s
}

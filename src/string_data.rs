use std::convert::TryInto;



#[derive(Debug, Clone, Copy)]
pub struct OffsetInformation {
    pub pos_in_bucket: u32,
    // length of string == bucket
    // pub bucket: u32,
}

impl OffsetInformation {
    #[inline]
    pub fn is_null(&self) -> bool {
        self.pos_in_bucket == u32::max_value()
    }
}

impl Default for OffsetInformation {
    #[inline]
    fn default() -> OffsetInformation {
        OffsetInformation{
            pos_in_bucket: u32::max_value(),
            // bucket: u32::max_value(),
        }
    }
}


#[derive(Debug, Default)]
struct S1([u8; 1]);
#[repr(align(2))]
#[derive(Debug, Default)]
struct S2([u8; 2]);
#[repr(align(4))]
#[derive(Debug, Default)]
struct S3([u8; 3]);
#[repr(align(4))]
#[derive(Debug, Default)]
struct S4([u8; 4]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S5([u8; 5]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S6([u8; 6]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S7([u8; 7]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S8([u8; 8]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S9([u8; 9]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S10([u8; 10]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S11([u8; 11]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S12([u8; 12]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S13([u8; 13]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S14([u8; 14]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S15([u8; 15]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S16([u8; 16]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S17([u8; 17]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S18([u8; 18]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S19([u8; 19]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S20([u8; 20]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S21([u8; 21]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S22([u8; 22]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S23([u8; 23]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S24([u8; 24]);
#[repr(align(8))]
#[derive(Debug, Default)]
struct S25([u8; 25]);

#[derive(Debug, Default)]
pub struct StringData {
    // string_len1:  Vec<S1>,
    // string_len2:  Vec<S2>,
    // string_len3:  Vec<S3>,
    // string_len4:  Vec<S4>,
    // string_len5:  Vec<S5>,
    // string_len6:  Vec<S6>,
    // string_len7:  Vec<S7>,
    string_len8:  Vec<S8>,
    // string_len9:  Vec<S9>,
    // string_len10: Vec<S10>,
    // string_len11: Vec<S11>,
    // string_len12: Vec<S12>,
    // string_len13: Vec<S13>,
    // string_len14: Vec<S14>,
    // string_len15: Vec<S15>,
    // string_len16: Vec<S16>,
    // string_len17: Vec<S17>,
    // string_len18: Vec<S18>,
    // string_len19: Vec<S19>,
    // string_len20: Vec<S20>,
    // string_len21: Vec<S21>,
    // string_len22: Vec<S22>,
    // string_len23: Vec<S23>,
    // string_len24: Vec<S24>,
    // string_len_other: Vec<u8>,
}

impl StringData {

    pub fn dbg(&self) {

        // dbg!(&self.string_len1.len());
        // dbg!(&self.string_len2.len());
        // dbg!(&self.string_len3.len());
        // dbg!(&self.string_len4.len());
        // dbg!(&self.string_len5.len());
        // dbg!(&self.string_len6.len());
        // dbg!(&self.string_len7.len());
        // dbg!(&self.string_len8.len());
        // dbg!(&self.string_len9.len());
        // dbg!(&self.string_len10.len());
        // dbg!(&self.string_len11.len());
        // dbg!(&self.string_len12.len());
        // dbg!(&self.string_len13.len());
        // dbg!(&self.string_len14.len());
        // dbg!(&self.string_len15.len());
        // dbg!(&self.string_len16.len());
        // dbg!(&self.string_len17.len());
        // dbg!(&self.string_len18.len());
        // dbg!(&self.string_len19.len());
        // dbg!(&self.string_len20.len());
        // dbg!(&self.string_len21.len());
        // dbg!(&self.string_len22.len());
        // dbg!(&self.string_len23.len());
        // dbg!(&self.string_len24.len());
        // dbg!(&self.string_len_other.len());

    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {

    }

    #[inline]
    pub fn read_string(&self, offsets: OffsetInformation) -> &str {

        let pos_in_bucket = offsets.pos_in_bucket as usize;
        // let bytes = match offsets.bucket {
        //     1 => &self.string_len1[pos_in_bucket].0 as &[u8],
        //     2 => &self.string_len2[pos_in_bucket].0 as &[u8],
        //     3 => &self.string_len3[pos_in_bucket].0 as &[u8],
        //     4 => &self.string_len4[pos_in_bucket].0 as &[u8],
        //     5 => &self.string_len5[pos_in_bucket].0 as &[u8],
        //     6 => &self.string_len6[pos_in_bucket].0 as &[u8],
        //     7 => &self.string_len7[pos_in_bucket].0 as &[u8],
        //     8 => &self.string_len8[pos_in_bucket].0 as &[u8],
        //     9 => &self.string_len9[pos_in_bucket].0 as &[u8],
        //     10 => &self.string_len10[pos_in_bucket].0 as &[u8],
        //     11 => &self.string_len11[pos_in_bucket].0 as &[u8],
        //     12 => &self.string_len12[pos_in_bucket].0 as &[u8],
        //     13 => &self.string_len13[pos_in_bucket].0 as &[u8],
        //     14 => &self.string_len14[pos_in_bucket].0 as &[u8],
        //     15 => &self.string_len15[pos_in_bucket].0 as &[u8],
        //     16 => &self.string_len16[pos_in_bucket].0 as &[u8],
        //     17 => &self.string_len17[pos_in_bucket].0 as &[u8],
        //     18 => &self.string_len18[pos_in_bucket].0 as &[u8],
        //     19 => &self.string_len19[pos_in_bucket].0 as &[u8],
        //     20 => &self.string_len20[pos_in_bucket].0 as &[u8],
        //     21 => &self.string_len21[pos_in_bucket].0 as &[u8],
        //     22 => &self.string_len22[pos_in_bucket].0 as &[u8],
        //     23 => &self.string_len23[pos_in_bucket].0 as &[u8],
        //     24 => &self.string_len24[pos_in_bucket].0 as &[u8],
        //     _ => {
        //         let length_string_bytes: [u8;4] = self.string_len_other[pos_in_bucket..pos_in_bucket + 4].try_into().unwrap();
        //         let length_string = u32::from_le_bytes(length_string_bytes);
        //         &self.string_len_other[pos_in_bucket + 4 .. pos_in_bucket + 4 + length_string as usize]
        //     },
        // };


        unsafe {
            let bytes = &self.string_len8.get_unchecked(pos_in_bucket).0 as &[u8];
            std::str::from_utf8_unchecked(bytes)
        }
    }

    pub fn insert(&mut self, el: &str) -> OffsetInformation {
        // let pos_in_bucket = match el.len() {
        //     1 => {&self.string_len1.push(S1(el.as_bytes().try_into().unwrap())); self.string_len1.len() - 1},
        //     2 => {&self.string_len2.push(S2(el.as_bytes().try_into().unwrap())); self.string_len2.len() - 1},
        //     3 => {&self.string_len3.push(S3(el.as_bytes().try_into().unwrap())); self.string_len3.len() - 1},
        //     4 => {&self.string_len4.push(S4(el.as_bytes().try_into().unwrap())); self.string_len4.len() - 1},
        //     5 => {&self.string_len5.push(S5(el.as_bytes().try_into().unwrap())); self.string_len5.len() - 1},
        //     6 => {&self.string_len6.push(S6(el.as_bytes().try_into().unwrap())); self.string_len6.len() - 1},
        //     7 => {&self.string_len7.push(S7(el.as_bytes().try_into().unwrap())); self.string_len7.len() - 1},
        //     8 => {&self.string_len8.push(S8(el.as_bytes().try_into().unwrap())); self.string_len8.len() - 1},
        //     9 => {&self.string_len9.push(S9(el.as_bytes().try_into().unwrap())); self.string_len9.len() - 1},
        //     10 =>{ &self.string_len10.push(S10(el.as_bytes().try_into().unwrap())); self.string_len10.len() - 1},
        //     11 =>{ &self.string_len11.push(S11(el.as_bytes().try_into().unwrap())); self.string_len11.len() - 1},
        //     12 =>{ &self.string_len12.push(S12(el.as_bytes().try_into().unwrap())); self.string_len12.len() - 1},
        //     13 =>{ &self.string_len13.push(S13(el.as_bytes().try_into().unwrap())); self.string_len13.len() - 1},
        //     14 =>{ &self.string_len14.push(S14(el.as_bytes().try_into().unwrap())); self.string_len14.len() - 1},
        //     15 =>{ &self.string_len15.push(S15(el.as_bytes().try_into().unwrap())); self.string_len15.len() - 1},
        //     16 =>{ &self.string_len16.push(S16(el.as_bytes().try_into().unwrap())); self.string_len16.len() - 1},
        //     17 =>{ &self.string_len17.push(S17(el.as_bytes().try_into().unwrap())); self.string_len17.len() - 1},
        //     18 =>{ &self.string_len18.push(S18(el.as_bytes().try_into().unwrap())); self.string_len18.len() - 1},
        //     19 =>{ &self.string_len19.push(S19(el.as_bytes().try_into().unwrap())); self.string_len19.len() - 1},
        //     20 =>{ &self.string_len20.push(S20(el.as_bytes().try_into().unwrap())); self.string_len20.len() - 1},
        //     21 =>{ &self.string_len21.push(S21(el.as_bytes().try_into().unwrap())); self.string_len21.len() - 1},
        //     22 =>{ &self.string_len22.push(S22(el.as_bytes().try_into().unwrap())); self.string_len22.len() - 1},
        //     23 =>{ &self.string_len23.push(S23(el.as_bytes().try_into().unwrap())); self.string_len23.len() - 1},
        //     24 =>{ &self.string_len24.push(S24(el.as_bytes().try_into().unwrap())); self.string_len24.len() - 1},
        //     _ => {
        //             let pos = self.string_len_other.len();
        //             let len_as_bytes = (el.len() as u32).to_le_bytes();
        //             self.string_len_other.extend_from_slice(&len_as_bytes);
        //             self.string_len_other.extend_from_slice(el.as_bytes());
        //             pos
        //     },
        // };


        let pos_in_bucket = {&self.string_len8.push(S8(el.as_bytes().try_into().unwrap())); self.string_len8.len() - 1};
        
        OffsetInformation{
            pos_in_bucket: pos_in_bucket as u32,
            // bucket: el.len() as u32,


        }
    }
}



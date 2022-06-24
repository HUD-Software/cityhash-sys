mod lipsum;

static CITY_HASH_32_RESULTS: [u32; 256] = [
    0xdc56d17a, 0xc0a92754, 0x2a1678b6, 0xd7c992e2, 0x616e1132, 0xfe6e37d4, 0x5145897e, 0xcfea845d,
    0xeb0fd2d6, 0x7cd3d6e0, 0x14e52250, 0x931da128, 0xde42ef1c, 0x8add7404, 0x69976bd0, 0xcacd0542,
    0x17aebf87, 0x1e9bcbda, 0xba680c4b, 0xf7cfbfda, 0xc41a2a96, 0x4a3f2b87, 0x4b7dd7b7, 0x1de0e4f5,
    0x60cf6aa4, 0x2e6ddf78, 0x17a5df60, 0x100139a4, 0xef678131, 0xc158707d, 0xd3d91d57, 0xe4345328,
    0x68943315, 0xe14e6d9e, 0x33f96086, 0x7549c70a, 0x43f74d13, 0x26911fcc, 0xc4c58416, 0xa104234,
    0x5042df8c, 0x7044f7ca, 0x055dbbbf, 0xce39467e, 0xfdb76981, 0x716e5ed0, 0x05d0c428, 0x1950b972,
    0x99fa0a24, 0xf531d568, 0x0789e78e, 0x56cd4f9b, 0x2b0a28b4, 0x22498538, 0x2691989e, 0x82ee00db,
    0xdc77f8be, 0xfeb09805, 0x05ad5eea, 0xc449c697, 0x8ebc55ce, 0xc119319b, 0x42561cd0, 0xefb0d898,
    0x53d2a4c3, 0x7a49977d, 0x1a7a3b62, 0xc9cbb478, 0xd04a9ff8, 0x1607f8fa, 0x1f71fd8b, 0xd44629b3,
    0x5d709512, 0x9cd509c2, 0x8db9c50a, 0x9497192a, 0xa0090ca2, 0xbe3f0434, 0xfe2c61ea, 0x4ea7f018,
    0xdd027e4a, 0x7e8714f9, 0x812718c7, 0x27319a37, 0x1bec91e2, 0xd7fa11f6, 0x2a4d4a55, 0xb456be6f,
    0xe417cd30, 0x5728c0fb, 0x5b566ca8, 0x7b146f39, 0x0c197980, 0x28add1ff, 0x3e7095b3, 0xeb0ab40f,
    0x611a1bec, 0xc60b1d1f, 0xfb75ed9f, 0xe5c2ed21, 0x56e258e5, 0xb9b1f97a, 0xcbbab8b3, 0x1c8bcd5a,
    0x360ff812, 0x593d2794, 0xe3b5e900, 0x79b282bc, 0x91559ebf, 0x64af1417, 0x42ad606f, 0xe9101891,
    0xaa82f3b0, 0xfb4bd972, 0xa8c7ae49, 0xce7a4050, 0x94a56c68, 0xf2919a58, 0x36cb6074, 0x127e2749,
    0xe00d134e, 0x4221cee7, 0x573feb8c, 0x14df4883, 0xdc8c23fb, 0xbdbf5cba, 0x81dc28d9, 0x558331a9,
    0x5321e430, 0xd737c0c8, 0x6c4e7352, 0x985deedc, 0xbee2fac2, 0x61f57165, 0xe0ea33b6, 0x80b29f25,
    0x37775178, 0x418437a1, 0x6c8b569e, 0xf770d828, 0x6fabe476, 0x6ec19d97, 0xcb410b3c, 0xd1c84844,
    0x889018d1, 0x415435fe, 0xe4a4ec1d, 0xfa49fa16, 0x97eadb32, 0xa0bbfb93, 0xd9fe5c31, 0x6201ccad,
    0xdc04203c, 0x35a284fe, 0x04e8d35b, 0x136ab950, 0xc4b4c7f6, 0x479df6ff, 0x2a4964e1, 0xbebd4da7,
    0xa048bb31, 0xb6a95161, 0xb88fbb76, 0xeb3804d8, 0x81878252, 0x93453547, 0x0d9cf26f, 0x8a8848cd,
    0xb2e83544, 0xbb2079bf, 0xbff1e5bb, 0x9313a579, 0x2f35813f, 0xba7d9b0e, 0x8c116f5c, 0x2690b5df,
    0x19d487af, 0x7245211e, 0x9b41b711, 0x246fa530, 0xc42c933a, 0x2dc64f76, 0x51910402, 0xb59e8a94,
    0xcf299383, 0xf63a92a4, 0xa6e69cdb, 0xdef3a2ca, 0xd488d2c6, 0x3f54a353, 0x42ecb6fa, 0xf0246bfd,
    0xbfe4087c, 0xca89cd7e, 0x0fae767d, 0x730608c9, 0xa8ccc638, 0x31eb911f, 0x04da366e, 0x35b52ca6,
    0x62d5b95a, 0xe0cd6d86, 0x710912ee, 0xa9a0e174, 0x7c1b479e, 0x664bf777, 0x1cd8f425, 0x195ed250,
    0x91b5afee, 0x419f49ce, 0xf4bbf69a, 0x890715ec, 0xf781eb91, 0x245c4d39, 0xee4e8908, 0xf086cba5,
    0x196c8a16, 0xd3d419b7, 0xd293cba7, 0x6aaa38e8, 0xe54dca45, 0x5ba0b60f, 0xafabbd81, 0xea4531cd,
    0x67a4a588, 0x803d4c59, 0x994a71bb, 0x369a676c, 0xe99cc2c4, 0xff00a6d8, 0xe96df54e, 0xfa5f33a0,
    0x49194151, 0x035ead1a, 0xcc8b2d4b, 0x2fa7895d, 0x9deddbc0, 0x1adaffbd, 0x9ad66495, 0x6b8c1d02,
    0x3a9f56d0, 0xb9ccb4fd, 0x1fb0ee25, 0x982b4896, 0x08dd93d5, 0x3282814b, 0x696151d2, 0x428f9244,
    0x3ef8a0ba, 0xbafc4de5, 0x4ca78146, 0xa87374f5, 0x4fa36aaf, 0x4967e1ca, 0x6491ed46, 0x966e909f,
];

#[test]
fn hash_32_from_null_ptr() {
    assert_eq!(
        cityhash_sys::city_hash_32(unsafe {
            core::slice::from_raw_parts(core::ptr::NonNull::dangling().as_ptr(), 0)
        }),
        0xDC56D17A
    );
}

#[test]
fn hash_32_no_seed() {
    let mut key = [0u8; 256];

    for index in 0..=255u8 {
        key[index as usize] = index;
        assert_eq!(
            cityhash_sys::city_hash_32(&key[0..index as usize]),
            CITY_HASH_32_RESULTS[index as usize]
        );
    }
}

#[test]
fn hash_32_from_str_lipsum() {
    assert_eq!(
        cityhash_sys::city_hash_32(lipsum::LIPSUM.as_bytes()),
        0xab8cceb7
    );
}
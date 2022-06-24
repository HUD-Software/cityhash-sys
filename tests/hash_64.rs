mod lipsum;

static CITY_HASH_64_RESULTS : [u64; 256]  = [
    0x9ae16a3b2f90404f, 0xbe6056edf5e94b54, 0xc2a04665ed038d75, 0x94a13d22e9eba49a, 
    0x82bffd898958e540, 0xb4bfa9e87732c149, 0x92fdbcd8e94a2333, 0xa2e0bff20db0a6a1, 
    0xad5a13e1e8e93b98, 0x81371e150e4ad84f, 0x9b704db2b2d6ffea, 0xf3212b3c1d803add, 
    0x9fd5df33aefc3d7d, 0xc1c5cf5c853d3c92, 0x4bd9a87bb2bb4671, 0x862a51555943bd9d,
    0x0efd25a0a34156d4, 0xbbb6a6f8f20d1f1c, 0x79f8c18b091f57e6, 0x9397ab7e5511df31,
    0x8dd7e7f1bf16a0e9, 0xebed9d4fcec9fa24, 0x28e6d435e458f020, 0x0662e334e298fddc,
    0x3f3b313dcbd16ec7, 0xb49fc64083cc4c3e, 0x1313dbdeed4e9ded, 0x427e4dbedaea19f6, 
    0xa56007058a23efa6, 0x96081fedc82adf9f, 0x48abb0e9ebd50ea7, 0xfbd950af27ef6941, 
    0x1a9d8199972cdf49, 0x46e1378cbc22daba, 0xb5bac3abe70d1522, 0x28e59da3f3008300, 
    0x0011dfc4eea43101, 0x3645772b95bd3743, 0x629946fa201ac819, 0xa5f6faba3c6fb303,
    0x36f7dd7a65e93d1c, 0xc07403048a1eeda7, 0xca4d3c89744f43a3, 0x4cf58d31ed02513e, 
    0x9e07c00bd59d7a01, 0x5e776542af3c0519, 0xfa62d2a4bc5d87c6, 0xb7f64877a70ee14a, 
    0xc7099eac443f4625, 0x60b11c741d047a11, 0x73286092ee9394c9, 0x7877a8d9b5a4b929, 
    0xd589cb2ce84723a8, 0xa85c240f3e025587, 0xc3d98ecdb6bf52cd, 0xf890c6323948a021, 
    0x96a5683630a4333f, 0xa84168edcd8646cc, 0x67f1e6f0ccfdf70e, 0x84cb84c6edffeab6,
    0xe954fc8f7968db69, 0xb29212c2d2ef6b03, 0x2f20733b20979be3, 0xaf30927a77ada6ef, 
    0xe99ab80f5ec7dca5, 0xac589c990483dd2e, 0xaf6f3ea7de248f72, 0x15452a0335f8d3ff, 
    0x87faaf9ffc00b792, 0xd735fd9242f41d1d, 0xbebcb91fd6057b44, 0x0177474a5d1ebbe6, 
    0x9273be0076008b09, 0x4ff1e9fb068a6a2b, 0xa9bd698beab91622, 0x112ef9fd43f6ab0c, 
    0xbd41cfd5fb432baf, 0x20959f28fef778b5, 0x7dc5c0f6bffa9f3a, 0x5d8d82ca24381650,
    0x4feedc7dada54816, 0x993dad25f4602235, 0x48ac17a39fa67d56, 0xa69f17df1e2b88c4,
    0x62e9f87e7484d48e, 0x86b1f8e0d03141a2, 0x4ccf1f0ffc2468c4, 0xc0828aa177219532, 
    0xffdce2c7bf331b8f, 0x6992c7d1be0fd9ca, 0xd1715e9954348633, 0xcd69d2919e30aa60, 
    0xa5f2c86dc3e7480b, 0x536f357a66399901, 0xb9da9a2379c19379, 0x364ae6782f889be4, 
    0xe3f6cd656b9c26be, 0xb1541a33562869ea, 0x72bf686a93a755af, 0xaf958d2eb9ed0dff,
    0xa47fe83e60b34cc6, 0xd61f11279fb8a7e5, 0x7ebe4b3d74386431, 0x41cc5ad24988a733, 
    0x0b87a4fb288a06c2, 0xe3c3810bf671386c, 0x7c1f3a08a8e24551, 0x8edc6a63e8b10115, 
    0xa0ce313ca06c2d01, 0x531f781e2c8264bc, 0xd2a65241d3709bbe, 0x96042182140074fd, 
    0xc2bb46ba9af19ed3, 0xe031e70d24a3eb33, 0xf9d935c7400b2407, 0x663845cc33b07f0a, 
    0xca3fc22c3bcd0200, 0x2c8dc0bcac69bd81, 0x3e49372b20c3da04, 0xbb9384101d3bbe76,
    0x612b5e5a1c296054, 0x1f5f351cc71b79a3, 0x5775c3be1a6af9fa, 0xde6a78ea8ba8dd0e, 
    0x90c6012b151e7de3, 0x8db4fd55a93183f1, 0xb4e37477c19e1ef3, 0xefd614390a7b1d95, 
    0x10b153630af1f395, 0x46be8f236f918770, 0x471b693b3226bc9f, 0xa46e7f270ac7812c,
    0xa0ac294f8aa62d68, 0xf31dd191643f41ac, 0xf2961255518716c0, 0x4059d62c19ad9e14, 
    0xe0df742546f097ac, 0xdff7feab5ca4d8b7, 0xa8957af3f869e737, 0xd44357cd3043768c,
    0xd6a0214e8245fea9, 0x5f50d9a469932ba7, 0x9f0c60bf760a9194, 0x370bcbd5b6ececed, 
    0x917922f50dea675e, 0xad2d366db2564e3c, 0xb121fb5707cdc3ff, 0x1183b019cd0033c5, 
    0x89f48ac41a6728ad, 0x6131b64dd658e8dc, 0x6e3aa5f0f424ec59, 0x2d701e8579d7970a, 
    0x3d70b9b6255a92a1, 0xb2dbd54203588d4b, 0x71a7c5fa31ddf60e, 0xd9707c56d133b51e, 
    0xa9968c6d71b9e415, 0x6a305f39728ed0c2, 0x7f6cc5b333cc4ee2, 0x23231e2aef8c2c48,
    0x45af15babe5d7c65, 0xada4d4a86da2e1ab, 0xe6258fc2db0fac5d, 0x14aa57bff770e293, 
    0x654528c6ddc74de9, 0x8b36ea8945b7ed49, 0xc6bcf2a03a78abe9, 0xcc8f0587fa2af10c, 
    0xda636827d57fa655, 0x9051b5c3b87fef61, 0xef16c25c523903d5, 0x7327f0b055f6ec29,
    0x71531d073f8b8314, 0x4d1543172054f04e, 0x3d4dc4e9ab8ba392, 0xa42bc184eb199366, 
    0x918b995f565e97e8, 0xd4c768803157baa4, 0xe638d049da45b516, 0xf1dce6e38d5f7674,
    0x48448058a3c22a6d, 0x498e30a542aac033, 0xded4e1b2f9853ba5, 0xf9c7ee3e76b38bf0, 
    0xa99afa5245d0bef8, 0x21276f9f37743e1e, 0xe34e51d2c449d0d0, 0xfca6453578ceddc1,
    0x838269263cf16f34, 0xa048360ccaffcffc, 0xc017e5cfd9ebec42, 0x214efe903049aac9, 
    0xd5ba92662e2d5074, 0xe56476a1931c992a, 0x71c47a32f8190d15, 0x348d5eaf446f1e0d, 
    0x4031a8122018a4a1, 0xe1e736a1c15db548, 0x63d193dce2ebda88, 0x2270c0be9bc5af94,
    0xf4d24e8c7493c3d8, 0x4eb48586354313fc, 0xc5520e2d63e6fed5, 0x95204130763e6755,
    0xec2fa707c89c58df, 0x73f8c1f9961bcf8c, 0xfb1755f531e55bc7, 0x09cd07d7167b6154, 
    0x80c47e3f57bd03c1, 0x72b348f7ffcb1ca8, 0xda6e47db129e8a22, 0x660b38cc131965a1, 
    0x748a65bed0389a40, 0xa58c597d5f008d19, 0x6071bf4d2700ed9d, 0x16e19ccd578efd5e, 
    0x5b053799983874da, 0x1918a56191596766, 0x27649c3904f890e1, 0x1227f1a51144baa6,
    0x7a779add715ab5e4, 0x386f3389e479c08c, 0x47808cd6db853d47, 0xcfd9c5c21c04f4e6, 
    0x029ec12f72e3d6b8, 0x638cb224e487c08e, 0xb5c95c00242919ce, 0x85771ffe82ceafdf, 
    0x79d2c1883cf91538, 0x083ddb31eb6008a1, 0x1e6cfc9472b49f3a, 0xce8854acea5e24b8, 
    0x27afb26b41020580, 0x9d25c68f8bcf1c3a, 0x805402644867c522, 0x41e8c5ffc4ea5d00, 
    0xcd81dd45d46e0c8d, 0xa8cdf8d8619a150e, 0xd8caa106697706fb, 0x6a1406de52333f04,
    0x619f36b7b1eb4599, 0xff8eecd0580401d2, 0xce75d7dd5a10eb0a, 0x0e0a9177ae74d947, 
    0xacf8b56059b2d479, 0x21575fd3c592dbdc, 0x4aa4877e2e125fff, 0xbaea8c242d7282a1,
    0xd5e260b93a603406, 0xa26b3c14f4a7523f, 0xf1bb64925dda14de, 0xee68f22335ac8309, 
    0xb9ddc15698504d79, 0xf99b26823feb8a52, 0xa794ae84547aaf93, 0x57466e1c585015b8
];

static CITY_HASH_64_WITH_SEED_RESULT :[u64; 256] = [
    0x0000000000000000, 0x2aa4bfde585a7a03, 0xd9b5c24c3ba22b12, 0xc8fea18b638d9ee6, 
    0x9be507f794951290, 0x123c7bc6667cf71, 0xb6eb566efc602c9, 0x4b0c969214ab97fb, 
    0xd2b027978bd5ce98, 0x89217de1e51efbca, 0x5ac58c1bdf7dc68b, 0xa8cf829ac4a6778b, 
    0x13408a73f249454f, 0x3e8efb81ab219a9d, 0xeb4c0a446d1c3bf9, 0xda4e8e4c414f72e6, 
    0xa02324eed0356e79, 0x63be2c7ff1a9172a, 0x38a2325c2adf7c6d, 0x9ae04c94f48e4799, 
    0xdba9ca67ab5804bb, 0xc640b2497cfa7f2b, 0xb180431181cb9dfb, 0x21c6f37ad5b4d0a8, 
    0xf7fb075ea212d1d0, 0x894f34f05a377c7b, 0xf0d37aa2c3043d8f, 0x387047c2600ba902, 
    0x8ce0057dbb889878, 0x177373ea030231ab, 0xa4a4bb4df13f65fe, 0x849556a03aefe73, 
    0x8e8a9218d8e624b6, 0x38e3e88ba5fd4ba8, 0x6ef658338ac4855b, 0xb8f826153cfdf0fa, 
    0x58f1c41d45c35d62, 0x5639deaaa575c009, 0xe68dba91e62bb398, 0xddb168fff15f913b, 
    0x5ad36941fa6a6a1b, 0x12ab6b55eab3f9de, 0xb5da94623d86cdb, 0xb7f82266b113ceeb, 
    0x68cd25f749ad67ed, 0x89616522dab0a30d, 0x72a18920bb8e3baf, 0xf197cfb800adda08, 
    0x9d9c0df1a9234b69, 0x4b30526702de7ee0, 0x90eddf02fae84cd0, 0xd52b37a31d717293, 
    0xda48c82e4765d206, 0x23938bbf9cae18bb, 0x3cacc659cd911e6e, 0xb76fe8210ae61f9d, 
    0x7d70459dcfa9372d, 0x28f7a7d084eb115c, 0x86c1971428d8f2d5, 0x1749c7ed4b611753, 
    0x46938dda1c9d53a7, 0x8c45708fe0d7de3b, 0x12ad79b7109ee11d, 0x831d65a44e7336d4, 
    0x640e2bff75cd84bd, 0xc185f470fd72c0f0, 0x118a1b0599ef8a19, 0x485a214fa7c31936, 
    0x3065ce02399c93f9, 0x9f643bd2d2e05144, 0xad6534a32873b6db, 0xbe6fa57f3b44d84, 
    0xec73909d1304f327, 0x85bfad2c931067de, 0x45ebdc349223f57f, 0xc0d7c000747ed7be, 
    0x7ce6077e0fca9dca, 0x48992b2311d8ace3, 0xccdcab0a5e2e7ab, 0x718f3dffd1ba6b88, 
    0x4159daa9e1107a4f, 0x70bf72f7c2e38767, 0x4e8a02bcb26998ba, 0x190594fcfb91ca53, 
    0xe026be06d5e9d686, 0xc761eb1a8dfa232d, 0xab303edfc00857a3, 0xd8479484e1c13feb, 
    0x9559a59e9620244e, 0x814d8e898da98765, 0x2fe18527de767d5f, 0xdf3a21cc88a42e82, 
    0x25ed46f427127fcf, 0x32bf65a01e70ef02, 0x73608d1507d589fb, 0x780c4d3bd2ce895d, 
    0x4439b02772150c02, 0x5e5ec9d3e5af9908, 0xcc8cdc1492e4f95f, 0x6aaa7c154c6e89, 
    0x83673c70bd239fad, 0xac89ebb0cf717853, 0xe812d75020b782a3, 0x8bef1e28c104bb60, 
    0xabac77059536262f, 0x6342fa31d41dfb17, 0xcc162007c695606d, 0x5b82b037833db81d, 
    0x5c2f0e4931522aa4, 0x8dcd563c4b66401d, 0xa417263432bcd85c, 0xcf6e27a746c49b28, 
    0x2e6e6b0866a1b7db, 0x7550558e75b38066, 0x90bb106f0c2ae73a, 0x80c1f89fbfe12c1e, 
    0xb63d991fa90e83bc, 0x6a23b333e2ec5886, 0x96fc614bd5227fa, 0x4b33fa6fdc9d6fab, 
    0xb36a6b9880d397b4, 0xfc3f9db6f7915b1, 0x81802bd1abb7b205, 0xfb30308798137fc4, 
    0x3fff8eb649580f4c, 0x7def1d0cafd11db5, 0xca8f44d6e315f032, 0x93428b8711146472,
    0x41c1bbed7b38669b, 0xbe3006f4995471df, 0xbd791af8571f0e29, 0xfd254a3ddfa796f9,
    0x45a42fe80dac7705, 0x3a68e2b330e44f60, 0x870f8ac93e0568ad, 0xd61e5ff0129d72ac,
    0x9cb0c71109146bb4, 0x63e3908c2ad0de08, 0x9e7b2d0c3f649ceb, 0x7437245e694115c7, 
    0xc43a8337758ee194, 0x5981de3aa243e37f, 0x8542024e50d7bdc6, 0x72f689abf22d81ef, 
    0x2bfba0d0253e3d67, 0xd94c29940faaef34, 0x206c574622135d56, 0xb5e1aa022bfc1a76, 
    0x449544b2406e02b9, 0x7371ab91c7189bc9, 0xe6ec7a2063398e8b, 0x279fcb35974bf372, 
    0xdae9f87c6f05d7e3, 0x65f0f599e3e667e6, 0x15d507228784912b, 0x4dc18cbb494f57ad, 
    0x60eae754c7c643ff, 0x30f7237343bf31b1, 0xf0603c7f9fd99fa0, 0xf3746861fcf2e3f6, 
    0xcb68c7ee2a0f52cc, 0xf69c92726cd09763, 0x2772751f5bc0463c, 0x6c561427b8d00590, 
    0x2a1b731f6211f867, 0x776cf20adede4605, 0xb34ef2502045b0a8, 0xf0fe4b250026b660, 
    0xcd1c7f25e2209e23, 0xf3c79da6d240107d, 0xc88758c2510c4a45, 0x8d7d09da1feef620, 
    0x613c1fca022304a7, 0xcc5104f3224a3f6, 0xc67c2e71b726e742, 0x509dbdca8483300b, 
    0x14bc2c56922d941e, 0x6d3a06aade21f158, 0xa23abc65dcdd252a, 0xca3f5c2d13e5c2af, 
    0x1b66ac0b5fc5c2cd, 0x52ff58fdba961069, 0xa26e2baec39fd4da, 0x670c25ca92c03bd3, 
    0x882b721f17dd9905, 0x74cc86f43a08b7d9, 0xa0fdbe823cb951c4, 0x17f6613a306f257e, 
    0x121b3612dbc18d4c, 0xa6a6f659b295daac, 0x5ac566d4580ebe10, 0xea7d2f1906f55004, 
    0xda2961d333dbbd1e, 0x512005923a3bebfb, 0x73b2f35c1b1f5d91, 0x844a67116e2133ea, 
    0xa8161e51d378746c, 0xc4b4ac416004155a, 0xc9415905b1db43c5, 0xb3724755e0ce40e9, 
    0xf16df9b560a2e311, 0x4bf7e985499cd1d2, 0x5cce9a200004c271, 0xddaaea142060097b, 
    0xfb2fea48db679745, 0x1f75e5b82486a82e, 0x3aa815258f4d2f2f, 0x9c2e10c2b19564a4, 
    0x2fd7e23bb9d3c474, 0x6cc90ed5b543a423, 0x35129b791af611df, 0x4e62d6104edf3be, 
    0xbf2b5737aa083c65, 0x3351f9722b300d54, 0xf7652b482945a969, 0xd6f74c45848e56ee, 
    0xa4fa8a6ce997396a, 0xd3ec382b49809dc7, 0xeb5ef07bd04bc07b, 0xb10d3e09e5e3c24d, 
    0xd782dd3f5edaaa49, 0x51656ec61341fe9d, 0x965cd58484571ab6, 0x67b3f20281cd3bb7, 
    0x368172fd510b7bc1, 0xc914e99268fa38ea, 0x8e3b20a5a11d1241, 0x4bb6764d27ab9485, 
    0xefa17d2745cae8e9, 0xc6f3278c116b2306, 0xbfa1379428c25726, 0xbcced829e7663f25,
    0x5876c4e689665b4c, 0x298770149625115, 0x4035c20481da817, 0xc99a2c256b66a017, 
    0x85f3848c8fe14c4e, 0xfd4c34307f62748f, 0x628d20d7a30b535b, 0xe7c99368bb0e4f41, 
    0xab65d456ba3dcdec, 0xe66cb754ee4e88c1, 0x2629073a8c5f0e2a, 0x2fc1f3dedfc166a2, 
    0x553a8227d03a0c05, 0xef135bea9d1604fd, 0xa73533ba823b188, 0xcc1b5775d37482ff, 
    0x10ea38fe8491e231, 0x5486ee656cdd2638, 0x527962c495658a4e, 0x5e06afe1ec8d9542, 
    0xdfd8be0bfb7385db, 0x95c90e2bbf6ebecb, 0xa66bd279fbe66f9d, 0x246b9d4666cbb398,
];

static CITY_HASH_64_WITH_SEEDS_RESULT :[u64; 256] = [
    0xb7ba6920c4789b50, 0x595bd86afd674b79, 0x1d0501091d0bc74b, 0xfe7c46a4d602b494, 
    0x67050a3c87168847, 0x7c45e7fec4fc16b7, 0x515d88f7f34fc950, 0x5b4cb252040167cf, 
    0x6e179a4a743ff01e, 0xfddc64f846816a35, 0xdf4ee9f1259fc671, 0x494ee43a7928094a, 
    0xfb4b040cc1a3c4a5, 0x65b99df91ca385e5, 0x139fd56ada01621f, 0x26660c99de207cfb, 
    0xa17054465576ae28, 0xa2ab783f53964d94, 0x682280a0adc93f8f, 0xad6295a29de66ec8, 
    0xe7b4ece2fd51bbc6, 0x4633cceb33dfe3f9, 0xb0bdada405b2af7e, 0xe972bfbb8471602b, 
    0x7318732a7bc8c08b, 0xb77ca3aed4efb14a, 0xa9a3349af590aae0, 0x4ec4d2883dd79b69, 
    0x216da14feafb2548, 0xcdc5dfd76c47e78a, 0x86b5628fbf13dcbf, 0xa181340b227ec3c7, 
    0x31475b7c5c0d2e73, 0x9dface7b5a3cf06d, 0xb6b8a728aa39c9fc, 0xd51b5eea5952fae3, 
    0x9267d1dcb1f85faa, 0xa75a92436cd90c22, 0x18642fcd6cb110b0, 0x7d575cd89983e8d, 
    0x695bc3dc02e4bca3, 0x2717d155c633b172, 0x37504ced7a5c372b, 0xf0c57232002e2c4e, 
    0x23c93ee3b08e76c9, 0xb874c5d0ddc22669, 0x6dbdc3621dd2280f, 0xbe8eb5af6960dc3b, 
    0x1eeaed5c571336ce, 0x993a968c29a00d1c, 0x4b1f3acbad1dacb1, 0xfc98004695694138, 
    0xf80f7752efc741b5, 0x80a18212f943f2de, 0xa12ecf9cd9f0330d, 0xa356375c9f3fa2d2, 
    0xa249175875c6e230, 0xc28277363a44c33, 0x2ae2ba9abab7d7fc, 0x1d7ad3b8c5e9f0b2, 
    0xd5d7f9a783078d55, 0xd5e3a911f97ae817, 0xf7283c1adb2dc63c, 0x67d9e4bc33efce71, 
    0xd705401161057498, 0x81e98d7546155eb4, 0xd949ba29e4cdc85b, 0x9fa8eb1bd91e21b4, 
    0xb5964d9f8f8bcb0d, 0x870ef585395a9627, 0xe5f3e5a4ad1979ae, 0xaa53d3b1a76bcd61, 
    0xec5ee6a35abc94c4, 0xb05b1596263c4e43, 0xf4b12291caf9ea41, 0xf82a129dfb5f4bfd, 
    0x7becbd8c5389d842, 0x379f08a56c145a0a, 0xd770afe0c6f1cfc, 0xf74fc55ef694eaba, 
    0xffc7dbcc8cd63c3b, 0xbc6c527f5eb44182, 0x8893dafcff7ba93e, 0x3b313942b3fe1c52,
    0xb4d51add988817c9, 0x12c3413d16481a8, 0xd8b2b6c2267a6bd2, 0xa80f95b8985f025b, 
    0x21fbc98a80d7c27b, 0xdf29a3ae2e8403c9, 0xc7b6ce7d4d620103, 0x55374445081d8ce6,
    0x95afefaa0757a0ee, 0x2b1dcb156d4d9ee, 0x401cf6b18c2f6831, 0x86c130c7c4ff1bb5, 
    0xff037fcfc2688de7, 0xf643e597451216f2, 0xa41afbe456137c80, 0xe7a93d94719fd38, 
    0x8c93ba694760965e, 0x13ffa205ec77bebb, 0xfa582bdecceca644, 0xbca580f8a1f012f0, 
    0x8f275679f345c0d4, 0xe4ef894580cca3de, 0x89c885a9775a4ade, 0xbd58801abfabede0, 
    0x51fe28e56a373051, 0x9c279340010b316c, 0x90378f90cf286d26, 0x141d365f320e6d8b, 
    0x3ba41dd123c64dbb, 0x46087d998ee5c00d, 0x86c5db488c78f288, 0x39a39bf304a43207, 
    0x927d71ff8c3aecb9, 0xc710c673695b378e, 0xfcf8820923465974, 0x117f1bd7c7d0833f, 
    0x5dfe0bf98343ed3a, 0xed32413c77963111, 0xcf45b7549af0b126, 0x5ce130419be85052, 
    0xce8d58741147917d, 0xbc0405551e70c3b3, 0x1011d76d379fed09, 0xedb70309efd3f358, 
    0x711b0c9fc0389166, 0xd891f4a892223e1c, 0x4b6de0e657228849, 0xc59c1a98220978ed, 
    0x737ea4bd16d2278, 0xd86eceee194b3030, 0xfe68f15965bce749, 0xb44d141816b67870, 
    0x328dcb8db8a43f5b, 0x30cfed23ad9b156, 0xbec21f733b6c6dee, 0xac1a862c1b70157b, 
    0x61c155faff24befa, 0xd02d1c8b4fdefd04, 0xb1276265c93ad856, 0x2585746e34f3de9e, 
    0x118029917fc604d6, 0xa2b784e1646b66ac, 0xc1cf0bbb6d5cbd5d, 0xe14c07af828dfbef, 
    0xd9cddba016060563, 0x8befb00342635, 0x5e88cd4cd900572f, 0xeae806ba82f98c02, 
    0xafb97f1d8379d19c, 0xf7e3d48d48246890, 0x85ac24bdab9e13ec, 0x31ad9130dd7b24e3, 
    0x537fa1af72b3abbd, 0x151474ea46894eaa, 0x2a07759ea921f841, 0xa70a62d386a9787e, 
    0xb468936f19441d92, 0x1dabb0db9950cdb1, 0xfc9c0576cdfbf960, 0xaa39d799fbdb104c, 
    0x4011718e600c89ae, 0x2a0110f50be5c1d9, 0x3d6dca3bd28cdf23, 0x24c630c6d70bb74d, 
    0x294e4c11d24ab428, 0xb25571d02d9c1c21, 0x39db5254552280da, 0x9ad7077134ffafde, 
    0x524bca4d951a4645, 0x307873b19fbe0a8e, 0x32fa9bda277d32dc, 0x1f7f963fcc86bcd0, 
    0x747bea24afde09a8, 0x4038a0b63a00cd92, 0x6d1ce3df5650d3e4, 0x2828fb0993c33273, 
    0xa0375682ccb4eeaf, 0x6d30e3527d7ffdc0, 0xb9ff8e2f74c540d6, 0xa97158eea3a439dd, 
    0x67fc8351001ea130, 0x9a4e0547705a68b3, 0x5a46c9fd0f92049, 0x316e83e852dd7967, 
    0x9f05752d3ea8dfca, 0x6ce6ca559446c9cf, 0xd17f155d1911065f, 0x7d0610048f31624, 
    0xc1b191c6a6144d63, 0x4e084c2a249974a8, 0xfb69fd4c0fd2cd92, 0xd49c79b4b1d31759, 
    0xbe54259c7643b026, 0xad2e0532f461445e, 0xef300ec8e26e3f2b, 0xab781031291fa074, 
    0x36756c897c06804, 0x3ea90e2ee158ab98, 0xee3f8a9627a002df, 0xddd13bf69bb746cc, 
    0x2b1bea3adfbdcb74, 0xde50be29958722ba, 0x4ed5f019d8516ac6, 0xf3e8b3a8dc0d4892, 
    0x8c7221ea39c0424a, 0x8bcc0cfcd47be6b7, 0x9ab4c25acef213fe, 0xd6db39d9b387c7c2, 
    0x1079e084add046cf, 0x68767f4e452b751c, 0xd87f32be35e111d0, 0x6c3b6e28b24276cf, 
    0x38c5ceb377e2554, 0x40641fbf1c87d5a3, 0xdd2d3f61eb3beb53, 0x1a813186ce90660a, 
    0x42d194e38a61d08f, 0x66a8f8fcd81a3c79, 0x6972765f53b75a5b, 0x6df2460567c1005f, 
    0xe6cf0a70cdd706ab, 0xb491b203af7ab8a1, 0x20bc8a22ece6345, 0x54b8aa2c4275d484, 
    0xd36ca4da6c65610, 0xbbf321107a04063d, 0x82c564447878b200, 0x5db6662b7290a638, 
    0x51ae6ba177c0bea2, 0x71b948613c5f1f32, 0x9052449537608bc4, 0xde3c3dadc2fab67c,
    0xd13f60d7e6096c2e, 0xd48c7d91b270d829, 0xfa3debfda425acdc, 0x298c156d480ad0bb,
    0x3f63bcf60fc230e6, 0xcde153100b420619, 0x1fda529194a695b, 0x93a0794f04f46b4d,
    0x1fa52fa06d161a92, 0x6b4d670d71333909, 0xd182d4f6521b89d1, 0x52c518b44002666a,
    0x84f79abab4ae5e80, 0xd7dea2b83e278c2, 0x6e701e32711064e4, 0x52ebf806cae3c6f5, 
    0x1b8997244275878e, 0xb03841f41cf01232, 0x3b4dc5a3135dadcb, 0x2a623425a27eeda4,
];

#[test]
fn hash_64_from_null_ptr() {
    assert_eq!(
        cityhash_sys::city_hash_64(unsafe { core::slice::from_raw_parts(core::ptr::NonNull::dangling().as_ptr(), 0) }),
        0x9AE16A3B2F90404Fu64
    );
}

#[test]
fn hash_64_with_seed_from_null_ptr() {
    assert_eq!(
        cityhash_sys::city_hash_64_with_seed(
            unsafe { core::slice::from_raw_parts(core::ptr::NonNull::dangling().as_ptr(), 0) },
            0x5555555555555555
        ),
        0x843a9b9c18a444a2
    );
}

#[test]
fn hash_64_with_seeds_from_null_ptr() {
    assert_eq!(
        cityhash_sys::city_hash_64_with_seeds(
            unsafe { core::slice::from_raw_parts(core::ptr::NonNull::dangling().as_ptr(), 0) },
            0x5555555555555555,
            0xAAAAAAAAAAAAAAAA
        ),
        0xc1ebecd2e7eb66d
    );
}

#[test]
fn hash_64_no_seed() {
    let mut key = [0u8; 256];

    for index in 0..=255u8 {
        key[index as usize] = index;
        assert_eq!(
            cityhash_sys::city_hash_64(&key[0..index as usize]),
            CITY_HASH_64_RESULTS[index as usize]
        );
    }
}

#[test]
fn hash_64_with_seed() {
    let mut key = [0u8; 256];

    for index in 0..=255u8 {
        key[index as usize] = index;
        assert_eq!(
            cityhash_sys::city_hash_64_with_seed(&key[0..index as usize], index as u64),
            CITY_HASH_64_WITH_SEED_RESULT[index as usize]
        );
    }
}

#[test]
fn hash_64_with_seeds() {
    let mut key = [0u8; 256];

    for index in 0..=255u8 {
        key[index as usize] = index;
        assert_eq!(
            cityhash_sys::city_hash_64_with_seeds(
                &key[0..index as usize],
                index as u64,
                0xAAAAAAAAAAAAAAAA
            ),
            CITY_HASH_64_WITH_SEEDS_RESULT[index as usize]
        );
    }
}

#[test]
fn hash_64_from_str_lipsum() {
    assert_eq!(
        cityhash_sys::city_hash_64(lipsum::LIPSUM.as_bytes()),
        0x999ac74addcd6dc7u64
    );
}

#[test]
fn hash_64_with_seed_from_str_lipsum() {
    assert_eq!(
        cityhash_sys::city_hash_64_with_seed(lipsum::LIPSUM.as_bytes(), 0x5555555555555555),
        0x6976f8f604619566
    );
}

#[test]
fn hash_64_with_seeds_from_str_lipsum() {
    assert_eq!(
        cityhash_sys::city_hash_64_with_seeds(
            lipsum::LIPSUM.as_bytes(),
            0x5555555555555555,
            0xAAAAAAAAAAAAAAAA
        ),
        0x716f9c1db5c27122
    );
}

#[test]
fn hasher64() {
    let mut hash: std::collections::HashSet<
        _,
        core::hash::BuildHasherDefault<cityhash_sys::CityHash64Hasher>,
    > = Default::default();
    hash.insert("the answer");
    hash.insert("the answer 2");
    assert_eq!(hash.get(&"the answer"), Some(&"the answer"));
    assert_eq!(hash.get(&"the answer 2"), Some(&"the answer 2"));
}
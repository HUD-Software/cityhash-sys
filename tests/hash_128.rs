mod lipsum;

static CITY_HASH_128_RESULTS: [u128; 256] = [
    0x3cb540c392e51e293df09dfc64c09a2b,
    0xec44f6870f0c779a4e790cf9d65d69f9,
    0xd837acd99d12eb08294247f63b8a20de,
    0x5f2b6684ddf1558eadd84b157895990d,
    0xcbdcae9279e8c05a433971b579e7553f,
    0xe3cb1f3f3ab9643bef3668c150012eec,
    0x8b8ee3b55899bd016a2b7fb149a5d27c,
    0xa713d924495620bd4d9b27f202a4190b,
    0x04f11aa2377e09ce5488788b02192ce6,
    0xabb8d247339074986a74d8e4769da538,
    0x57b6779de5a1b9f04e4072163114d91d,
    0xe72bf54feab39cf4d4d990f02f7ebc7b,
    0x308bd66a58415c49d10c722501c73e73,
    0x7e2ee18e3554eccc17e6aa55530f0fb8,
    0x5e9b170312d5c61bf254778a75429113,
    0xb06b4aef76dba14b527464a5d6763187,
    0x1483444197d80b512d2cabe186699639,
    0x1133848ad55b7e2865daaab406671434,
    0x6df45e62c3ac5069ebe42076b5af2445,
    0x23b1fa3c09833f9e16312a078cb331bd,
    0x01edc3858b6258474cef9a0e27d4a595,
    0x17c267244150132d4ee6b755bf5282f0,
    0x8bb566887632ba6d071544e1bd944fc5,
    0x210e466af90afcd39b7e2f1c657141b6,
    0x6c70ea848ed65230f34cad9da35d5807,
    0x1128cf22227d9a05a9cb5a285f1e17fc,
    0x83622d93402152dde8bebc38b47bb577,
    0x3fe1358a42799269e4f53d790e201a0a,
    0x1044567fda9c242978358d9a5f14d23f,
    0x2c69599ded49a1b447c31af07926569a,
    0x223ec336cfa0707f4a6c352f3a5715eb,
    0x2d05375249455140d9e3b6604307af1b,
    0x70c7a8c6dfda37f12e7084c66b496d21,
    0xdd7bc7669361e16e000e6231c59d57c9,
    0x033008c0f0f9d9261da1bafefe8df041,
    0x9182d26a46e41b39abd44db37becf07a,
    0x7a8f9fbae1ad6550d19ad0d27416afb7,
    0x02f74e44e4d681095470c36633b0975b,
    0x217b06ef88682ec9d24a218f64fa9ac4,
    0x54c51a6e89276ea21fedb2ea10195909,
    0xf380d0c15e6253a645ffb346110be804,
    0x2ebbd772b4d4123674c9ef4abf549fe3,
    0x8ed8d9a78f8201cf1975e1aee599f087,
    0x35bc4a75ed36486e30b7e29491797cb4,
    0x25329150a7a36ca9daefaeac7f61683b,
    0xd2bba363926dfcafbdb7ddd53955761e,
    0xfd22f325e99dcba755b968811af35193,
    0xf3061f2e4eb543e550de8f9958d81bc5,
    0xdf2867fb34de4d71b2be68b8ec4bef0f,
    0x9b59594837434449136a64fb59ffa080,
    0xbbd092f6b3791696d1a38e8084783efc,
    0x6c2304f21dd07dc388dc5738d2c16a7f,
    0x0a026a9591533ce4ddf8aa7473249d0c,
    0x9e5b9bb1d07ef9fe81be87ad6bb281c5,
    0xe65ea36577519b6c228e755276d107c3,
    0xeb28f41f872fe9676114d88627ddb3c8,
    0x78ae209bb1566be5486239389d2fefd1,
    0xb532e5b3cd99bfa51ea6bf8f8ed4a819,
    0xcc84bcb7143d79cbc53d4c9fffcc9a98,
    0x0e41318dfb50d05a2084ee6b2bf131a9,
    0x02f23e691a75c3245bcf61cf93a787ae,
    0x400c38e4d33c9d8ca32e00517f12a77b,
    0x83cff08fd53c7575221efa5f95439256,
    0x5adbaa92d64a5a9937313fa5f8101c5b,
    0xcd557eef420821aff3afb1aec5c36739,
    0x8b20fcfb9d50e08096c2ef74483ad331,
    0xc386a812831cf79777f0958428d88036,
    0x137404537fb5dd11db1e64ac599e6fac,
    0xf0d9c46d62b0df56d37ab9d29e8ea862,
    0x38aaeb03fe99ce1aa7e79863519666e8,
    0xbc03f9ffc30bfd68befe0316c6cb6fca,
    0x277c0e245a73532994a3cbe43106b7c6,
    0x31e9b33a2f9088f53b37374d3bd20c89,
    0x314d2f941f3c41b8c68cb59edb5e77e8,
    0x9492d3f3376a9f705f532a0e558e5068,
    0xf733b6db651d6d37adfbfe0f588d25ca,
    0x6d51fb1ff9537b976eced68329428ccd,
    0xf85174a1b63cd3105d08f8f6b76f9f5b,
    0xefff6c11281ce7febd3793028d099c69,
    0x3de277c37731f0b4c39b0691842f77b7,
    0xcb82f6ffe1a610ef87f8fde0451340f5,
    0xc52432dc68a463a300391223a02a9ba0,
    0xa3e70d9ed36b6e0fe7fbac51f1e08e35,
    0x98ffd8adf8272bc2d9d5b26122b0dd89,
    0xe42a97ddc01153e851f98d5ce1386753,
    0x3a72fbfc121f7b746efc8afbb4610b41,
    0x65211a10ae9c81aa6a09d00f7331e094,
    0x8f30bcc49a253f87bc5fb30655c55b5c,
    0x13a21eb3b3d3112311938f1080d10bac,
    0xcd94afc89effb7b36caff5b4654440ee,
    0xb026b80a15cfd7b74f9532627874fe3c,
    0xa2fe5150bd4cdec3173db9a4a9f8e48e,
    0x69c908ef4c540a25e228ccfc278270e6,
    0x12a3075a56dd185c7488058638dcf966,
    0x16434eaea89fc43477a1fb03d4119e87,
    0x6c15d77d4f1700b08f59e0f29c85d8f8,
    0xad1d73b74f1ab5d172326d8a3be43854,
    0xf7350dd3a92cae23c065e831021889f8,
    0x416afdb088d477727884814b7421d63e,
    0x86b125b56440f9f30591a0ef036f6c19,
    0x4af6eac6b81177e082940c1b36354e6f,
    0xd81faca1bd00a684260bff0f40d30bb9,
    0xc8a0342b87ce7551cad927e9ddc5e070,
    0x9ea118bfa7bb7fd846ee3b1a461abc1e,
    0xb1c672b7c6fcb1d2b0fc39e0cdd48fe0,
    0x375a08d99009b569401795da7ba56f4e,
    0x7ba68613731895f66f9451abdcb160c8,
    0x2e7e847b29bca05928fac4a6b6153a09,
    0x27db54fa0dd67c5caa610fb268f17bea,
    0xa35701f27a76d37285d6bc4379990fdc,
    0x5bc84927bd995d97eae25ddfbdcb6018,
    0xca887315e344a9b447c8ffce0ec34858,
    0x17ddd7b713247dc605b8dfc0de554f1a,
    0x73d8848f272a54afa34867cd22e4f0d3,
    0x01701df3083a1cfc5bcebc95b18c7d2c,
    0x9682015c55ba4a515261574ecc1523bd,
    0xd888f723560790d303dc85d917eaabe8,
    0x44ffd22ce438b16c498780e8a67ed7c3,
    0x51e471f52eea4ac7b98497384eb499e6,
    0x34f91747f3972dd11bef8d1547542dca,
    0xb6fa77fc0434f64a6065674c13ba6791,
    0xf6561ff643330179945583035bf4ef1e,
    0xa674ffb935079e41b6c02a394d99468f,
    0x0e12a91dcaa201e464e79cb45f509236,
    0x3d6a83cd43ab6cc8841c6f8d6bf23050,
    0x543d1656a6452eff39c89d8b501f279c,
    0x1546c3ea33d6fb92c0db4319bc47149d,
    0x61e6d23078b7ac2d428fb4f9e24908d1,
    0xd7e962fbe3832fa2987f34a745925167,
    0x18edd1dfa71d848e1b234b36ef05f145,
    0x48bb17fdfb82ca4a8be3bc78f3daf3c1,
    0x1a83b5ffa93ee18281de32d9b94b6bc3,
    0x3dc33a9efd8f5d3c53f0e51618051267,
    0xc6e6a97284962b53c2e88c11a99edaf2,
    0xe1b0d20e1b080442d9b965d94e661b08,
    0xdc535bca0fd2a372e496b12d6e1fa8f3,
    0x0b9867d7869ab05aa907d6f98b06ab2b,
    0x494f091b260d3a402ae3f7b53526490c,
    0x04b6aae1b8a30a40ed4d38ac06f9d894,
    0xad44a921988e94321f4865480e11af7a,
    0x2d7f6d737f8118261cf53915b5ab2fe3,
    0xad1fa85190063d4a7b4390c7b3fdcf39,
    0xd876d5c788563428432489c7b6db6032,
    0xe669f284a5e352cc559af9fa52171a01,
    0xcda5dffa3919033611a3eb8dc4c0134e,
    0x3a61e7e721e6fc7fc627706f2015dba2,
    0x023701c46354f12b8f50852d3ad381d6,
    0x833c4eb0d543915b8abd770636eab705,
    0x472ded0161ea26360db1db20b976bf36,
    0x043863bc829b52339edcc0c887132417,
    0xa63a34d7a0cc7bffe077458cb1cab04e,
    0x20767fc05d542e47986786bae3ed6661,
    0x2e739953503b5d5c7cb86a7518d5f025,
    0x036656eb2ba7d8a5ddf91f4e297cc182,
    0xf1c81c237752f08b2cca8ddb5d3ded53,
    0xba5e21daec9cbf50bf655e533cb2e9c3,
    0x90ee0adbb11787dd3949229132af7870,
    0x03389f5e969af946e0b0af9a2d7b253b,
    0x9a499083475628ed98be4bfc04b04400,
    0x267b8f8d3ec25f04a2890e01a11bc8b8,
    0x0d354d435453c11a081ab9ab65678612,
    0xaf344335464e264afe57421631e69992,
    0x9b43a2a0aefd69ae4a1bce562de0e63e,
    0x3772fd209ac9c8685e98761e5b38e37b,
    0x71d6d73fc7195e083fc871d8f76c571a,
    0x41b65f346924eacc6cf6517ba0f9ec46,
    0x6a72078fd35d452b5f7c740249c3c9b8,
    0x1d21f98815a8ee555cd78ff302f30c9b,
    0x2453580b6a70d498be0cbf1e9ee0edc4,
    0x6d683df8766bbbeb0dbe93b4b0e56652,
    0xc57ebb88eda0b5fb6082214725faea08,
    0x8ee6bb1decda2b7e0996e2d243117f56,
    0xa677de71dc7c2392195dc87c0b86f360,
    0xfb02a5b52f6cc751f1dd02717780a97b,
    0xdaaf32917e4c5cc10218115acf07a899,
    0x5a2d874d4b8c816a2eeffd0c40a55b3f,
    0x6bdd4a785f39150ea745baf960fda3c4,
    0xf60c1ffdc5f20b0064ae3a0f1e7865b6,
    0xf980398beae8d4e7214b281922188031,
    0xf8231ec543a48d825d224b15870c636b,
    0xc9ccc4b73ae4590bfffaf9ad2ef42aeb,
    0x886ec2ecf64f8fcbaf46ba87e9356fce,
    0xd39f8a20e1f6a4786dd5e1e55d2440f5,
    0x9b0a774e178ac97fed172174b26765f2,
    0x30fc384997e060717dc2dcae556b0c72,
    0xba3ee0c5c083cd29286e13c369b8b15c,
    0xe419ff0188e1be96d0d263abd64b9649,
    0x9b6aec794a242b04b6ecbef95eeff7f4,
    0x36fb0d4406dc861f51abf9acd16ac073,
    0x8f2009640be1b2af69a9e6ddbfab8834,
    0x268bddb75fd85246dcd32f5474671894,
    0x59c686c71c391fc69aaf7646b523b698,
    0x3254945b14398687125e5c0660c4b6f6,
    0x000103f31c96b7eddce731575c598161,
    0x9a191e757bc07251c9f197db53d8679c,
    0xd2a159f9e1841256bfca268392c530b3,
    0xc243423b3fbac9a6e61e6fe1e3e685e8,
    0xd9948e4392d46a5b6c82bde2e565e348,
    0x32b84fb061300015569f20dfa0966ed8,
    0xf14ad3276260cef96226cdb412ed0090,
    0x5ecd9df9c773c308041b130aac352f97,
    0xa48618abe99425bc8829d357cd6e24ae,
    0x02d467d3dda252a5e724abc5b99f040f,
    0x4b1b9600a66b46d89d75af29240efd64,
    0xed3f0cdf2ea155da2dcb45a491dfd8a1,
    0x6acc6dbfa9a1197ede97667b83c4bb48,
    0xd3c26a134fa5f52a51a5956a2210441b,
    0x05195fb40fdbe96a03c5a7508182d49e,
    0xa8f1390f6f449cc5f84aef44148dd964,
    0x2f7a3b34ff74fd7f63e5a3ac99212683,
    0x38caa787b76e4ca915f5b69fb47c8a2b,
    0xfcc3cd5a9dc3a539aa9a1fd466083fda,
    0x0b1732d40d52804c850ae4e378d0b8c1,
    0x0d22610fada209b3678db9e4d89ec9dc,
    0x20b7b00d36a4cb1fe8625705e4448289,
    0x0df32bf2689d503b0e46e4c1fe5f3726,
    0x28887b7e70817d5bc02c1df309b932fb,
    0xd27e58cbcd73b77b07776568a4918db4,
    0x213aec555da9deb1e2a7022a3bf7a799,
    0x6d10709cc63fd9ea77d00936a950fe0a,
    0x65f2dd346c181f0efa58cf720f121442,
    0x34b13f9c4900c42f05672ef92109d8cf,
    0x0f1828a8841c871d4b1b2b126df44f9c,
    0x2c4d410949fe2a46a7e0ad52c6db4f04,
    0xed04f2baca3529042b0120d7b1cb88b2,
    0x677c1a018cad3c641530c6885fd0cbb6,
    0x62931ec8028324d6a067a32d9d08a951,
    0xc79ec5ea2bdfeedfa381e5585d9f83e9,
    0xb4ca9a50c92490734257a1e33945a861,
    0xcfbf8d987ad0d620467450d7b5506038,
    0x9b4f078ba4ff8ee4b2bac482263e8fb3,
    0xeb50c45ae5d76684e364a3627b792f23,
    0x71387edc0309456d4bac6d575f7419d1,
    0x434cf5c11223456853bca18c96287abf,
    0x0e3bd90afb1d53cb16ef2cddf136a87e,
    0xcfb9a0280f72160b244af12421c273fb,
    0x4f4121bde9fd33fd9f91b19012f68df8,
    0x0b7f71cfb1dcddd01de1f9ba11c29867,
    0x630fe826e1c353f6faebd38d17b1fa5b,
    0x33f442190d199d6e1f2b21b455ccdd57,
    0x2d60b9775786e6f4cdc66b9f6cc57383,
    0xc673eed49cd418473c0fbe3fc956680e,
    0x1592956570e2183d3767ae1c3178b3d6,
    0x6431159621362a1ddc9304e571b7b7bc,
    0x8fc234388cd1d8bc504347df42f16b15,
    0x7e03ba81c5cac92a5cd6b11b2ef2616c,
    0xe0764821a0618d8a3e0e1ac3ba89c2d5,
    0x2bd5f162e1e2e032898579b0c4c700a8,
    0x336b7246f23bd70e82ec01503887a433,
    0xd5526c0f328f2122527b92a388ee6238,
    0x638ea5f52d62019257ca25013f2fd62b,
    0x7da2dedfbbf2380d4b5e1823d16ad3c7,
    0x5ce3c95322ff4f9f6fcc6fd65975ec63,
    0x166d806e604410f92e887d136ca4f76c,
    0x7a79b2fbe94b1f770a58c9507639a02d,
    0x527af4357baa9a09606aa1233aabbd56,
];

static CITY_HASH_128_WITH_SEED_RESULT: [u128; 256] = [
    0xb2369acfccf83dbffcf7cc0ecf416467,
    0xde377fa8978b9f559c1f2c0d0b6ed4c0,
    0x1a19684e2ee4a08179af732971716b03,
    0x4adffc09d7ec9a1f1ce4223110420fe3,
    0xd6b80f23bcd3d7b9a68c836f669ccabb,
    0xfc02375e824d9ca74900e2e12bb12871,
    0x34bcfb65614550aff419323ae9a7d1c4,
    0x36116751fa1e17180b46ad5668a3ffb1,
    0x143f3a53089f4ae1feb3ff57d1d3b39d,
    0x5e303746c32cb908393e5b84fd67253c,
    0xe46c67eba6720faad4c86bbd6d758f56,
    0x17dc1af74b75bb403d24cccb634e85c9,
    0xed2fd5f7f220994515ca307e1cf2b382,
    0x2c3942bc7428c5d85268a9087e77b7e5,
    0x88cb5a748cb79593884b36f2da13cefd,
    0x2cea238f72402e03acb707f99b791f58,
    0x99cb9771f8cd93a4c6625f0fe18fc827,
    0x6c96b96bd5e025875736b61760de2b80,
    0x39a922514ce90d866a21673d0ce37809,
    0x578a9adddc00385f70074b785093839c,
    0xd553e0e8869b1d3e03389751936187ed,
    0xb79d7072313def548ee906a690e5b809,
    0x54e7f8fb22a26bf7e840050cd38dca9d,
    0x6c4c4a5c98bb6361b99ad87fa783fa7c,
    0xc8ebd7015f74e4a1729a0a192ee32c15,
    0x2b31f5fb1e7033683d45508609a29d1f,
    0x56c809c561896cecf2358666bc87c4ef,
    0x38188d8550b1868861a3e72be85cba14,
    0xe34c719a181d6dd83ccfb4a1853adfe4,
    0xd374e8b7b12458f40685f4a5e3215fc7,
    0x3662bbd77c47112589c49f5d90af049f,
    0x2862f8b7109a3a9e2e0181230de1f80a,
    0xd284ec0ee991bfa3846be0c67752095a,
    0xbf9cbcccbf3db2f2434142f26b14b98c,
    0x503d7ff415bd115bc4ce2303dbba3311,
    0x47d43c10e4160f9b8b9785388b3f8a37,
    0x686e2460ef2c7fddf639f69fb3580cdd,
    0x5f39348e4ce7248a187feb008351c297,
    0x31db2bc27925484bef054a5ef2815fa3,
    0x2fa0d399f1fa740beb2c5a405a7899c8,
    0x8157ee727f15431d31cc03bb5b7b031f,
    0x59ff1492d558d0c8de57772a7fe9bac4,
    0x119e4085f7a88cdee1c28df282ca8d3d,
    0xb7548a380c071b17fe6e34c7ec36ae88,
    0x0e8d983e786f5e37660e28ea2046704d,
    0xc0c9cfe6366a8d4daeeedbcfa3161272,
    0x70bef5545ca892b50c32951ea7d8becd,
    0x82340e51b9fc9ef842ecc0c22ad79ffd,
    0x0a3878fd534a2d5bd6f80b001d2c37fb,
    0x95d89e9a1bd7f9281963e4d1f4e7df42,
    0xaa25b24e3b8020dc83e364c80c2f242b,
    0x873f4e6f03fc449d854cbbaeb9feb3f4,
    0x1615416c613aace72215983b12fd65e6,
    0x30f5274100c47963a22fc4537decd621,
    0x4bbf69eee940441b06671c06a884f99e,
    0x037d5861d0db86ee6b38e1589fc465c5,
    0xb5959e74183a37b7cbfb71400162c7f3,
    0xd7350412d929ed21f4f9f7e3be86eb87,
    0x5aaf6554120046460a07939a0701d645,
    0xef6643c3c464bf07c4aa656fe81e7fec,
    0x6a63ac635ff1c1bdd64222d39c92fcb3,
    0x4556eb8fe58aebb7709b0ffe84dd95bd,
    0x981f2fa496dca03efc33b9cd0041164f,
    0xf958a91d49c618c8b188652c4b93f725,
    0xb23ec43c6e78f65e492f66cd5aa8064a,
    0xbf6a8b5782f340aaa695f3b798b8b9b2,
    0x57b75553b32c9298707eace3d9d08b20,
    0xdd76902e0aeb3cc8edae28cda333f2e9,
    0xd9fb395f5202bf8285e888a33818d4d7,
    0xd88449ac38e8974dfa4a4f5dce11139c,
    0x4027d12657c1a39623cdfd8a23b7e2ed,
    0x613bacae6573ea51f4dff5de04fe2fd8,
    0xfcb5780734b8d9fa18fa5fd18dfc307c,
    0x22470cfad48b71d9c842e5a0aa439d21,
    0xa57c559d84c5bfa23ecdaaa668d89a61,
    0xca2deb42d3f9f311db1e61fac59bde82,
    0x7a71d875cd433fdb87c696697db1feb6,
    0xb99f4ad5938f092bee3d67a52264aac9,
    0x438184925d67792a76a3e830a24e7004,
    0xe8105cd0e441666a35a86985781320e1,
    0x51d6a99443298002af3866e294ad46a0,
    0x3c37450adffbda9ef2726dc2356069e5,
    0xe3c68735284aaebcc737e0df4946c2f7,
    0xbbc3a1c56d66aab5873c9759577d04e0,
    0x2af30bbdc62753dc0c35309fd04ef4e2,
    0x318b615b4c13b3ee0a0f4bfcb8d12340,
    0xf2bd36ab7d11cc819ee75f2af4c018f1,
    0xc783f41ef19eb098236d10b6b98bc457,
    0x61c2eee560b7e9fe00b88617d5e6b512,
    0x00bdea0615e930a8c824736e99e7754d,
    0xce04dc5a968a88114c0b0a30093578ac,
    0x1f7dd933179578b14f8338667ef4171f,
    0xa5544644f848e0a40d6b8fe96c2f9874,
    0x67780d1cbdf6312db44adbba831648d3,
    0x70dea03edfec9fe2403beec160565372,
    0xd70294e832484d1eed6ad46e6a34a55f,
    0xa4cb7da4439d6bf2b02bd424b6701aea,
    0x325cba5227f19e3bda275aae9c570e79,
    0xe07eb26731c16676549091466ad9e3ec,
    0x5b909d2ee22731e7ce20c5053df60c04,
    0x4b493e21a6b49d29b26018535c7ac574,
    0x4556280f97ebb6810881ea07b865786a,
    0x58eff566db28ba8ec35bb744d950d22f,
    0x6e8da0c460bf2e74989a2ebd45a20fe5,
    0x14fc28ef7a23e7a104503c1425780ad8,
    0xfab8a794380d16130078e46380b08e6d,
    0xc2a882ce285bab8ad244f10e161ff583,
    0x835f483881c7b33d360f88be0ab0bf6c,
    0x668ea29787bec9e95bb898e27c3fb650,
    0xf72236f1e107e7db70547dee726d017d,
    0x602e0f3106f20c2e76f2a08d3dd8ef75,
    0xf7eaeda1134507bb59a58ece3fda2812,
    0x6654560d2030c4078c0847685540882d,
    0x3740eafdbdeb92866d67a870081434cc,
    0x4fd8e93cb5611084e6583dab943b0059,
    0xf99ac136e34d31730e57871d17fa793a,
    0x25f0526dd967a0929426139a1cb7c392,
    0x9d3ab3010ef16808cbd28153202628ff,
    0x10c2318016289d053f47eece4c29d90e,
    0x1ee4553a338eebe9a667e25e15b74e2c,
    0x54c300b4165bdd88af670e7a5a73c5af,
    0x6af67b0cdf77340d16cb0ce3f62f3c65,
    0x251ea0c0665ae050502ebf76accfa5ed,
    0x0df7da3b0a89aad50f14e5d81fdac2f2,
    0xbdf07834e9229fece71a671c8dbbbc07,
    0x8ffe390fb9251806453dea931c2cd6c8,
    0x6992ca36cfb621404e99760fb4c6dd0d,
    0x0f5735dc64562dbe59f4d561693a8479,
    0xc09e13785001e77995ab82cec7c42d1c,
    0x39aef724aa3ca4202b4222dd8906aa18,
    0x4fd43307e304df2cecd47f9e024e2a33,
    0x9b94dababd7e80c0dc5df1b9ff6834b7,
    0x0fdafd7658a67fe08445b0c244418f20,
    0xdb14997e835bc5c6022131688bdf9712,
    0x9c33193b83bd0a37c12ec0b791e61e91,
    0x22603591cec52dd4c2ac6dfbdf0ffb74,
    0x083822c0e0aa1ccef24184071b11817e,
    0x4e160daf2062c4fc2f0700f8dacb107b,
    0xe33901c2c63bdab972d43ba15b3d4781,
    0x2c32eed363b18b3e89bba99675f4e457,
    0x3f075682a8af72fc264c97f7aaa320a4,
    0xb6db9d5733acb44e81a7cac3f591e85f,
    0x2c3dcf29176b657965b2bb1961f9507f,
    0x9a9c0579e80b27db83ed39b5d823e1fe,
    0x05ddb67324e922b5d58531780c18f45c,
    0x4b16ba5ac9fc73a4f26080c57a82e8bb,
    0xe9f11a0170d32d3708b9ce6a206d969a,
    0xead8b1b993947ce1914409fce1e66dc5,
    0x4af9a8b9223e7f18b8301f860511c93b,
    0x923923fd356837ab67684c76bc385760,
    0x259d06a9454dbe3e357bbba454a805b1,
    0x6bf59015f75577af876eb989aefc47ce,
    0xa1ce84f883d089fce60a4dea65ad3d82,
    0x9586ffeae947b332287a0f45e2695e0c,
    0x0ded6cbcf18da12ae8e0bf630310305b,
    0x7059bacfc243d45f4bf517643c3b8031,
    0xa6427118c768f22d0d209d336cf4413c,
    0x5f8e5fbe15ed34abd6bf3e78c2427a47,
    0x900b7c14ec4d2885305f218e8c52f725,
    0x3a2b365929bde81bbb9f8d0239895cff,
    0xf0807cea02ca86fa6cf84b0143b4c559,
    0x40ae54750deaadb0c48a818057c862ec,
    0xefe407e848a6fb295a5bdd6e9cf7d896,
    0x5e4255d150b6d83aabe61db9603a2a45,
    0x527b954af275f0ea299faa4d15afb8f6,
    0x819d485c408d2b2c8ff5964b8758f92a,
    0x19db6875ca4a787ed8c2f46857391ec4,
    0x554d5349dc7900435e9a0276d24a4fa3,
    0x7445761d8b48a5888d87757c30894615,
    0x6bef0482a4a3b1f633dc193bd00d4e16,
    0x21e79df5f5becb26d07d108697fc9cfa,
    0xbbe191b8111aaf72345f695bc6552955,
    0xef78d22f7c12ac89799e59ca3f3172d7,
    0xee1021ca14ddadd55facae77265e6ad4,
    0x77ad3f0cc6d71200cc5ed72ad8da9466,
    0x72b75e745a686a44baa90a239c836314,
    0x4dac119bc646044fdbe927be98f0fb3f,
    0x2f848e8dfcd82a08eb0ac626f5f1a2c5,
    0x3d31b4c68a109f6dca24031d03438b10,
    0x6c270fd1728ffcb4d587ee172c442ee4,
    0xe6e5f5f5609c4da5e63ec39daa7880d0,
    0xdc440146e0f74a9f8d3ded9872f7812b,
    0x54442f5594e547e0d26d41e1d5648cf6,
    0x505a9e9c920f12bed5331de917b039ac,
    0xea2ad9c8f2d0f9bcff5ffeb8b124798d,
    0xf345afa1697893dcdf968386a8f521dc,
    0x698879d8b03af989b76a0bdad8c8f5c3,
    0xd75cb65cf40ecb0e2d2bdd7ff1167c7f,
    0x7ae66afd6d5ced84adf45a640f528eb7,
    0xc93eb38c7e69ebeb85f4b2720c2e7577,
    0x6e992e7483e4f7f9a5376f4b060fd29f,
    0xa42ebf8813b60b73bb736f5e6392afab,
    0x290f0b024f0ea94dd17d67c593fcb017,
    0xee6bb6de872d5050fe5c335a7657630c,
    0xc74a4d1f5718d248d96698b0e3fa3dbd,
    0xe04a169d1835e7813f1fb09d1484f368,
    0xd9751e3da0fbdbd7d48f59cf29e1e6c3,
    0xffee9084126da669a880b0d736c15e90,
    0x7b195ef588a96f7ff2aad89232e049c9,
    0x21166a326998d7d976b4469ed385dc1d,
    0x42f7d467807d4cc024b6fb3f7f4ec89c,
    0x252ce30af27970be2939b6173e171ddf,
    0x728611c5e481800465f825c97635a0bf,
    0x2f7c372083ddfa9ac7e78853d79515fc,
    0x35eb145db54cf42c9ecc1d74ec5f2f9c,
    0xc70fc0e2e4f87d18a70dfb805fca8bd8,
    0x5f8669f7c5ee3889d788da9e402a58cb,
    0xdbd5e816b48366c4cf5cc5157b428bab,
    0xbdb69edb6407c2c3ff49b59711fff418,
    0xf6ed38223da5f87daf7a3b6de652ef6d,
    0x73e9574d221e55be11e398fe61c9386b,
    0xf26f8c8d910a441d3fa54be567bb6faa,
    0xfb7128b79b1d0e2cdda7fcfbf7f9b466,
    0xef36aa992116d35b78b3dee6261f5c82,
    0x70674803c81eb00cd0f0f539e2418d37,
    0x32f3be95e14ed7341bbad1d23d1bc416,
    0x631cdac0051d7d12392d7b9dc7594d49,
    0x51bfe31b392f8e17d7d178981fc5a7cc,
    0xd9d369a34434d948576f2d4fe60d7e6d,
    0x32e8d45ac901dbe7fb67848cd12832d6,
    0x743f96fd067e20d74ce427aa4579756c,
    0x75cf04f2e3f0bc164be50cb749e4c7a5,
    0x69e22b31f61932823a9be676150e3007,
    0x23f16eaa8c3cd24fb4cffb8b1a5f88d1,
    0xf6793ad7987bbbe6082c285f7347aa4f,
    0x030dc730fe50d282d0d419358eb956be,
    0xfb9919ec664fd67fca1935e5d2df0634,
    0xa96ff025c2661602dddcb17f3248d35d,
    0x8fd9855960e864d1bc7aff393d3830ce,
    0x60c42abb1f3590817e4029d16d8f14ab,
    0xd009c5efa2a6698bdda0f9c26031fb74,
    0x151d66d5205783cd4e088c1d379597e1,
    0xc4c4353a9fcbd3a776d2f4efd09d5085,
    0x54c44c09a6781a692f7e16f6db9fdc16,
    0x60e0dc6f767506b3b58806bda1a09ba6,
    0x18626ad02634f3d0f49db0caae102e12,
    0xb5af1141410cb5ac83b286038eefcccc,
    0xf27368cf4ac36a982f1e69dcf1332b52,
    0xb43e8b1e040d705d48be33c502439ca0,
    0x6e3911ce2b858546370509bb9694e872,
    0xbe3d3598baff52a7757e5cc92418f0c9,
    0x8044b122fa055075877c4c832aab4969,
    0x74486e0e895b3a12ec0621d7c759cd4e,
    0x1082aac7539cc8177f8b2d8e9f52469a,
    0xed756c8d0d777ea543e4292046b74f6c,
    0xf9f7ec7bbf9c548d9c6646be9756facf,
    0x68f5ac49d42010d81dfd144378a68dc3,
    0x0f19b865e098b21f6b65aed90df860ce,
    0x02cdc2e930f947c12b3fc32582260033,
    0xec510e4985874743563bbb82e1f281f3,
    0xa37944aabe26813c70889e9f60b996a8,
    0xeae2a311d606902796caf9249e3b269d,
    0x7eaedacb259dddc37750372203839364,
    0x193877b710b3d4a8661c4ae1fe07ece9,
    0x007a36ff1259721f75086068b7778b6e,
    0x65a8a31f98261aabea1d8cddffffaa91,
];

#[test]
fn hash_128_from_null_ptr() {
    assert_eq!(
        cityhash_sys::city_hash_128(unsafe {
            core::slice::from_raw_parts(core::ptr::null::<u8>(), 0)
        }),
        0x3CB540C392E51E293DF09DFC64C09A2B
    );
}

#[test]
fn hash_128_no_seed() {
    let mut key = [0u8; 256];

    for index in 0..=255u8 {
        key[index as usize] = index;
        assert_eq!(
            cityhash_sys::city_hash_128(&key[0..index as usize]),
            CITY_HASH_128_RESULTS[index as usize]
        );
    }
}

#[test]
fn hash_128_with_seed() {
    let mut key = [0u8; 256];

    for index in 0..=255u8 {
        key[index as usize] = index;
        assert_eq!(
            cityhash_sys::city_hash_128_with_seed(&key[0..index as usize], index as u128),
            CITY_HASH_128_WITH_SEED_RESULT[index as usize]
        );
    }
}

#[test]
fn hash_128_from_str_lipsum() {
    assert_eq!(
        cityhash_sys::city_hash_128(lipsum::LIPSUM.as_bytes()),
        0x9C5514CDF7881DDB8326FD07983BD576
    );
}

#[test]
fn hash_128_to_64() {
    assert_eq!(
        cityhash_sys::city_hash_128_to_64(0xAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBB),
        0x88FC029FFEBB98B4
    )
}

#[test]
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
fn hash_crc_128_from_null_ptr() {
    assert_eq!(
        cityhash_sys::city_hash_crc_128(unsafe {
            core::slice::from_raw_parts(core::ptr::null::<u8>(), 0)
        }),
        0x3CB540C392E51E293DF09DFC64C09A2B
    );
}

#[test]
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
fn hash_crc_128_no_seed() {
    let mut key = [0u8; 256];
    for index in 0..=255u8 {
        key[index as usize] = index;
        assert_eq!(
            cityhash_sys::city_hash_crc_128(&key[0..index as usize]),
            CITY_HASH_128_RESULTS[index as usize]
        );
    }
}

#[test]
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
fn hash_crc_128_with_seed() {
    let mut key = [0u8; 256];

    for index in 0..=255u8 {
        key[index as usize] = index;
        assert_eq!(
            cityhash_sys::city_hash_crc_128_with_seed(&key[0..index as usize], index as u128),
            CITY_HASH_128_WITH_SEED_RESULT[index as usize]
        );
    }
}

#[test]
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
fn hash_crc_128_from_str_lipsum() {
    assert_eq!(
        cityhash_sys::city_hash_crc_128(lipsum::LIPSUM.as_bytes()),
        0x9C5514CDF7881DDB8326FD07983BD576
    );
}

mod impl_for_u8 {
    use crate::CITY_HASH_128_RESULTS;
    use crate::CITY_HASH_128_WITH_SEED_RESULT;
    use cityhash_sys::CityHash;
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    use cityhash_sys::CityHashCrc;

    #[test]
    fn hash_128_from_null_ptr() {
        assert_eq!(
            unsafe { core::slice::from_raw_parts(core::ptr::null::<u8>(), 0) }.city_hash_128(),
            0x3CB540C392E51E293DF09DFC64C09A2B
        );
    }

    #[test]
    fn hash_128_no_seed() {
        let mut key = [0u8; 256];
        for index in 0..=255u8 {
            key[index as usize] = index;
            assert_eq!(
                key[0..index as usize].city_hash_128(),
                CITY_HASH_128_RESULTS[index as usize]
            );
        }
    }

    #[test]
    fn hash_128_with_seed() {
        let mut key = [0u8; 256];
        for index in 0..=255u8 {
            key[index as usize] = index;
            assert_eq!(
                key[0..index as usize].city_hash_128_with_seed(index as u128),
                CITY_HASH_128_WITH_SEED_RESULT[index as usize]
            );
        }
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn hash_crc_128_from_null_ptr() {
        assert_eq!(
            unsafe { core::slice::from_raw_parts(core::ptr::null::<u8>(), 0) }.city_hash_crc_128(),
            0x3CB540C392E51E293DF09DFC64C09A2B
        );
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn hash_crc_128_no_seed() {
        let mut key = [0u8; 256];

        for index in 0..=255u8 {
            key[index as usize] = index;
            assert_eq!(
                key[0..index as usize].city_hash_crc_128(),
                CITY_HASH_128_RESULTS[index as usize]
            );
        }
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn hash_crc_128_with_seed() {
        let mut key = [0u8; 256];

        for index in 0..=255u8 {
            key[index as usize] = index;
            assert_eq!(
                key[0..index as usize].city_hash_crc_128_with_seed(index as u128),
                CITY_HASH_128_WITH_SEED_RESULT[index as usize]
            );
        }
    }
}

mod impl_for_str {
    use crate::lipsum;
    use cityhash_sys::CityHash;

    #[test]
    fn hash_128_from_str_lipsum() {
        assert_eq!(
            lipsum::LIPSUM.city_hash_128(),
            0x9C5514CDF7881DDB8326FD07983BD576
        );
    }

    #[test]
    fn hash_128_with_seed() {
        assert_eq!(
            lipsum::LIPSUM.city_hash_128_with_seed(0x5555555555555555),
            0xB0EA5866D4FB15B8AD82F66132767D3D
        );
    }

    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    use cityhash_sys::CityHashCrc;

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn hash_crc_128_from_str_lipsum() {
        assert_eq!(
            lipsum::LIPSUM.city_hash_crc_128(),
            0x9C5514CDF7881DDB8326FD07983BD576
        );
    }

    #[test]
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn hash_crc_128_with_seed() {
        assert_eq!(
            lipsum::LIPSUM.city_hash_crc_128_with_seed(0x5555555555555555),
            0xB0EA5866D4FB15B8AD82F66132767D3D
        );
    }
}

/// MiMC Sponge implementation
///
/// Source: https://github.com/hideyour-cash/hideyour-cash/tree/main/packages/contract-libraries/near_mimc
use ff_wasm_unknown_unknown::PrimeField;

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct Fp([u64; 4]);

impl Fp {
	pub const fn dangerous_new(limbs: [u64; 4]) -> Self {
		Self(limbs)
	}
}

impl From<[u8; 32]> for Fp {
	fn from(val: [u8; 32]) -> Self {
		Fp::from_repr(FpRepr(val)).unwrap()
	}
}

pub(super) fn mimc_feistel(left: Fp, right: Fp) -> (Fp, Fp) {
	let mut x_left = left;
	let mut x_right = right;

	for &round_constant in ROUND_CONSTANTS.iter() {
		let t = ROUND_CONSTANTS[0] + x_left + round_constant;
		let t2 = t * t;
		let t5 = t2 * t2 * t;

		(x_left, x_right) = (x_right + t5, x_left);
	}

	(x_right, x_left)
}

macro_rules! fp_array {
    ( $($x:expr,)* ) => ([$(Fp::dangerous_new($x)),*]);
    ( $($x:expr),* ) => ([$(Fp::dangerous_new($x)),*]);
}

pub const ROUND_CONSTANTS: [Fp; 220] = fp_array!(
	[0, 0, 0, 0],
	[10834192717526305924, 16698740010641692384, 16538181000120689738, 377783109351314812],
	[5221564414513982296, 8778502761099945834, 14713263936961097835, 3155493624356573779],
	[6671724915643106057, 6305474247691119016, 17656128581227009646, 915078537332096441],
	[5479204579704553418, 9605960169276904010, 12534553728666658223, 3270235071970780903],
	[5954866882904877494, 1582249246030962115, 4571704482395834104, 577338099478639445],
	[16543643564296325891, 1944090962372330371, 9523530705789150333, 2206146165491101759],
	[2491565693030639387, 12433280022097990222, 5985586075564949428, 1295424467202497919],
	[17684590149948279933, 4592935702381159308, 2744733139080152047, 3071983465037180031],
	[9737629668848184468, 12829864615992652952, 17786295070710405063, 3024663088393611694],
	[12774516371319745117, 13523292013458299674, 11562970968207341344, 621038071572239382],
	[313055841212908740, 449681427842140844, 15131262285125695392, 2333350408850051697],
	[2956365394921583093, 6259514159899351017, 2927813060750362351, 2506220499363585410],
	[6650263186078541210, 15416056445831126554, 18397133529013185040, 2915288166401205586],
	[12839527618615088469, 3320486258427407067, 6331109552682315100, 3277337778348482810],
	[17211823202444331105, 15474897701653444618, 4370420819050605215, 3440398389866786407],
	[9276947723986449704, 4000857804817955393, 11221386452088129980, 2608721638932911145],
	[14062478049602074888, 7930087043553670402, 12777194227030267638, 2353679129807011427],
	[17636845037695889482, 2096779549429392446, 18275890495450762330, 995458177448247838],
	[5427192975025252509, 150478945785728972, 7349389769790292597, 3257944853341436916],
	[15708990139670285729, 4050073660643186866, 9813111531173520328, 1890077509444518573],
	[4349489547941313968, 12557256278576034134, 12156597846754818557, 2135515466902232075],
	[1242998266284832617, 18224757852811568438, 6477074107496818272, 811038566026200677],
	[1390646553578529002, 12928380458537033208, 4195196744759647444, 537721691222663224],
	[4201127539656101440, 13461411022839845741, 3885010396189466114, 867881731033512690],
	[8989470302037257432, 11145697536107583207, 18065264591046370134, 683835765982872428],
	[15757864229067841210, 18045588188928813699, 13287499488500099781, 1502154605442850829],
	[5446838547888314698, 4392894045632513109, 13727880035306393802, 611075600021634734],
	[17889069247925875701, 3072455025546860568, 15118013841567426434, 1867959223188355119],
	[13248671969849973983, 15828619497618833788, 6211030143725584432, 2570270038341898536],
	[9240518382378889173, 6464840753082954461, 2485044792729537001, 737184308635465992],
	[11716006243473622467, 15322531963486938927, 5252652478812041179, 1039280808490503504],
	[2985803742591262877, 4428235450564616426, 1389634455937969861, 1793307969381486267],
	[12129574069518190334, 11442590196057393105, 10516773510548692323, 41456503218142037],
	[14944482622837502035, 556843863271630707, 5198213101659067828, 248404998135292137],
	[18204319375653037551, 15015453613668011442, 11168826331432578234, 3456450606130734727],
	[10217866608315070077, 13141758883294979950, 10581880660538066772, 3353277560465587161],
	[18071208885415532040, 14607613781904771420, 5110969317080115001, 330823540475469934],
	[5736674879346364023, 13781219501875033386, 3697284945472432180, 1100262865555853225],
	[18169291100191085365, 2678340740896778747, 1927617635592731538, 228233861711687285],
	[12656081136347872901, 9595519771302497829, 4203029763835445946, 2383581316749636216],
	[7157231183359632603, 13128937249729647167, 14486925673106449503, 1114136586414062602],
	[9966152323156426770, 14120852415739344005, 11603874935639986809, 866098297856619465],
	[9909297017562502132, 14519099369179974273, 6421333437658023266, 1314200146951896814],
	[16778720279419192048, 18327780081823531108, 3462322712550678495, 893453898700951495],
	[6225344697859817858, 15862808334045089424, 407062611220997934, 881117985911043564],
	[284506663202285742, 3377569348167810768, 3419461151454385155, 1036186576084276254],
	[838061760867715010, 10122215429394320219, 3883399311005092566, 944592955361363519],
	[15804765882216971667, 9849264472108310066, 6934537179026438418, 2731518772789215229],
	[2526827349205142858, 10913233768228658861, 407231415748436590, 232858003085193428],
	[16798764600505518070, 17902748242149803067, 7041487216134407336, 1628443389926312129],
	[1283128860679674761, 6122043739684886226, 2804168607274294779, 2311372647383018211],
	[1574079379003698463, 6331460535652933491, 10219954970600319425, 3152059116555545519],
	[3886854613611119986, 13697511304182230587, 96741179045889544, 1760032316932760794],
	[5823622501224324128, 16021158361817928506, 17333830842335852442, 2872032706101963766],
	[6227579865426380641, 10378779565862873023, 120055669053867870, 2013458937431502831],
	[13930484995701793144, 15544286333213462530, 17719376559365934742, 856991514848668800],
	[1389483935479617837, 820760461375615566, 788817360027948046, 210173358465303704],
	[17221299923307728934, 10704676056470047710, 10760442799859628072, 999846493121221256],
	[8952405941016536992, 5724902947805084857, 6451601579071751977, 2603600896925274518],
	[4626439233890762207, 3592323067673456582, 2257660301351287683, 3458557424687522980],
	[1556753245904247243, 18323032797680267394, 2653674459025712827, 950753046016826145],
	[14023599497588995739, 140234758482847671, 4506292436835253078, 2788595176124634115],
	[7402811841192824004, 14249720874531031089, 11085252400758068304, 2932866097206938833],
	[562361892123369782, 4007965948617594429, 7118072195985795767, 3254864594131069228],
	[3669516781985950980, 10708765372782467413, 5607918828157621937, 2663710841414414926],
	[3609418313636571785, 2668315263721499947, 18303711552604927028, 346376522377743797],
	[15658378635663337506, 10883858747734932027, 14347724164229473175, 1423223790047203274],
	[12413392742109236040, 11179385802144551161, 11711449825251766998, 3259813914497378820],
	[14093642874934235087, 16179306751576998187, 9968433169429213158, 414874161811668801],
	[1344972922252166016, 2769465026374005091, 17127490412475777244, 285613946016017352],
	[5568334928661812301, 3601497583206118239, 17273381309772676419, 1073894573524065656],
	[17283705499331368206, 7031145378830393788, 13666376462786731444, 1349473089915569670],
	[18005034255997071050, 15423543838781661164, 15187202232026033630, 417755884214575139],
	[14323610721354517910, 17317949361498853076, 12775123956569568598, 3279343676296382906],
	[13452293414706098207, 8736804984795913596, 13735718854458111912, 2015647891317205934],
	[13168526436548450986, 16790742892477052990, 15874386590080922313, 726703870801965168],
	[4544127162750281744, 9470338074041413040, 14170696522724428941, 141990995365270619],
	[1694525293295876567, 17808434864207277346, 9031215809290303929, 1399891404818583431],
	[11947301858835185261, 15820400780164540267, 13901202581061808537, 1486897296255532359],
	[5528131191410205941, 9463985876997959277, 5048008405433876590, 21599808174075541],
	[8458227092636916436, 3266749551252138946, 10809864993423633816, 488364993906421765],
	[806147678318817216, 1560052838793803934, 17696796312912325526, 2317674495834118505],
	[2193675878861205416, 16088176932581590651, 11239148224231185438, 3330179581941294754],
	[782285173323498631, 12774481197198471042, 6299230045993159416, 701319953642332109],
	[11854860402093100165, 8758000795114519087, 9490019356781092766, 1205964530428130240],
	[18392934071959619737, 14707217422772095850, 1323660495021618312, 1385572384983423717],
	[10266066797928793941, 17115144619381563304, 5903513976916252396, 3063385445116692325],
	[18096977222146325675, 16809678846174021404, 5307712907946166745, 2702748315045053144],
	[6517695699130377477, 17385147105926100674, 3460530942870166878, 2564701731092018303],
	[9378959416531158031, 5882267706650934413, 6329401475789811704, 640825050548324072],
	[14144129205495429484, 192518222557943768, 10742729612730891442, 3159260808008805493],
	[1752100537330501730, 5320055101940114735, 9095983934081212530, 1885593804065895792],
	[16731726455008714085, 16776636426841757773, 6452553228459748958, 249495206848789265],
	[17392761655573683688, 3654426969548812735, 2068200774910483414, 1694729789253883498],
	[2665697038800249287, 14607107775435198114, 10825622027725955714, 3098929055453901421],
	[15369387451072994160, 17875329668610462630, 11583698281407124250, 1046932237274465013],
	[18385063906727820053, 2292984117359718865, 9628544895018525266, 1783696159000472825],
	[16731813986201249535, 17555416923727875009, 5572754414507688202, 1932866380446853806],
	[7135276271873192380, 4832465144808471338, 16783703536661114026, 3243645827718196108],
	[16835367480419880760, 1987695271227180407, 7363154225022053618, 2797586365308658378],
	[7695968563385494957, 10623598458859497606, 1004495343449180478, 678832088931812219],
	[18027556712885310070, 2433445839970085146, 16804008994698321793, 3284038656075748173],
	[4787551436209163110, 14821565565853630199, 7672483311223375670, 2566548868962705924],
	[2193251489965669252, 13091234452442750017, 4455373899120143383, 465950832772483833],
	[6805395765768281088, 10273958939318285436, 9462224693219917607, 2798852782197998844],
	[2807383755386420165, 2136463224878244657, 16892347909040951795, 2869193223303604013],
	[17957452016448766432, 17283289306817581167, 4218656337930009345, 1597644332645310555],
	[500006482211464607, 92220376008962430, 6576540465463866248, 3274516470952649941],
	[14912626745919856158, 15007305815045867273, 12373046314279861481, 2540701402905054564],
	[16645022630729417301, 820554481716682639, 14188613056356048736, 1145099895486937016],
	[4370253185580554740, 8506674583987576512, 358681988701711991, 2211615469911974422],
	[13602372334849309840, 12159783152584737010, 13905034968519427833, 107655603753648427],
	[2403645360167664028, 11340617246926363509, 17033224436231822714, 481568716636242819],
	[5193320073027195074, 13338925539697629196, 9401625605061613216, 3361233876282193778],
	[17014176222962589622, 7881834592944501435, 5886779195685312237, 553170195625172499],
	[18402597271459648575, 3938583961598849926, 1837786304968994502, 1205544003032106308],
	[7610954213876369393, 5074785655051455932, 6019383942713872145, 3057395071607424060],
	[13233645261566239609, 8833171421583547189, 10642397421650736007, 875465940858502126],
	[9100537088953638068, 6507429109599281694, 1673663078746277403, 3305137654711759093],
	[13166128198744824176, 12617963105792171738, 10658101106097345437, 607747320186438156],
	[10196780040033229891, 6840971145022684103, 7139435646838194073, 1234466617368314732],
	[16838645403280421273, 13058094885776780302, 1999124869342559721, 2974721404802208726],
	[5171590986953052380, 5171427819868772721, 4680439403178613974, 1554944057532014628],
	[2966297111537407604, 5477763646368763268, 9457132541081958345, 714457637916403093],
	[3031227127134110427, 12347876178980727859, 3317900151287793004, 742418744603606949],
	[15500505862559803234, 13495380583079275081, 10247649994837609275, 2441092789907778985],
	[17793439935592882734, 9713394790681562565, 8391739021582656255, 1893090433343848219],
	[16154064110714354918, 13640265593513197288, 5139651524683181331, 246083617969384174],
	[7260135743244897444, 12305454222824889777, 16505152584188217560, 36838202976508349],
	[16618847545592178091, 10488610505517392391, 14208717775138776761, 1784668783386136089],
	[12818203027058607872, 4067175115707219467, 4652790072818043687, 2970228423284051778],
	[5711935687673202297, 4039208602112220260, 5877818561561115551, 2794499267310985922],
	[17601397498627002470, 10091256799542398816, 8491761005330849893, 357848494511832611],
	[5446069692575369057, 3242202829204374730, 6866034376088825644, 1413469368418565463],
	[15309015244879602924, 17442673692507628365, 17415963849761027414, 3110291923097068096],
	[16821509524603124958, 10303769430133333642, 9715864977074710249, 2238236825876514765],
	[16945658705498390125, 12513450061784565968, 16133075175978773924, 3225151648532741926],
	[16526965249975328931, 10473654833534907999, 1426112179638053171, 1700436591837593671],
	[11900213388567228973, 17346000348921061886, 3516422356579497075, 2705996507106036703],
	[9655774166460892107, 14901377707040129479, 7082803017656647798, 1159034386068736855],
	[12142859648196309755, 12719857962177601344, 1878437569143591439, 3438228025777570794],
	[12787012846258043499, 6815287122866061043, 15931395346297799022, 2912919025244543854],
	[10455503608471522419, 16770909468848265113, 5983016019993324989, 2942593233839722988],
	[11023068778074522975, 17726256695974484871, 9227900710579446887, 1309370709378390739],
	[9333805446956024211, 15922038746998091822, 8735015778736223996, 3081915341648835972],
	[17485360828749314690, 1450956269596448838, 15030286873124482908, 1074842304715395042],
	[422511974895293062, 1561233381772924058, 8493817431126128061, 200314995853212511],
	[7049900937646962095, 12502115927081740856, 2526234754485544213, 988463237466629418],
	[4281011215858024462, 3971485694051144834, 10939245108113227585, 2497418774837434285],
	[12658986522279289123, 3279368008519731828, 6805597528388084604, 867846198144599216],
	[11368046901437649937, 1732100346005428125, 5082475250855835256, 1159080057815249629],
	[8153396388374499157, 8381020187174142052, 12828619054304093917, 1366896143507138105],
	[2919569776487892655, 277001296926877661, 8289572219431234862, 1041980453865090390],
	[14067063111950458768, 13224862745952567011, 14922097760533231413, 1502057419982851322],
	[7521031461189883133, 5094543219670195574, 6510665210154135030, 779870840191413925],
	[408578882723833766, 12711019197528577762, 2992650249994899145, 1180493340958687929],
	[591833454063537008, 18266947907650042742, 10749129230589870980, 2508074703816737021],
	[12599756973065714225, 16405794398001955694, 11321231239249600166, 2931221263881317914],
	[679526938356201450, 4210796439910441244, 3877630766514430810, 303754070455024261],
	[10233473420597396435, 5899960275371243061, 14555799369757761144, 604414353020591333],
	[18442491414799105447, 6389220913951150619, 13349996386439647812, 2372125749404521548],
	[1007681302992786998, 10683444457442117352, 15990226003974788813, 1356092416284362716],
	[15730360701538623314, 8704638547398074868, 8759849043337533125, 295630449276500774],
	[16042361843226977023, 13840165128453747265, 13313609665512116366, 2336161301722166695],
	[121604398132143010, 7970738617668116010, 2184130627801963164, 3079551717608590645],
	[9354403030573562520, 15477153574401628962, 17039447706996357446, 1109313339511365449],
	[15473764113967915638, 8326991513137407550, 13319001446505196208, 2868826418321621345],
	[13089793379415105016, 14350669114242469917, 3769050823981662933, 2436297073425318366],
	[519514839030251538, 6476624516446117094, 11545102449590069903, 2658864178757022542],
	[7184058358372500527, 10757372392011185019, 12065909138679837853, 1370502753325365741],
	[15160416449397916091, 6432582465750996192, 12766341374025696261, 580481110749748807],
	[16329835357128347049, 10237711792564910566, 6956696408995780017, 420955647405690097],
	[12100491881450236071, 6821138815993850783, 785587131455621898, 2422071240433994957],
	[4410912015722523269, 10373360259825827160, 13611860689389404292, 296157093124219099],
	[6374040984767189058, 16144240946147545003, 11972617677680033071, 2518971349520029884],
	[9332616369336592410, 17984925793023241925, 8582726634557116256, 2636979635239097552],
	[5198506613997373650, 17534556151750579581, 9881037226096896349, 2186593481795864077],
	[14436811687079900768, 12262822378621815991, 16508265225396915499, 3283460570267909489],
	[10828354478911313792, 15652412879963056964, 2759407489776610441, 2380799613807677515],
	[17481145368559014042, 5236884593575622349, 4467385803903003083, 2139388804221422730],
	[11943387285523992047, 8769675935517271122, 3511517491564504257, 3397553292967914424],
	[12609800444219821120, 16497005096084162153, 13941348069558973913, 2703318002770398803],
	[5582103750117970926, 13774932383429825505, 10767001646738039396, 624846514811890216],
	[16117284424815889235, 9737177859942922642, 12766056458235846437, 35252149832912338],
	[14203149705851144446, 4760035685746692529, 9927377228018367237, 1542609472616556590],
	[6786386883329652511, 18055221873362452706, 11145995947428494108, 2950115636541591382],
	[14867739732511053006, 2025254458133252713, 11537750736757694567, 1020246719921379385],
	[6076622470957011873, 10879878001820355777, 5016245258222260390, 3167407522783900043],
	[15845752894348713678, 15181560035649590594, 13450833926337221203, 1975589915779988688],
	[14906613621688819781, 6421284065844236408, 3785969990186916600, 1440310399685312274],
	[7440954816269722436, 12995352624423866916, 15098185674135618733, 2770548320638470113],
	[16289302341307971258, 4583481613904056068, 5949720929221554487, 1187353554622987519],
	[6735376043063618516, 12389921646879043685, 2883188006063945767, 1874820730740663242],
	[13543318172949637520, 2454421602490293673, 6703330727431343256, 880090175186451050],
	[9457721972496831312, 3773164110684017389, 16959454316692416134, 2714237943160218789],
	[7617186263273440066, 8172392984033514391, 9127696324914638367, 3356229418295133790],
	[2358365694283276283, 17475622287511570936, 993044104198737116, 3187755787141609236],
	[15683914401765855477, 2460288404653130586, 9302675043170641137, 2941008132113436291],
	[12521011913368106857, 16249834356236573350, 6805027147271522228, 1447118513827802772],
	[15588480510304292008, 17925205851789297104, 13551650690366535224, 2665439566663208795],
	[5906645651035700531, 2250007655960140745, 11266525648809846092, 2876717390015477356],
	[4340169829447915361, 8599615392319690559, 16201187492389732885, 3097085820040774851],
	[9972536893150553093, 17056404129687797845, 3626863979440503241, 2836080686497257608],
	[13040312167412210644, 12995558461271741884, 6760988187178805105, 523173646296498139],
	[272537285892501273, 3817531555623520640, 8183195194663262927, 3377282824875782689],
	[2236855280274392440, 4261386450763175633, 18312336767146011379, 437652390072731944],
	[13956837701977267784, 6399409393031406069, 16018398469602245861, 1669458847894004639],
	[6401538888404801593, 398220020251304791, 9816626497021203639, 1034145522518869282],
	[13361851600337121367, 11990142186475086999, 626115816643581777, 2548393802280709517],
	[67022516524338790, 2159408557479898039, 3220608626432186709, 1907842862707347149],
	[18018411233966872767, 6275719932708278181, 10008682989245375530, 416791370931675016],
	[4548792813906161732, 14962057080228046153, 5396340186264912609, 1336407134446314440],
	[10959066322543508781, 16754241516372196093, 17727770129998545179, 755255044914276524],
	[8938953766061283948, 16027397690322617859, 2641723356528531551, 3266757182650282266],
	[17874812307635951801, 346147286074858322, 5909890112065023934, 2233360523169808916],
	[11734559965890386260, 6708210059086347345, 8132569680193604523, 1729641749899116937],
	[1464798592992216329, 3035648681551011783, 6054911946525116924, 231283935945959193],
	[10638244795695933810, 14332444851852596508, 2882072417649478317, 2429280932290573038],
	[0, 0, 0, 0],
);

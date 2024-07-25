use isocountry::CountryCode;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Country {
    /// Afghanistan
    AFG = 0,
    /// Aland Islands
    ALA = 1,
    /// Albania
    ALB = 2,
    /// Algeria
    DZA = 3,
    /// American Samoa
    ASM = 4,
    /// Andorra
    AND = 5,
    /// Angola
    AGO = 6,
    /// Anguilla
    AIA = 7,
    /// Antarctica
    ATA = 8,
    /// Antigua and Barbuda
    ATG = 9,
    /// Argentina
    ARG = 10,
    /// Armenia
    ARM = 11,
    /// Aruba
    ABW = 12,
    /// Australia
    AUS = 13,
    /// Austria
    AUT = 14,
    /// Azerbaijan
    AZE = 15,
    /// Bahamas
    BHS = 16,
    /// Bahrain
    BHR = 17,
    /// Bangladesh
    BGD = 18,
    /// Barbados
    BRB = 19,
    /// Belarus
    BLR = 20,
    /// Belgium
    BEL = 21,
    /// Belize
    BLZ = 22,
    /// Benin
    BEN = 23,
    /// Bermuda
    BMU = 24,
    /// Bhutan
    BTN = 25,
    /// Bolivia (Plurinational State of)
    BOL = 26,
    /// Bonaire, Sint Eustatius and Saba
    BES = 27,
    /// Bosnia and Herzegovina
    BIH = 28,
    /// Botswanna
    BWA = 29,
    /// Bouvet Island
    BVT = 30,
    /// Brazil
    BRA = 31,
    /// British Indian Ocean Territory
    IOT = 32,
    /// Brunei Darussalam
    BRN = 33,
    /// Bulgaria
    BGR = 34,
    /// Burkina Faso
    BFA = 35,
    /// Burundi
    BDI = 36,
    /// Cabo Verde
    CPV = 37,
    /// Cambodia
    KHM = 38,
    /// Cameroon
    CMR = 39,
    /// Canada
    CAN = 40,
    /// Cayman Islands
    CYM = 41,
    /// Central African Republic
    CAF = 42,
    /// Chad
    TCD = 43,
    /// Chile
    CHL = 44,
    /// China
    CHN = 45,
    /// Christmas Island
    CXR = 46,
    /// Cocos (Keeling) Islands
    CCK = 47,
    /// Colombia
    COL = 48,
    /// Comoros
    COM = 49,
    /// Congo
    COG = 50,
    /// Congo (Democratic Republic of the)
    COD = 51,
    /// Cook Islands
    COK = 52,
    /// Costa Rica
    CRI = 53,
    /// Cote d'Ivoire
    CIV = 54,
    /// Croatia
    HRV = 55,
    /// Cuba
    CUB = 56,
    /// Curacao
    CUW = 57,
    /// Cyprus
    CYP = 58,
    /// Czechia
    CZE = 59,
    /// Denmark
    DNK = 60,
    /// Djibouti
    DJI = 61,
    /// Dominica
    DMA = 62,
    /// Dominican Republic
    DOM = 63,
    /// Ecuador
    ECU = 64,
    /// Egypt
    EGY = 65,
    /// El Salvador
    SLV = 66,
    /// Equatorial Guinea
    GNQ = 67,
    /// Eritrea
    ERI = 68,
    /// Estonia
    EST = 69,
    /// Ethiopia
    ETH = 70,
    /// Falkland Islands (Malvinas)
    FLK = 71,
    /// Faroe Islands
    FRO = 72,
    /// Fiji
    FJI = 73,
    /// Finland
    FIN = 74,
    /// France
    FRA = 75,
    /// French Guiana
    GUF = 76,
    /// French Polynesia
    PYF = 77,
    /// French Southern Territories
    ATF = 78,
    /// Gabon
    GAB = 79,
    /// Gambia
    GMB = 80,
    /// Georgia
    GEO = 81,
    /// Germany
    DEU = 82,
    /// Ghana
    GHA = 83,
    /// Gibraltar
    GIB = 84,
    /// Greece
    GRC = 85,
    /// Greenland
    GRL = 86,
    /// Grenada
    GRD = 87,
    /// Guadeloupe
    GLP = 88,
    /// Guam
    GUM = 89,
    /// Guatemala
    GTM = 90,
    /// Guernsey
    GGY = 91,
    /// Guinea
    GIN = 92,
    /// Guinea-Bissau
    GNB = 93,
    /// Guyana
    GUY = 94,
    /// Haiti
    HTI = 95,
    /// Heard Island and McDonald Islands
    HMD = 96,
    /// Holy See (Vatican City)
    VAT = 97,
    /// Honduras
    HND = 98,
    /// Hong Kong
    HKG = 99,
    /// Hungary
    HUN = 100,
    /// Iceland
    ISL = 101,
    /// India
    IND = 102,
    /// Indonesia
    IDN = 103,
    /// Iran (Islamic Republic of)
    IRN = 104,
    /// Iraq
    IRQ = 105,
    /// Ireland
    IRL = 106,
    /// Isle of Man
    IMN = 107,
    /// Israel
    ISR = 108,
    /// Italy
    ITA = 109,
    /// Jamaica
    JAM = 110,
    /// Japan
    JPN = 111,
    /// Jersey
    JEY = 112,
    /// Jordan
    JOR = 113,
    /// Kazakhstan
    KAZ = 114,
    /// Kenya
    KEN = 115,
    /// Kiribati
    KIR = 116,
    /// Korea (Democratic People's Republic of - North Korea)
    PRK = 117,
    /// Korea (Republic of - South Korea)
    KOR = 118,
    /// Kuwait
    KWT = 119,
    /// Kyrgyzstan
    KGZ = 120,
    /// Lao People's Democratic Republic
    LAO = 121,
    /// Latvia
    LVA = 122,
    /// Lebanon
    LBN = 123,
    /// Lesotho
    LSO = 124,
    /// Liberia
    LBR = 125,
    /// Libya
    LBY = 126,
    /// Liechtenstein
    LIE = 127,
    /// Lithuania
    LTU = 128,
    /// Luxembourg
    LUX = 129,
    /// Macao
    MAC = 130,
    /// Macedonia (the former Yugoslav Republic of)
    MKD = 131,
    /// Madagascar
    MDG = 132,
    /// Malawi
    MWI = 133,
    /// Malaysia
    MYS = 134,
    /// Maldives
    MDV = 135,
    /// Mali
    MLI = 136,
    /// Malta
    MLT = 137,
    /// Marshall Islands
    MHL = 138,
    /// Martinique
    MTQ = 139,
    /// Mauritania
    MRT = 140,
    /// Mauritius
    MUS = 141,
    /// Mayotte
    MYT = 142,
    /// Mexico
    MEX = 143,
    /// Micronesia (Federated States of)
    FSM = 144,
    /// Moldova (Republic of)
    MDA = 145,
    /// Monaco
    MCO = 146,
    /// Mongolia
    MNG = 147,
    /// Montenegro
    MNE = 148,
    /// Montserrat
    MSR = 149,
    /// Morocco
    MAR = 150,
    /// Mozambique
    MOZ = 151,
    /// Myanmar
    MMR = 152,
    /// Namibia
    NAM = 153,
    /// Nauru
    NRU = 154,
    /// Nepal
    NPL = 155,
    /// Netherlands
    NLD = 156,
    /// New Caledonia
    NCL = 157,
    /// New Zealand
    NZL = 158,
    /// Nicaragua
    NIC = 159,
    /// Niger
    NER = 160,
    /// Nigeria
    NGA = 161,
    /// Niue
    NIU = 162,
    /// Norfolk Island
    NFK = 163,
    /// Northern Mariana Islands
    MNP = 164,
    /// Norway
    NOR = 165,
    /// Oman
    OMN = 166,
    /// Pakistan
    PAK = 167,
    /// Palau
    PLW = 168,
    /// Palestine, State of
    PSE = 169,
    /// Panama
    PAN = 170,
    /// Papua New Guinea
    PNG = 171,
    /// Paraguay
    PRY = 172,
    /// Peru
    PER = 173,
    /// Philippines
    PHL = 174,
    /// Pitcairn
    PCN = 175,
    /// Poland
    POL = 176,
    /// Portugal
    PRT = 177,
    /// Puerto Rico
    PRI = 178,
    /// Qatar
    QAT = 179,
    /// Reunion
    REU = 180,
    /// Romania
    ROU = 181,
    /// Russian Federation
    RUS = 182,
    /// Rwanda
    RWA = 183,
    /// Saint Barthelemy
    BLM = 184,
    /// Saint Helena, Ascension and Tristan da Cunha
    SHN = 185,
    /// Saint Kitts and Nevis
    KNA = 186,
    /// Saint Lucia
    LCA = 187,
    /// Saint Martin (French part)
    MAF = 188,
    /// Saint Pierre and Miquelon
    SPM = 189,
    /// Saint Vincent and the Grenadines
    VCT = 190,
    /// Samoa
    WSM = 191,
    /// San Marino
    SMR = 192,
    /// Sao Tome and Principe
    STP = 193,
    /// Saudi Arabia
    SAU = 194,
    /// Senegal
    SEN = 195,
    /// Serbia
    SRB = 196,
    /// Seychelles
    SYC = 197,
    /// Sierra Leone
    SLE = 198,
    /// Singapore
    SGP = 199,
    /// Sint Maarten (Dutch part)
    SXM = 200,
    /// Slovakia
    SVK = 201,
    /// Slovenia
    SVN = 202,
    /// Solomon Islands
    SLB = 203,
    /// Somalia
    SOM = 204,
    /// South Africa
    ZAF = 205,
    /// South Georgia and the South Sandwich Islands
    SGS = 206,
    /// South Sudan
    SSD = 207,
    /// Spain
    ESP = 208,
    /// Sri Lanka
    LKA = 209,
    /// Sudan
    SDN = 210,
    /// Suriname
    SUR = 211,
    /// Svalbard and Jan Mayen
    SJM = 212,
    /// Swaziland
    SWZ = 213,
    /// Sweden
    SWE = 214,
    /// Switzerland
    CHE = 215,
    /// Syrian Arab Republic
    SYR = 216,
    /// Taiwan, Province of China
    TWN = 217,
    /// Tajikistan
    TJK = 218,
    /// Tanzania, United Republic of
    TZA = 219,
    /// Thailand
    THA = 220,
    /// Timor-Leste
    TLS = 221,
    /// Togo
    TGO = 222,
    /// Tokelau
    TKL = 223,
    /// Tonga
    TON = 224,
    /// Trinidad and Tobago
    TTO = 225,
    /// Tunisia
    TUN = 226,
    /// Turkey
    TUR = 227,
    /// Turkmenistan
    TKM = 228,
    /// Turks and Caicos Islands
    TCA = 229,
    /// Tuvalu
    TUV = 230,
    /// Uganda
    UGA = 231,
    /// Ukraine
    UKR = 232,
    /// United Arab Emirates
    ARE = 233,
    /// United Kingdom of Great Britain and Northern Ireland
    GBR = 234,
    /// United States of America
    USA = 235,
    /// United States Minor Outlying Islands
    UMI = 236,
    /// Uruguay
    URY = 237,
    /// Uzbekistan
    UZB = 238,
    /// Vanuatu
    VUT = 239,
    /// Venezuela (Boliverian Republic of)
    VEN = 240,
    /// Viet Nam
    VNM = 241,
    /// Virgin Islands (British)
    VGB = 242,
    /// Virgin Islands (U.S.)
    VIR = 243,
    /// Wallis and Futuna
    WLF = 244,
    /// Western Samoa
    ESH = 245,
    /// Yemen
    YEM = 246,
    /// Zambia
    ZMB = 247,
    /// Zimbabwe
    ZWE = 248,
}

impl TryFrom<u8> for Country {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let country = match value {
            0 => Self::AFG,
            1 => Self::ALA,
            2 => Self::ALB,
            3 => Self::DZA,
            4 => Self::ASM,
            5 => Self::AND,
            6 => Self::AGO,
            7 => Self::AIA,
            8 => Self::ATA,
            9 => Self::ATG,
            10 => Self::ARG,
            11 => Self::ARM,
            12 => Self::ABW,
            13 => Self::AUS,
            14 => Self::AUT,
            15 => Self::AZE,
            16 => Self::BHS,
            17 => Self::BHR,
            18 => Self::BGD,
            19 => Self::BRB,
            20 => Self::BLR,
            21 => Self::BEL,
            22 => Self::BLZ,
            23 => Self::BEN,
            24 => Self::BMU,
            25 => Self::BTN,
            26 => Self::BOL,
            27 => Self::BES,
            28 => Self::BIH,
            29 => Self::BWA,
            30 => Self::BVT,
            31 => Self::BRA,
            32 => Self::IOT,
            33 => Self::BRN,
            34 => Self::BGR,
            35 => Self::BFA,
            36 => Self::BDI,
            37 => Self::CPV,
            38 => Self::KHM,
            39 => Self::CMR,
            40 => Self::CAN,
            41 => Self::CYM,
            42 => Self::CAF,
            43 => Self::TCD,
            44 => Self::CHL,
            45 => Self::CHN,
            46 => Self::CXR,
            47 => Self::CCK,
            48 => Self::COL,
            49 => Self::COM,
            50 => Self::COG,
            51 => Self::COD,
            52 => Self::COK,
            53 => Self::CRI,
            54 => Self::CIV,
            55 => Self::HRV,
            56 => Self::CUB,
            57 => Self::CUW,
            58 => Self::CYP,
            59 => Self::CZE,
            60 => Self::DNK,
            61 => Self::DJI,
            62 => Self::DMA,
            63 => Self::DOM,
            64 => Self::ECU,
            65 => Self::EGY,
            66 => Self::SLV,
            67 => Self::GNQ,
            68 => Self::ERI,
            69 => Self::EST,
            70 => Self::ETH,
            71 => Self::FLK,
            72 => Self::FRO,
            73 => Self::FJI,
            74 => Self::FIN,
            75 => Self::FRA,
            76 => Self::GUF,
            77 => Self::PYF,
            78 => Self::ATF,
            79 => Self::GAB,
            80 => Self::GMB,
            81 => Self::GEO,
            82 => Self::DEU,
            83 => Self::GHA,
            84 => Self::GIB,
            85 => Self::GRC,
            86 => Self::GRL,
            87 => Self::GRD,
            88 => Self::GLP,
            89 => Self::GUM,
            90 => Self::GTM,
            91 => Self::GGY,
            92 => Self::GIN,
            93 => Self::GNB,
            94 => Self::GUY,
            95 => Self::HTI,
            96 => Self::HMD,
            97 => Self::VAT,
            98 => Self::HND,
            99 => Self::HKG,
            100 => Self::HUN,
            101 => Self::ISL,
            102 => Self::IND,
            103 => Self::IDN,
            104 => Self::IRN,
            105 => Self::IRQ,
            106 => Self::IRL,
            107 => Self::IMN,
            108 => Self::ISR,
            109 => Self::ITA,
            110 => Self::JAM,
            111 => Self::JPN,
            112 => Self::JEY,
            113 => Self::JOR,
            114 => Self::KAZ,
            115 => Self::KEN,
            116 => Self::KIR,
            117 => Self::PRK,
            118 => Self::KOR,
            119 => Self::KWT,
            120 => Self::KGZ,
            121 => Self::LAO,
            122 => Self::LVA,
            123 => Self::LBN,
            124 => Self::LSO,
            125 => Self::LBR,
            126 => Self::LBY,
            127 => Self::LIE,
            128 => Self::LTU,
            129 => Self::LUX,
            130 => Self::MAC,
            131 => Self::MKD,
            132 => Self::MDG,
            133 => Self::MWI,
            134 => Self::MYS,
            135 => Self::MDV,
            136 => Self::MLI,
            137 => Self::MLT,
            138 => Self::MHL,
            139 => Self::MTQ,
            140 => Self::MRT,
            141 => Self::MUS,
            142 => Self::MYT,
            143 => Self::MEX,
            144 => Self::FSM,
            145 => Self::MDA,
            146 => Self::MCO,
            147 => Self::MNG,
            148 => Self::MNE,
            149 => Self::MSR,
            150 => Self::MAR,
            151 => Self::MOZ,
            152 => Self::MMR,
            153 => Self::NAM,
            154 => Self::NRU,
            155 => Self::NPL,
            156 => Self::NLD,
            157 => Self::NCL,
            158 => Self::NZL,
            159 => Self::NIC,
            160 => Self::NER,
            161 => Self::NGA,
            162 => Self::NIU,
            163 => Self::NFK,
            164 => Self::MNP,
            165 => Self::NOR,
            166 => Self::OMN,
            167 => Self::PAK,
            168 => Self::PLW,
            169 => Self::PSE,
            170 => Self::PAN,
            171 => Self::PNG,
            172 => Self::PRY,
            173 => Self::PER,
            174 => Self::PHL,
            175 => Self::PCN,
            176 => Self::POL,
            177 => Self::PRT,
            178 => Self::PRI,
            179 => Self::QAT,
            180 => Self::REU,
            181 => Self::ROU,
            182 => Self::RUS,
            183 => Self::RWA,
            184 => Self::BLM,
            185 => Self::SHN,
            186 => Self::KNA,
            187 => Self::LCA,
            188 => Self::MAF,
            189 => Self::SPM,
            190 => Self::VCT,
            191 => Self::WSM,
            192 => Self::SMR,
            193 => Self::STP,
            194 => Self::SAU,
            195 => Self::SEN,
            196 => Self::SRB,
            197 => Self::SYC,
            198 => Self::SLE,
            199 => Self::SGP,
            200 => Self::SXM,
            201 => Self::SVK,
            202 => Self::SVN,
            203 => Self::SLB,
            204 => Self::SOM,
            205 => Self::ZAF,
            206 => Self::SGS,
            207 => Self::SSD,
            208 => Self::ESP,
            209 => Self::LKA,
            210 => Self::SDN,
            211 => Self::SUR,
            212 => Self::SJM,
            213 => Self::SWZ,
            214 => Self::SWE,
            215 => Self::CHE,
            216 => Self::SYR,
            217 => Self::TWN,
            218 => Self::TJK,
            219 => Self::TZA,
            220 => Self::THA,
            221 => Self::TLS,
            222 => Self::TGO,
            223 => Self::TKL,
            224 => Self::TON,
            225 => Self::TTO,
            226 => Self::TUN,
            227 => Self::TUR,
            228 => Self::TKM,
            229 => Self::TCA,
            230 => Self::TUV,
            231 => Self::UGA,
            232 => Self::UKR,
            233 => Self::ARE,
            234 => Self::GBR,
            235 => Self::USA,
            236 => Self::UMI,
            237 => Self::URY,
            238 => Self::UZB,
            239 => Self::VUT,
            240 => Self::VEN,
            241 => Self::VNM,
            242 => Self::VGB,
            243 => Self::VIR,
            244 => Self::WLF,
            245 => Self::ESH,
            246 => Self::YEM,
            247 => Self::ZMB,
            248 => Self::ZWE,
            _ => return Err("invalid country discriminator"),
        };
        Ok(country)
    }
}

impl From<&Country> for CountryCode {
    fn from(c: &Country) -> Self {
        match c {
            Country::AFG => Self::AFG,
            Country::ALA => Self::ALA,
            Country::ALB => Self::ALB,
            Country::DZA => Self::DZA,
            Country::ASM => Self::ASM,
            Country::AND => Self::AND,
            Country::AGO => Self::AGO,
            Country::AIA => Self::AIA,
            Country::ATA => Self::ATA,
            Country::ATG => Self::ATG,
            Country::ARG => Self::ARG,
            Country::ARM => Self::ARM,
            Country::ABW => Self::ABW,
            Country::AUS => Self::AUS,
            Country::AUT => Self::AUT,
            Country::AZE => Self::AZE,
            Country::BHS => Self::BHS,
            Country::BHR => Self::BHR,
            Country::BGD => Self::BGD,
            Country::BRB => Self::BRB,
            Country::BLR => Self::BLR,
            Country::BEL => Self::BEL,
            Country::BLZ => Self::BLZ,
            Country::BEN => Self::BEN,
            Country::BMU => Self::BMU,
            Country::BTN => Self::BTN,
            Country::BOL => Self::BOL,
            Country::BES => Self::BES,
            Country::BIH => Self::BIH,
            Country::BWA => Self::BWA,
            Country::BVT => Self::BVT,
            Country::BRA => Self::BRA,
            Country::IOT => Self::IOT,
            Country::BRN => Self::BRN,
            Country::BGR => Self::BGR,
            Country::BFA => Self::BFA,
            Country::BDI => Self::BDI,
            Country::CPV => Self::CPV,
            Country::KHM => Self::KHM,
            Country::CMR => Self::CMR,
            Country::CAN => Self::CAN,
            Country::CYM => Self::CYM,
            Country::CAF => Self::CAF,
            Country::TCD => Self::TCD,
            Country::CHL => Self::CHL,
            Country::CHN => Self::CHN,
            Country::CXR => Self::CXR,
            Country::CCK => Self::CCK,
            Country::COL => Self::COL,
            Country::COM => Self::COM,
            Country::COG => Self::COG,
            Country::COD => Self::COD,
            Country::COK => Self::COK,
            Country::CRI => Self::CRI,
            Country::CIV => Self::CIV,
            Country::HRV => Self::HRV,
            Country::CUB => Self::CUB,
            Country::CUW => Self::CUW,
            Country::CYP => Self::CYP,
            Country::CZE => Self::CZE,
            Country::DNK => Self::DNK,
            Country::DJI => Self::DJI,
            Country::DMA => Self::DMA,
            Country::DOM => Self::DOM,
            Country::ECU => Self::ECU,
            Country::EGY => Self::EGY,
            Country::SLV => Self::SLV,
            Country::GNQ => Self::GNQ,
            Country::ERI => Self::ERI,
            Country::EST => Self::EST,
            Country::ETH => Self::ETH,
            Country::FLK => Self::FLK,
            Country::FRO => Self::FRO,
            Country::FJI => Self::FJI,
            Country::FIN => Self::FIN,
            Country::FRA => Self::FRA,
            Country::GUF => Self::GUF,
            Country::PYF => Self::PYF,
            Country::ATF => Self::ATF,
            Country::GAB => Self::GAB,
            Country::GMB => Self::GMB,
            Country::GEO => Self::GEO,
            Country::DEU => Self::DEU,
            Country::GHA => Self::GHA,
            Country::GIB => Self::GIB,
            Country::GRC => Self::GRC,
            Country::GRL => Self::GRL,
            Country::GRD => Self::GRD,
            Country::GLP => Self::GLP,
            Country::GUM => Self::GUM,
            Country::GTM => Self::GTM,
            Country::GGY => Self::GGY,
            Country::GIN => Self::GIN,
            Country::GNB => Self::GNB,
            Country::GUY => Self::GUY,
            Country::HTI => Self::HTI,
            Country::HMD => Self::HMD,
            Country::VAT => Self::VAT,
            Country::HND => Self::HND,
            Country::HKG => Self::HKG,
            Country::HUN => Self::HUN,
            Country::ISL => Self::ISL,
            Country::IND => Self::IND,
            Country::IDN => Self::IDN,
            Country::IRN => Self::IRN,
            Country::IRQ => Self::IRQ,
            Country::IRL => Self::IRL,
            Country::IMN => Self::IMN,
            Country::ISR => Self::ISR,
            Country::ITA => Self::ITA,
            Country::JAM => Self::JAM,
            Country::JPN => Self::JPN,
            Country::JEY => Self::JEY,
            Country::JOR => Self::JOR,
            Country::KAZ => Self::KAZ,
            Country::KEN => Self::KEN,
            Country::KIR => Self::KIR,
            Country::PRK => Self::PRK,
            Country::KOR => Self::KOR,
            Country::KWT => Self::KWT,
            Country::KGZ => Self::KGZ,
            Country::LAO => Self::LAO,
            Country::LVA => Self::LVA,
            Country::LBN => Self::LBN,
            Country::LSO => Self::LSO,
            Country::LBR => Self::LBR,
            Country::LBY => Self::LBY,
            Country::LIE => Self::LIE,
            Country::LTU => Self::LTU,
            Country::LUX => Self::LUX,
            Country::MAC => Self::MAC,
            Country::MKD => Self::MKD,
            Country::MDG => Self::MDG,
            Country::MWI => Self::MWI,
            Country::MYS => Self::MYS,
            Country::MDV => Self::MDV,
            Country::MLI => Self::MLI,
            Country::MLT => Self::MLT,
            Country::MHL => Self::MHL,
            Country::MTQ => Self::MTQ,
            Country::MRT => Self::MRT,
            Country::MUS => Self::MUS,
            Country::MYT => Self::MYT,
            Country::MEX => Self::MEX,
            Country::FSM => Self::FSM,
            Country::MDA => Self::MDA,
            Country::MCO => Self::MCO,
            Country::MNG => Self::MNG,
            Country::MNE => Self::MNE,
            Country::MSR => Self::MSR,
            Country::MAR => Self::MAR,
            Country::MOZ => Self::MOZ,
            Country::MMR => Self::MMR,
            Country::NAM => Self::NAM,
            Country::NRU => Self::NRU,
            Country::NPL => Self::NPL,
            Country::NLD => Self::NLD,
            Country::NCL => Self::NCL,
            Country::NZL => Self::NZL,
            Country::NIC => Self::NIC,
            Country::NER => Self::NER,
            Country::NGA => Self::NGA,
            Country::NIU => Self::NIU,
            Country::NFK => Self::NFK,
            Country::MNP => Self::MNP,
            Country::NOR => Self::NOR,
            Country::OMN => Self::OMN,
            Country::PAK => Self::PAK,
            Country::PLW => Self::PLW,
            Country::PSE => Self::PSE,
            Country::PAN => Self::PAN,
            Country::PNG => Self::PNG,
            Country::PRY => Self::PRY,
            Country::PER => Self::PER,
            Country::PHL => Self::PHL,
            Country::PCN => Self::PCN,
            Country::POL => Self::POL,
            Country::PRT => Self::PRT,
            Country::PRI => Self::PRI,
            Country::QAT => Self::QAT,
            Country::REU => Self::REU,
            Country::ROU => Self::ROU,
            Country::RUS => Self::RUS,
            Country::RWA => Self::RWA,
            Country::BLM => Self::BLM,
            Country::SHN => Self::SHN,
            Country::KNA => Self::KNA,
            Country::LCA => Self::LCA,
            Country::MAF => Self::MAF,
            Country::SPM => Self::SPM,
            Country::VCT => Self::VCT,
            Country::WSM => Self::WSM,
            Country::SMR => Self::SMR,
            Country::STP => Self::STP,
            Country::SAU => Self::SAU,
            Country::SEN => Self::SEN,
            Country::SRB => Self::SRB,
            Country::SYC => Self::SYC,
            Country::SLE => Self::SLE,
            Country::SGP => Self::SGP,
            Country::SXM => Self::SXM,
            Country::SVK => Self::SVK,
            Country::SVN => Self::SVN,
            Country::SLB => Self::SLB,
            Country::SOM => Self::SOM,
            Country::ZAF => Self::ZAF,
            Country::SGS => Self::SGS,
            Country::SSD => Self::SSD,
            Country::ESP => Self::ESP,
            Country::LKA => Self::LKA,
            Country::SDN => Self::SDN,
            Country::SUR => Self::SUR,
            Country::SJM => Self::SJM,
            Country::SWZ => Self::SWZ,
            Country::SWE => Self::SWE,
            Country::CHE => Self::CHE,
            Country::SYR => Self::SYR,
            Country::TWN => Self::TWN,
            Country::TJK => Self::TJK,
            Country::TZA => Self::TZA,
            Country::THA => Self::THA,
            Country::TLS => Self::TLS,
            Country::TGO => Self::TGO,
            Country::TKL => Self::TKL,
            Country::TON => Self::TON,
            Country::TTO => Self::TTO,
            Country::TUN => Self::TUN,
            Country::TUR => Self::TUR,
            Country::TKM => Self::TKM,
            Country::TCA => Self::TCA,
            Country::TUV => Self::TUV,
            Country::UGA => Self::UGA,
            Country::UKR => Self::UKR,
            Country::ARE => Self::ARE,
            Country::GBR => Self::GBR,
            Country::USA => Self::USA,
            Country::UMI => Self::UMI,
            Country::URY => Self::URY,
            Country::UZB => Self::UZB,
            Country::VUT => Self::VUT,
            Country::VEN => Self::VEN,
            Country::VNM => Self::VNM,
            Country::VGB => Self::VGB,
            Country::VIR => Self::VIR,
            Country::WLF => Self::WLF,
            Country::ESH => Self::ESH,
            Country::YEM => Self::YEM,
            Country::ZMB => Self::ZMB,
            Country::ZWE => Self::ZWE,
        }
    }
}

impl From<Country> for CountryCode {
    fn from(c: Country) -> Self {
        Self::from(&c)
    }
}

impl From<&CountryCode> for Country {
    fn from(cc: &CountryCode) -> Self {
        match cc {
            CountryCode::AFG => Self::AFG,
            CountryCode::ALA => Self::ALA,
            CountryCode::ALB => Self::ALB,
            CountryCode::DZA => Self::DZA,
            CountryCode::ASM => Self::ASM,
            CountryCode::AND => Self::AND,
            CountryCode::AGO => Self::AGO,
            CountryCode::AIA => Self::AIA,
            CountryCode::ATA => Self::ATA,
            CountryCode::ATG => Self::ATG,
            CountryCode::ARG => Self::ARG,
            CountryCode::ARM => Self::ARM,
            CountryCode::ABW => Self::ABW,
            CountryCode::AUS => Self::AUS,
            CountryCode::AUT => Self::AUT,
            CountryCode::AZE => Self::AZE,
            CountryCode::BHS => Self::BHS,
            CountryCode::BHR => Self::BHR,
            CountryCode::BGD => Self::BGD,
            CountryCode::BRB => Self::BRB,
            CountryCode::BLR => Self::BLR,
            CountryCode::BEL => Self::BEL,
            CountryCode::BLZ => Self::BLZ,
            CountryCode::BEN => Self::BEN,
            CountryCode::BMU => Self::BMU,
            CountryCode::BTN => Self::BTN,
            CountryCode::BOL => Self::BOL,
            CountryCode::BES => Self::BES,
            CountryCode::BIH => Self::BIH,
            CountryCode::BWA => Self::BWA,
            CountryCode::BVT => Self::BVT,
            CountryCode::BRA => Self::BRA,
            CountryCode::IOT => Self::IOT,
            CountryCode::BRN => Self::BRN,
            CountryCode::BGR => Self::BGR,
            CountryCode::BFA => Self::BFA,
            CountryCode::BDI => Self::BDI,
            CountryCode::CPV => Self::CPV,
            CountryCode::KHM => Self::KHM,
            CountryCode::CMR => Self::CMR,
            CountryCode::CAN => Self::CAN,
            CountryCode::CYM => Self::CYM,
            CountryCode::CAF => Self::CAF,
            CountryCode::TCD => Self::TCD,
            CountryCode::CHL => Self::CHL,
            CountryCode::CHN => Self::CHN,
            CountryCode::CXR => Self::CXR,
            CountryCode::CCK => Self::CCK,
            CountryCode::COL => Self::COL,
            CountryCode::COM => Self::COM,
            CountryCode::COG => Self::COG,
            CountryCode::COD => Self::COD,
            CountryCode::COK => Self::COK,
            CountryCode::CRI => Self::CRI,
            CountryCode::CIV => Self::CIV,
            CountryCode::HRV => Self::HRV,
            CountryCode::CUB => Self::CUB,
            CountryCode::CUW => Self::CUW,
            CountryCode::CYP => Self::CYP,
            CountryCode::CZE => Self::CZE,
            CountryCode::DNK => Self::DNK,
            CountryCode::DJI => Self::DJI,
            CountryCode::DMA => Self::DMA,
            CountryCode::DOM => Self::DOM,
            CountryCode::ECU => Self::ECU,
            CountryCode::EGY => Self::EGY,
            CountryCode::SLV => Self::SLV,
            CountryCode::GNQ => Self::GNQ,
            CountryCode::ERI => Self::ERI,
            CountryCode::EST => Self::EST,
            CountryCode::ETH => Self::ETH,
            CountryCode::FLK => Self::FLK,
            CountryCode::FRO => Self::FRO,
            CountryCode::FJI => Self::FJI,
            CountryCode::FIN => Self::FIN,
            CountryCode::FRA => Self::FRA,
            CountryCode::GUF => Self::GUF,
            CountryCode::PYF => Self::PYF,
            CountryCode::ATF => Self::ATF,
            CountryCode::GAB => Self::GAB,
            CountryCode::GMB => Self::GMB,
            CountryCode::GEO => Self::GEO,
            CountryCode::DEU => Self::DEU,
            CountryCode::GHA => Self::GHA,
            CountryCode::GIB => Self::GIB,
            CountryCode::GRC => Self::GRC,
            CountryCode::GRL => Self::GRL,
            CountryCode::GRD => Self::GRD,
            CountryCode::GLP => Self::GLP,
            CountryCode::GUM => Self::GUM,
            CountryCode::GTM => Self::GTM,
            CountryCode::GGY => Self::GGY,
            CountryCode::GIN => Self::GIN,
            CountryCode::GNB => Self::GNB,
            CountryCode::GUY => Self::GUY,
            CountryCode::HTI => Self::HTI,
            CountryCode::HMD => Self::HMD,
            CountryCode::VAT => Self::VAT,
            CountryCode::HND => Self::HND,
            CountryCode::HKG => Self::HKG,
            CountryCode::HUN => Self::HUN,
            CountryCode::ISL => Self::ISL,
            CountryCode::IND => Self::IND,
            CountryCode::IDN => Self::IDN,
            CountryCode::IRN => Self::IRN,
            CountryCode::IRQ => Self::IRQ,
            CountryCode::IRL => Self::IRL,
            CountryCode::IMN => Self::IMN,
            CountryCode::ISR => Self::ISR,
            CountryCode::ITA => Self::ITA,
            CountryCode::JAM => Self::JAM,
            CountryCode::JPN => Self::JPN,
            CountryCode::JEY => Self::JEY,
            CountryCode::JOR => Self::JOR,
            CountryCode::KAZ => Self::KAZ,
            CountryCode::KEN => Self::KEN,
            CountryCode::KIR => Self::KIR,
            CountryCode::PRK => Self::PRK,
            CountryCode::KOR => Self::KOR,
            CountryCode::KWT => Self::KWT,
            CountryCode::KGZ => Self::KGZ,
            CountryCode::LAO => Self::LAO,
            CountryCode::LVA => Self::LVA,
            CountryCode::LBN => Self::LBN,
            CountryCode::LSO => Self::LSO,
            CountryCode::LBR => Self::LBR,
            CountryCode::LBY => Self::LBY,
            CountryCode::LIE => Self::LIE,
            CountryCode::LTU => Self::LTU,
            CountryCode::LUX => Self::LUX,
            CountryCode::MAC => Self::MAC,
            CountryCode::MKD => Self::MKD,
            CountryCode::MDG => Self::MDG,
            CountryCode::MWI => Self::MWI,
            CountryCode::MYS => Self::MYS,
            CountryCode::MDV => Self::MDV,
            CountryCode::MLI => Self::MLI,
            CountryCode::MLT => Self::MLT,
            CountryCode::MHL => Self::MHL,
            CountryCode::MTQ => Self::MTQ,
            CountryCode::MRT => Self::MRT,
            CountryCode::MUS => Self::MUS,
            CountryCode::MYT => Self::MYT,
            CountryCode::MEX => Self::MEX,
            CountryCode::FSM => Self::FSM,
            CountryCode::MDA => Self::MDA,
            CountryCode::MCO => Self::MCO,
            CountryCode::MNG => Self::MNG,
            CountryCode::MNE => Self::MNE,
            CountryCode::MSR => Self::MSR,
            CountryCode::MAR => Self::MAR,
            CountryCode::MOZ => Self::MOZ,
            CountryCode::MMR => Self::MMR,
            CountryCode::NAM => Self::NAM,
            CountryCode::NRU => Self::NRU,
            CountryCode::NPL => Self::NPL,
            CountryCode::NLD => Self::NLD,
            CountryCode::NCL => Self::NCL,
            CountryCode::NZL => Self::NZL,
            CountryCode::NIC => Self::NIC,
            CountryCode::NER => Self::NER,
            CountryCode::NGA => Self::NGA,
            CountryCode::NIU => Self::NIU,
            CountryCode::NFK => Self::NFK,
            CountryCode::MNP => Self::MNP,
            CountryCode::NOR => Self::NOR,
            CountryCode::OMN => Self::OMN,
            CountryCode::PAK => Self::PAK,
            CountryCode::PLW => Self::PLW,
            CountryCode::PSE => Self::PSE,
            CountryCode::PAN => Self::PAN,
            CountryCode::PNG => Self::PNG,
            CountryCode::PRY => Self::PRY,
            CountryCode::PER => Self::PER,
            CountryCode::PHL => Self::PHL,
            CountryCode::PCN => Self::PCN,
            CountryCode::POL => Self::POL,
            CountryCode::PRT => Self::PRT,
            CountryCode::PRI => Self::PRI,
            CountryCode::QAT => Self::QAT,
            CountryCode::REU => Self::REU,
            CountryCode::ROU => Self::ROU,
            CountryCode::RUS => Self::RUS,
            CountryCode::RWA => Self::RWA,
            CountryCode::BLM => Self::BLM,
            CountryCode::SHN => Self::SHN,
            CountryCode::KNA => Self::KNA,
            CountryCode::LCA => Self::LCA,
            CountryCode::MAF => Self::MAF,
            CountryCode::SPM => Self::SPM,
            CountryCode::VCT => Self::VCT,
            CountryCode::WSM => Self::WSM,
            CountryCode::SMR => Self::SMR,
            CountryCode::STP => Self::STP,
            CountryCode::SAU => Self::SAU,
            CountryCode::SEN => Self::SEN,
            CountryCode::SRB => Self::SRB,
            CountryCode::SYC => Self::SYC,
            CountryCode::SLE => Self::SLE,
            CountryCode::SGP => Self::SGP,
            CountryCode::SXM => Self::SXM,
            CountryCode::SVK => Self::SVK,
            CountryCode::SVN => Self::SVN,
            CountryCode::SLB => Self::SLB,
            CountryCode::SOM => Self::SOM,
            CountryCode::ZAF => Self::ZAF,
            CountryCode::SGS => Self::SGS,
            CountryCode::SSD => Self::SSD,
            CountryCode::ESP => Self::ESP,
            CountryCode::LKA => Self::LKA,
            CountryCode::SDN => Self::SDN,
            CountryCode::SUR => Self::SUR,
            CountryCode::SJM => Self::SJM,
            CountryCode::SWZ => Self::SWZ,
            CountryCode::SWE => Self::SWE,
            CountryCode::CHE => Self::CHE,
            CountryCode::SYR => Self::SYR,
            CountryCode::TWN => Self::TWN,
            CountryCode::TJK => Self::TJK,
            CountryCode::TZA => Self::TZA,
            CountryCode::THA => Self::THA,
            CountryCode::TLS => Self::TLS,
            CountryCode::TGO => Self::TGO,
            CountryCode::TKL => Self::TKL,
            CountryCode::TON => Self::TON,
            CountryCode::TTO => Self::TTO,
            CountryCode::TUN => Self::TUN,
            CountryCode::TUR => Self::TUR,
            CountryCode::TKM => Self::TKM,
            CountryCode::TCA => Self::TCA,
            CountryCode::TUV => Self::TUV,
            CountryCode::UGA => Self::UGA,
            CountryCode::UKR => Self::UKR,
            CountryCode::ARE => Self::ARE,
            CountryCode::GBR => Self::GBR,
            CountryCode::USA => Self::USA,
            CountryCode::UMI => Self::UMI,
            CountryCode::URY => Self::URY,
            CountryCode::UZB => Self::UZB,
            CountryCode::VUT => Self::VUT,
            CountryCode::VEN => Self::VEN,
            CountryCode::VNM => Self::VNM,
            CountryCode::VGB => Self::VGB,
            CountryCode::VIR => Self::VIR,
            CountryCode::WLF => Self::WLF,
            CountryCode::ESH => Self::ESH,
            CountryCode::YEM => Self::YEM,
            CountryCode::ZMB => Self::ZMB,
            CountryCode::ZWE => Self::ZWE,
        }
    }
}

impl From<CountryCode> for Country {
    fn from(cc: CountryCode) -> Self {
        Self::from(&cc)
    }
}

use std::{
    ffi::OsStr,
    fmt::Debug,
    process::{Command, Stdio},
};

pub fn exec<I, S>(cmd: S, args: I) -> Option<String>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Debug,
{
    let output = Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub async fn exec_async<S, I>(cmd: S, args: I) -> Option<String>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr> + Debug,
{
    let output = tokio::process::Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .await
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn get_file_name(path: &str) -> Option<String> {
    let path = path.replace('\\', "/");
    let name = path.split('/').next_back()?.split('.').next()?.trim();
    Some(name.into())
}
pub fn get_pid_name(id: u32) -> Option<String> {
    std::fs::read_to_string(format!("/proc/{id}/comm").as_str())
        .ok()
        .map(|i| i.trim().to_string())
}

pub fn get_ppid(id: u32) -> Option<u32> {
    if let Some(ppid) = exec(
        "grep",
        ["-i", "-F", "PPid:", format!("/proc/{id}/status").as_str()],
    ) {
        let ppid = ppid.split(':').next_back()?.trim();
        let ppid: u32 = ppid.parse().ok()?;
        return Some(ppid);
    }
    None
}

#[cfg(windows)]
pub async fn wmi_query<T: serde::de::DeserializeOwned>() -> Option<Vec<T>> {
    use wmi::{COMLibrary, WMIConnection};
    let com = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com).ok()?;
    let results: Vec<T> = wmi_con.async_query().await.ok()?;
    Some(results)
}

#[cfg(target_os = "android")]
unsafe extern "C" {
    fn __system_property_get(name: *const libc::c_char, value: *mut libc::c_char) -> i32;
}

#[cfg(target_os = "android")]
pub fn get_property(property: &str) -> Option<String> {
    use std::ffi::{CStr, CString};
    use std::io;

    let prop_cstr = CString::new(property)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
        .ok()?;
    let mut buffer = [0i8; 92];

    let result = unsafe {
        __system_property_get(
            prop_cstr.as_ptr() as *mut u8,
            buffer.as_mut_ptr() as *mut u8,
        )
    };

    if result < 0 {
        return None;
    }

    let value = unsafe {
        CStr::from_ptr(buffer.as_ptr() as *const u8)
            .to_string_lossy()
            .into_owned()
    };

    if value.is_empty() {
        return None;
    }

    Some(value)
}

pub const CPU_MAPPINGS: &[(&str, &str)] = &[
    // SM
    (
        "SM8750-AC",
        "Qualcomm Snapdragon 8 Elite for Galaxy [SM8750-AC]",
    ),
    ("SM8750-3", "Qualcomm Snapdragon 8 Elite [SM8750-3]"),
    ("SM8750", "Qualcomm Snapdragon 8 Elite [SM8750]"),
    ("SM8635", "Qualcomm Snapdragon 8s Gen 3 [SM8635]"),
    (
        "SM8650-AC",
        "Qualcomm Snapdragon 8 Gen 3 for Galaxy [SM8650-AC]",
    ),
    ("SM8650", "Qualcomm Snapdragon 8 Gen 3 [SM8650]"),
    (
        "SM8550-AC",
        "Qualcomm Snapdragon 8 Gen 2 for Galaxy [SM8550-AC]",
    ),
    ("SM8550", "Qualcomm Snapdragon 8 Gen 2 [SM8550]"),
    ("SM8475", "Qualcomm Snapdragon 8+ Gen 1 [SM8475]"),
    ("SM8450", "Qualcomm Snapdragon 8 Gen 1 [SM8450]"),
    ("SM7675", "Qualcomm Snapdragon 7+ Gen 3 [SM7675]"),
    ("SM7635", "Qualcomm Snapdragon 7s Gen 3 [SM7635]"),
    ("SM7550", "Qualcomm Snapdragon 7 Gen 3 [SM7550]"),
    ("SM7475", "Qualcomm Snapdragon 7+ Gen 2 [SM7550]"),
    ("SM7435", "Qualcomm Snapdragon 7s Gen 2 [SM7435]"),
    ("SM7450", "Qualcomm Snapdragon 7 Gen 1 [SM7450]"),
    ("SM6375-AC", "Qualcomm Snapdragon 6s Gen 3 [SM6375-AC]"),
    ("SM6475", "Qualcomm Snapdragon 6 Gen 3 [SM6475]"),
    ("SM6115", "Qualcomm Snapdragon 6s Gen 1 [SM6115]"),
    ("SM6450", "Qualcomm Snapdragon 6 Gen 1 [SM6450]"),
    ("SM4635", "Qualcomm Snapdragon 4s Gen 2 [SM4635]"),
    ("SM4450", "Qualcomm Snapdragon 4 Gen 2 [SM4450]"),
    ("SM4375", "Qualcomm Snapdragon 4 Gen 1 [SM4375]"),
    // MTK
    ("MT6991", "MediaTek Dimensity 9400 [MT6991]"),
    ("MT6991Z", "MediaTek Dimensity 9400 [MT6991Z]"),
    ("MT6989Z", "MediaTek Dimensity 9300+ [MT6989Z]"),
    ("MT8796Z", "MediaTek Dimensity 9300+ [MT8796Z]"),
    ("MT6989", "MediaTek Dimensity 9300 [MT6989]"),
    ("MT8796", "MediaTek Dimensity 9300 [MT8796]"),
    ("MT6985W", "MediaTek Dimensity 9200+ [MT6985W]"),
    ("MT6985", "MediaTek Dimensity 9200 [MT6985]"),
    ("MT6983W", "MediaTek Dimensity 9000+ [MT6983W]"),
    ("MT8798Z/T", "MediaTek Dimensity 9000+ [MT8798Z/T]"),
    ("MT6983Z", "MediaTek Dimensity 9000 [MT6983Z]"),
    ("MT8798Z/C", "MediaTek Dimensity 9000 [MT8798Z/C]"),
];

pub fn detect_cpu(name: &str) -> Option<String> {
    CPU_MAPPINGS.iter().find_map(|i| {
        if i.0 == name {
            Some(i.1.to_string())
        } else {
            None
        }
    })
}

// https://www.autoitscript.com/autoit3/docs/appendix/OSLangCodes.htm
const LOCALE_MAPPINGS: &[(&str, &str, &str, &str)] = &[
    ("0004", "4", "zh-CHS", "Chinese - Simplified"),
    ("0401", "1025", "ar-SA", "Arabic - Saudi Arabia"),
    ("0402", "1026", "bg-BG", "Bulgarian - Bulgaria"),
    ("0403", "1027", "ca-ES", "Catalan - Spain"),
    ("0404", "1028", "zh-TW", "Chinese (Traditional) - Taiwan"),
    ("0405", "1029", "cs-CZ", "Czech - Czech Republic"),
    ("0406", "1030", "da-DK", "Danish - Denmark"),
    ("0407", "1031", "de-DE", "German - Germany"),
    ("0408", "1032", "el-GR", "Greek - Greece"),
    ("0409", "1033", "en-US", "English - United States"),
    ("040A", "1034", "es-ES_tradnl", "Spanish - Spain"),
    ("040B", "1035", "fi-FI", "Finnish - Finland"),
    ("040C", "1036", "fr-FR", "French - France"),
    ("040D", "1037", "he-IL", "Hebrew - Israel"),
    ("040E", "1038", "hu-HU", "Hungarian - Hungary"),
    ("040F", "1039", "is-IS", "Icelandic - Iceland"),
    ("0410", "1040", "it-IT", "Italian - Italy"),
    ("0411", "1041", "ja-JP", "Japanese - Japan"),
    ("0412", "1042", "ko-KR", "Korean - Korea"),
    ("0413", "1043", "nl-NL", "Dutch - Netherlands"),
    ("0414", "1044", "nb-NO", "Norwegian (Bokmål) - Norway"),
    ("0415", "1045", "pl-PL", "Polish - Poland"),
    ("0416", "1046", "pt-BR", "Portuguese - Brazil"),
    ("0417", "1047", "rm-CH", "Romansh - Switzerland"),
    ("0418", "1048", "ro-RO", "Romanian - Romania"),
    ("0419", "1049", "ru-RU", "Russian - Russia"),
    ("041A", "1050", "hr-HR", "Croatian - Croatia"),
    ("041B", "1051", "sk-SK", "Slovak - Slovakia"),
    ("041C", "1052", "sq-AL", "Albanian - Albania"),
    ("041D", "1053", "sv-SE", "Swedish - Sweden"),
    ("041E", "1054", "th-TH", "Thai - Thailand"),
    ("041F", "1055", "tr-TR", "Turkish - Turkey"),
    ("0420", "1056", "ur-PK", "Urdu - Pakistan"),
    ("0421", "1057", "id-ID", "Indonesian - Indonesia"),
    ("0422", "1058", "uk-UA", "Ukrainian - Ukraine"),
    ("0423", "1059", "be-BY", "Belarusian - Belarus"),
    ("0424", "1060", "sl-SI", "Slovenian - Slovenia"),
    ("0425", "1061", "et-EE", "Estonian - Estonia"),
    ("0426", "1062", "lv-LV", "Latvian - Latvia"),
    ("0427", "1063", "lt-LT", "Lithuanian - Lithuanian"),
    (
        "0428",
        "1064",
        "tg-Cyrl-TJ",
        "Tajik (Cyrillic) - Tajikistan",
    ),
    ("0429", "1065", "fa-IR", "Persian - Iran"),
    ("042A", "1066", "vi-VN", "Vietnamese - Vietnam"),
    ("042B", "1067", "hy-AM", "Armenian - Armenia"),
    ("042C", "1068", "az-Latn-AZ", "Azeri (Latin) - Azerbaijan"),
    ("042D", "1069", "eu-ES", "Basque - Basque"),
    ("042E", "1070", "hsb-DE", "Upper Sorbian - Germany"),
    ("042F", "1071", "mk-MK", "Macedonian - Macedonia"),
    ("0432", "1074", "tn-ZA", "Setswana / Tswana - South Africa"),
    ("0434", "1076", "xh-ZA", "isiXhosa - South Africa"),
    ("0435", "1077", "zu-ZA", "isiZulu - South Africa"),
    ("0436", "1078", "af-ZA", "Afrikaans - South Africa"),
    ("0437", "1079", "ka-GE", "Georgian - Georgia"),
    ("0438", "1080", "fo-FO", "Faroese - Faroe Islands"),
    ("0439", "1081", "hi-IN", "Hindi - India"),
    ("043A", "1082", "mt-MT", "Maltese - Malta"),
    ("043B", "1083", "se-NO", "Sami (Northern) - Norway"),
    ("043e", "1086", "ms-MY", "Malay - Malaysia"),
    ("043F", "1087", "kk-KZ", "Kazakh - Kazakhstan"),
    ("0440", "1088", "ky-KG", "Kyrgyz - Kyrgyzstan"),
    ("0441", "1089", "sw-KE", "Swahili - Kenya"),
    ("0442", "1090", "tk-TM", "Turkmen - Turkmenistan"),
    ("0443", "1091", "uz-Latn-UZ", "Uzbek (Latin) - Uzbekistan"),
    ("0444", "1092", "tt-RU", "Tatar - Russia"),
    ("0445", "1093", "bn-IN", "Bangla - Bangladesh"),
    ("0446", "1094", "pa-IN", "Punjabi - India"),
    ("0447", "1095", "gu-IN", "Gujarati - India"),
    ("0448", "1096", "or-IN", "Oriya - India"),
    ("0449", "1097", "ta-IN", "Tamil - India"),
    ("044A", "1098", "te-IN", "Telugu - India"),
    ("044B", "1099", "kn-IN", "Kannada - India"),
    ("044C", "1100", "ml-IN", "Malayalam - India"),
    ("044D", "1101", "as-IN", "Assamese - India"),
    ("044E", "1102", "mr-IN", "Marathi - India"),
    ("044F", "1103", "sa-IN", "Sanskrit - India"),
    ("0450", "1104", "mn-MN", "Mongolian (Cyrillic) - Mongolia"),
    ("0451", "1105", "bo-CN", "Tibetan - China"),
    ("0452", "1106", "cy-GB", "Welsh - United Kingdom"),
    ("0453", "1107", "km-KH", "Khmer - Cambodia"),
    ("0454", "1108", "lo-LA", "Lao - Lao PDR"),
    ("0456", "1110", "gl-ES", "Galician - Spain"),
    ("0457", "1111", "kok-IN", "Konkani - India"),
    ("0459", "1113", "sd-Deva-IN", "(reserved) - (reserved)"),
    ("045A", "1114", "syr-SY", "Syriac - Syria"),
    ("045B", "1115", "si-LK", "Sinhala - Sri Lanka"),
    ("045C", "1116", "chr-Cher-US", "Cherokee - Cherokee"),
    (
        "045D",
        "1117",
        "iu-Cans-CA",
        "Inuktitut (Canadian_Syllabics) - Canada",
    ),
    ("045E", "1118", "am-ET", "Amharic - Ethiopia"),
    ("0461", "1121", "ne-NP", "Nepali - Nepal"),
    ("0462", "1122", "fy-NL", "Frisian - Netherlands"),
    ("0463", "1123", "ps-AF", "Pashto - Afghanistan"),
    ("0464", "1124", "fil-PH", "Filipino - Philippines"),
    ("0465", "1125", "dv-MV", "Divehi - Maldives"),
    ("0468", "1128", "ha-Latn-NG", "Hausa - Nigeria"),
    ("046A", "1130", "yo-NG", "Yoruba - Nigeria"),
    ("046B", "1131", "quz-BO", "Quechua - Bolivia"),
    ("046C", "1132", "nso-ZA", "Sesotho sa Leboa - South Africa"),
    ("046D", "1133", "ba-RU", "Bashkir - Russia"),
    ("046E", "1134", "lb-LU", "Luxembourgish - Luxembourg"),
    ("046F", "1135", "kl-GL", "Greenlandic - Greenland"),
    ("0470", "1136", "ig-NG", "Igbo - Nigeria"),
    ("0473", "1139", "ti-ET", "Tigrinya - Ethiopia"),
    ("0475", "1141", "haw-US", "Hawiian - United States"),
    ("0478", "1144", "ii-CN", "Yi - China"),
    ("047A", "1146", "arn-CL", "Mapudungun - Chile"),
    ("047C", "1148", "moh-CA", "Mohawk - Canada"),
    ("047E", "1150", "br-FR", "Breton - France"),
    ("0480", "1152", "ug-CN", "Uyghur - China"),
    ("0481", "1153", "mi-NZ", "Maori - New Zealand"),
    ("0482", "1154", "oc-FR", "Occitan - France"),
    ("0483", "1155", "co-FR", "Corsican - France"),
    ("0484", "1156", "gsw-FR", "Alsatian - France"),
    ("0485", "1157", "sah-RU", "Sakha - Russia"),
    ("0486", "1158", "quc-Latn-GT", "K'iche - Guatemala"),
    ("0487", "1159", "rw-RW", "Kinyarwanda - Rwanda"),
    ("0488", "1160", "wo-SN", "Wolof - Senegal"),
    ("048C", "1164", "prs-AF", "Dari - Afghanistan"),
    ("0491", "1169", "gd-GB", "Scottish Gaelic - United Kingdom"),
    ("0492", "1170", "ku-Arab-IQ", "Central Kurdish - Iraq"),
    ("0801", "2049", "ar-IQ", "Arabic - Iraq"),
    ("0803", "2051", "ca-ES-valencia", "Valencian - Valencia"),
    ("0804", "2052", "zh-CN", "Chinese (Simplified) - China"),
    ("0807", "2055", "de-CH", "German - Switzerland"),
    ("0809", "2057", "en-GB", "English - United Kingdom"),
    ("080A", "2058", "es-MX", "Spanish - Mexico"),
    ("080C", "2060", "fr-BE", "French - Belgium"),
    ("0810", "2064", "it-CH", "Italian - Switzerland"),
    ("0813", "2067", "nl-BE", "Dutch - Belgium"),
    ("0814", "2068", "nn-NO", "Norwegian (Nynorsk) - Norway"),
    ("0816", "2070", "pt-PT", "Portuguese - Portugal"),
    (
        "081A",
        "2074",
        "sr-Latn-CS",
        "Serbian (Latin) - Serbia and Montenegro",
    ),
    ("081D", "2077", "sv-FI", "Swedish - Finland"),
    ("0820", "2080", "ur-IN", "Urdu - (reserved)"),
    (
        "082C",
        "2092",
        "az-Cyrl-AZ",
        "Azeri (Cyrillic) - Azerbaijan",
    ),
    ("082E", "2094", "dsb-DE", "Lower Sorbian - Germany"),
    ("0832", "2098", "tn-BW", "Setswana / Tswana - Botswana"),
    ("083B", "2107", "se-SE", "Sami (Northern) - Sweden"),
    ("083C", "2108", "ga-IE", "Irish - Ireland"),
    ("083E", "2110", "ms-BN", "Malay - Brunei Darassalam"),
    (
        "0843",
        "2115",
        "uz-Cyrl-UZ",
        "Uzbek (Cyrillic) - Uzbekistan",
    ),
    ("0845", "2117", "bn-BD", "Bangla - Bangladesh"),
    ("0846", "2118", "pa-Arab-PK", "Punjabi - Pakistan"),
    ("0849", "2121", "ta-LK", "Tamil - Sri Lanka"),
    ("0850", "2128", "mn-Mong-CN", "Mongolian (Mong) - Mongolia"),
    ("0859", "2137", "sd-Arab-PK", "Sindhi - Pakistan"),
    ("085D", "2141", "iu-Latn-CA", "Inuktitut (Latin) - Canada"),
    ("085F", "2143", "tzm-Latn-DZ", "Tamazight (Latin) - Algeria"),
    ("0867", "2151", "ff-Latn-SN", "Pular - Senegal"),
    ("086B", "2155", "quz-EC", "Quechua - Ecuador"),
    ("0873", "2163", "ti-ER", "(reserved) - (reserved)"),
    ("0873", "2163", "ti-ER", "Tigrinya - Eritrea"),
    ("0C01", "3073", "ar-EG", "Arabic - Egypt"),
    ("0C04", "3076", "zh-HK", "Chinese - Hong Kong SAR"),
    ("0C07", "3079", "de-AT", "German - Austria"),
    ("0C09", "3081", "en-AU", "English - Australia"),
    ("0C0A", "3082", "es-ES", "Spanish - Spain"),
    ("0C0C", "3084", "fr-CA", "French - Canada"),
    (
        "0C1A",
        "3098",
        "sr-Cyrl-CS",
        "Serbian (Cyrillic) - Serbia and Montenegro",
    ),
    ("0C3B", "3131", "se-FI", "Sami (Northern) - Finland"),
    ("0C6B", "3179", "quz-PE", "Quechua - Peru"),
    ("1001", "4097", "ar-LY", "Arabic - Libya"),
    ("1004", "4100", "zh-SG", "Chinese - Singapore"),
    ("1007", "4103", "de-LU", "German - Luxembourg"),
    ("1009", "4105", "en-CA", "English - Canada"),
    ("100A", "4106", "es-GT", "Spanish - Guatemala"),
    ("100C", "4108", "fr-CH", "French - Switzerland"),
    (
        "101A",
        "4122",
        "hr-BA",
        "Croatian (Latin) - Bosnia and Herzegovina",
    ),
    ("103B", "4155", "smj-NO", "Sami (Lule) - Norway"),
    (
        "105F",
        "4191",
        "tzm-Tfng-MA",
        "Central Atlas Tamazight (Tifinagh) - Morocco",
    ),
    ("1401", "5121", "ar-DZ", "Arabic - Algeria"),
    ("1404", "5124", "zh-MO", "Chinese - Macao SAR"),
    ("1407", "5127", "de-LI", "German - Liechtenstein"),
    ("1409", "5129", "en-NZ", "English - New Zealand"),
    ("140A", "5130", "es-CR", "Spanish - Costa Rica"),
    ("140C", "5132", "fr-LU", "French - Luxembourg"),
    (
        "141A",
        "5146",
        "bs-Latn-BA",
        "Bosnian (Latin) - Bosnia and Herzegovina",
    ),
    ("143B", "5179", "smj-SE", "Sami (Lule) - Sweden"),
    ("1801", "6145", "ar-MA", "Arabic - Morocco"),
    ("1809", "6153", "en-IE", "English - Ireland"),
    ("180A", "6154", "es-PA", "Spanish - Panama"),
    ("180C", "6156", "fr-MC", "French - Monaco"),
    (
        "181A",
        "6170",
        "sr-Latn-BA",
        "Serbian (Latin) - Bosnia and Herzegovina",
    ),
    ("183B", "6203", "sma-NO", "Sami (Southern) - Norway"),
    ("1C01", "7169", "ar-TN", "Arabic - Tunisia"),
    ("1c09", "7177", "en-ZA", "English - South Africa"),
    ("1C0A", "7178", "es-DO", "Spanish - Dominican Republic"),
    (
        "1C1A",
        "7194",
        "sr-Cyrl-BA",
        "Serbian (Cyrillic) - Bosnia and Herzegovina",
    ),
    ("1C3B", "7227", "sma-SE", "Sami (Southern) - Sweden"),
    ("2001", "8193", "ar-OM", "Arabic - Oman"),
    ("2009", "8201", "en-JM", "English - Jamaica"),
    ("200A", "8202", "es-VE", "Spanish - Venezuela"),
    (
        "201A",
        "8218",
        "bs-Cyrl-BA",
        "Bosnian (Cyrillic) - Bosnia and Herzegovina",
    ),
    ("203B", "8251", "sms-FI", "Sami (Skolt) - Finland"),
    ("2401", "9217", "ar-YE", "Arabic - Yemen"),
    ("2409", "9225", "en-029", "English - Caribbean"),
    ("240A", "9226", "es-CO", "Spanish - Colombia"),
    ("241A", "9242", "sr-Latn-RS", "Serbian (Latin) - Serbia"),
    ("243B", "9275", "smn-FI", "Sami (Inari) - Finland"),
    ("2801", "10241", "ar-SY", "Arabic - Syria"),
    ("2809", "10249", "en-BZ", "English - Belize"),
    ("280A", "10250", "es-PE", "Spanish - Peru"),
    ("281A", "10266", "sr-Cyrl-RS", "Serbian (Cyrillic) - Serbia"),
    ("2C01", "11265", "ar-JO", "Arabic - Jordan"),
    ("2C09", "11273", "en-TT", "English - Trinidad and Tobago"),
    ("2C0A", "11274", "es-AR", "Spanish - Argentina"),
    (
        "2C1A",
        "11290",
        "sr-Latn-ME",
        "Serbian (Latin) - Montenegro",
    ),
    ("3001", "12289", "ar-LB", "Arabic - Lebanon"),
    ("3009", "12297", "en-ZW", "English - Zimbabwe"),
    ("300A", "12298", "es-EC", "Spanish - Ecuador"),
    (
        "301A",
        "12314",
        "sr-Cyrl-ME",
        "Serbian (Cyrillic) - Montenegro",
    ),
    ("3401", "13313", "ar-KW", "Arabic - Kuwait"),
    ("3409", "13321", "en-PH", "English - Philippines"),
    ("340A", "13322", "es-CL", "Spanish - Chile"),
    ("3801", "14337", "ar-AE", "Arabic - U.A.E."),
    ("380A", "14346", "es-UY", "Spanish - Uruguay"),
    ("3C01", "15361", "ar-BH", "Arabic - Bahrain"),
    ("3C0A", "15370", "es-PY", "Spanish - Paraguay"),
    ("4001", "16385", "ar-QA", "Arabic - Qatar"),
    ("4009", "16393", "en-IN", "English - India"),
    ("400A", "16394", "es-BO", "Spanish - Bolivia"),
    ("4409", "17417", "en-MY", "English - Malaysia"),
    ("440A", "17418", "es-SV", "Spanish - El Salvador"),
    ("4809", "18441", "en-SG", "English - Singapore"),
    ("480A", "18442", "es-HN", "Spanish - Honduras"),
    ("4C0A", "19466", "es-NI", "Spanish - Nicaragua"),
    ("500A", "20490", "es-PR", "Spanish - Puerto Rico"),
    ("540A", "21514", "es-US", "Spanish - United States"),
    ("7C04", "31748", "zh-CHT", "Chinese - Traditional"),
];

pub fn detect_locale(name: &str) -> Option<String> {
    LOCALE_MAPPINGS.iter().find_map(|i| {
        if i.0 == name {
            Some(i.2.to_string())
        } else {
            None
        }
    })
}

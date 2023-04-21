use std::{env};

enum KanaToken {
    // For simplicity, we can include anything non-kana
    NonKana(u8),
    Kana(Kana),
}

enum Kana {
    // Base
    A,  I,   U,   E,  O,
    Ka, Ki,  Ku,  Ke, Ko, 
    Sa, Shi, Su,  Se, So,       // Variant for sokuon
    Ta, Chi, Tsu, Te, To,       LittleTsu,
    Na, Ni,  Nu,  Ne, No, 
    Ha, Hi,  Fu,  He, Ho, 
    Ma, Mi,  Mu,  Me, Mo, 
    Ya,      Yu,      Yo, 
    Ra, Ri,  Ru,  Re, Ro, 
    Wa,               Wo,
    N,

    // Dakuon
    Ga, Gi,  Gu,  Ge, Go,
    Za, Ji,  Zu,  Ze, Zo,
    Da,           De, Do,
    Ba, Bi,  Bu,  Be, Bo,
    Pa, Pi,  Pu,  Pe, Po,

    // Yoon (combo)
    Kya,     Kyu,     Kyo,
    Gya,     Gyu,     Gyo,
    Sha,     Shu,     Sho,
    Ja,      Ju,      Jo,
    Cha,     Chu,     Cho,
    Nya,     Nyu,     Nyo,
    Hya,     Hyu,     Hyo,
    Bya,     Byu,     Byo,
    Pya,     Pyu,     Pyo,
    Mya,     Myu,     Myo,
    Rya,     Ryu,     Ryo,

    // Additionnal combo (for loanwords)

}

impl Kana {
    pub fn to_hiragana(&self) -> &str {
        match &self {
            Kana::A => "あ",
            Kana::I => "い",
            Kana::U => "う",
            Kana::E => "え",
            Kana::O => "お",
            Kana::Ka => "か",
            Kana::Ki => "き",
            Kana::Ku => "く",
            Kana::Ke => "け",
            Kana::Ko => "こ",
            Kana::Sa => "さ",
            Kana::Shi => "し",
            Kana::Su => "す",
            Kana::Se => "せ",
            Kana::So => "そ",
            Kana::Ta => "た",
            Kana::Chi => "ち",
            Kana::Tsu => "つ",
            Kana::Te => "て",
            Kana::To => "と",
            Kana::Na => "な",
            Kana::Ni => "に",
            Kana::Nu => "ぬ",
            Kana::Ne => "ね",
            Kana::No => "の",
            Kana::Ha => "は",
            Kana::Hi => "ひ",
            Kana::Fu => "ふ",
            Kana::He => "へ",
            Kana::Ho => "ほ",
            Kana::Ma => "ま",
            Kana::Mi => "み",
            Kana::Mu => "む",
            Kana::Me => "め",
            Kana::Mo => "も",
            Kana::Ya => "や",
            Kana::Yu => "ゆ",
            Kana::Yo => "よ",
            Kana::Ra => "ら",
            Kana::Ri => "り",
            Kana::Ru => "る",
            Kana::Re => "れ",
            Kana::Ro => "ろ",
            Kana::Wa => "わ",
            Kana::Wo => "を",
            Kana::N => "ん",
            Kana::Ga => "が",
            Kana::Gi => "ぎ",
            Kana::Gu => "ぐ",
            Kana::Ge => "げ",
            Kana::Go => "ご",
            Kana::Za => "ざ",
            Kana::Ji => "じ",
            Kana::Zu => "ず",
            Kana::Ze => "ぜ",
            Kana::Zo => "ぞ",
            Kana::Da => "だ",
            Kana::De => "で",
            Kana::Do => "ど",
            Kana::Ba => "ば",
            Kana::Bi => "び",
            Kana::Bu => "ぶ",
            Kana::Be => "べ",
            Kana::Bo => "ぼ",
            Kana::Pa => "ぱ",
            Kana::Pi => "ぴ",
            Kana::Pu => "ぷ",
            Kana::Pe => "ぺ",
            Kana::Po => "ぽ",
            Kana::Kya => "きゃ",
            Kana::Kyu => "きゅ",
            Kana::Kyo => "きょ",
            Kana::Gya => "ぎゃ",
            Kana::Gyu => "ぎゅ",
            Kana::Gyo => "ぎょ",
            Kana::Sha => "しゃ",
            Kana::Shu => "しゅ",
            Kana::Sho => "しょ",
            Kana::Ja => "じゃ",
            Kana::Ju => "じゅ",
            Kana::Jo => "じょ",
            Kana::Cha => "ちゃ",
            Kana::Chu => "ちゅ",
            Kana::Cho => "ちょ",
            Kana::Nya => "にゃ",
            Kana::Nyu => "にゅ",
            Kana::Nyo => "にょ",
            Kana::Hya => "ひゃ",
            Kana::Hyu => "ひゅ",
            Kana::Hyo => "ひょ",
            Kana::Bya => "びゃ",
            Kana::Byu => "びゅ",
            Kana::Byo => "びょ",
            Kana::Pya => "ぴゃ",
            Kana::Pyu => "ぴゅ",
            Kana::Pyo => "ぴょ",
            Kana::Mya => "みゃ",
            Kana::Myu => "みゅ",
            Kana::Myo => "みょ",
            Kana::Rya => "りゃ",
            Kana::Ryu => "りゅ",
            Kana::Ryo => "りょ",
            Kana::LittleTsu => "っ",
        }
    }

    pub fn to_katakana(&self) -> &str {
        match &self {
            Kana::A => "ア",
            Kana::I => "イ",
            Kana::U => "ウ",
            Kana::E => "エ",
            Kana::O => "オ",
            Kana::Ka => "カ",
            Kana::Ki => "キ",
            Kana::Ku => "ク",
            Kana::Ke => "ケ",
            Kana::Ko => "コ",
            Kana::Sa => "サ",
            Kana::Shi => "シ",
            Kana::Su => "ス",
            Kana::Se => "セ",
            Kana::So => "ソ",
            Kana::Ta => "タ",
            Kana::Chi => "チ",
            Kana::Tsu => "ツ",
            Kana::Te => "テ",
            Kana::To => "ト",
            Kana::Na => "ナ",
            Kana::Ni => "ニ",
            Kana::Nu => "ヌ",
            Kana::Ne => "ネ",
            Kana::No => "ノ",
            Kana::Ha => "ハ",
            Kana::Hi => "ヒ",
            Kana::Fu => "フ",
            Kana::He => "ヘ",
            Kana::Ho => "ホ",
            Kana::Ma => "マ",
            Kana::Mi => "ミ",
            Kana::Mu => "ム",
            Kana::Me => "メ",
            Kana::Mo => "モ",
            Kana::Ya => "ヤ",
            Kana::Yu => "ユ",
            Kana::Yo => "ヨ",
            Kana::Ra => "ラ",
            Kana::Ri => "リ",
            Kana::Ru => "ル",
            Kana::Re => "レ",
            Kana::Ro => "ロ",
            Kana::Wa => "ワ",
            Kana::Wo => "ヲ",
            Kana::N => "ン",
            Kana::Ga => "ガ",
            Kana::Gi => "ギ",
            Kana::Gu => "グ",
            Kana::Ge => "ゲ",
            Kana::Go => "ゴ",
            Kana::Za => "ザ",
            Kana::Ji => "ジ",
            Kana::Zu => "ズ",
            Kana::Ze => "ゼ",
            Kana::Zo => "ゾ",
            Kana::Da => "ダ",
            Kana::De => "デ",
            Kana::Do => "ド",
            Kana::Ba => "バ",
            Kana::Bi => "ビ",
            Kana::Bu => "ブ",
            Kana::Be => "ベ",
            Kana::Bo => "ボ",
            Kana::Pa => "パ",
            Kana::Pi => "ピ",
            Kana::Pu => "プ",
            Kana::Pe => "ペ",
            Kana::Po => "ポ",
            Kana::Kya => "キャ",
            Kana::Kyu => "キュ",
            Kana::Kyo => "キョ",
            Kana::Gya => "ギャ",
            Kana::Gyu => "ギュ",
            Kana::Gyo => "ギョ",
            Kana::Sha => "シャ",
            Kana::Shu => "シュ",
            Kana::Sho => "ショ",
            Kana::Ja => "ジャ",
            Kana::Ju => "ジュ",
            Kana::Jo => "ジョ",
            Kana::Cha => "チャ",
            Kana::Chu => "チュ",
            Kana::Cho => "チョ",
            Kana::Nya => "ニャ",
            Kana::Nyu => "ニュ",
            Kana::Nyo => "ニョ",
            Kana::Hya => "ヒャ",
            Kana::Hyu => "ヒュ",
            Kana::Hyo => "ヒョ",
            Kana::Bya => "ビャ",
            Kana::Byu => "ビュ",
            Kana::Byo => "ビョ",
            Kana::Pya => "ピャ",
            Kana::Pyu => "ピュ",
            Kana::Pyo => "ピョ",
            Kana::Mya => "ミャ",
            Kana::Myu => "ミュ",
            Kana::Myo => "ミョ",
            Kana::Rya => "リャ",
            Kana::Ryu => "リュ",
            Kana::Ryo => "リョ",
            Kana::LittleTsu => "ッ",
        }
    }
}

pub struct KanaSequence(Vec<KanaToken>);

impl KanaSequence {
    pub fn to_hiragana(&self) -> String {
        let mut output = String::new();

        let mut non_kana_bytes = vec![];

        for token in &self.0 {
            match token {
                KanaToken::Kana(x) => {
                    // try to push the accumulated data first, as an utf-8 string
                    if !non_kana_bytes.is_empty() {
                        if let Ok(string) = std::str::from_utf8(non_kana_bytes.as_slice()) {
                            output.push_str(string);
                        }

                        non_kana_bytes.clear();
                    }

                    output.push_str(x.to_hiragana());
                },
                KanaToken::NonKana(x) => {
                    non_kana_bytes.push(*x);
                }
            }
        }

        if !non_kana_bytes.is_empty() {
            if let Ok(string) = std::str::from_utf8(non_kana_bytes.as_slice()) {
                output.push_str(string);
            }
        }

        output
    }
    
    pub fn to_katakana(&self) -> String {
        let mut output = String::new();

        let mut non_kana_bytes = vec![];

        for token in &self.0 {
            match token {
                KanaToken::Kana(x) => {
                    // try to push the accumulated data first, as an utf-8 string
                    if !non_kana_bytes.is_empty() {
                        if let Ok(string) = std::str::from_utf8(non_kana_bytes.as_slice()) {
                            output.push_str(string);
                        }

                        non_kana_bytes.clear();
                    }

                    output.push_str(x.to_katakana());
                },
                KanaToken::NonKana(x) => {
                    non_kana_bytes.push(*x);
                }
            }
        }

        if !non_kana_bytes.is_empty() {
            if let Ok(string) = std::str::from_utf8(non_kana_bytes.as_slice()) {
                output.push_str(string);
            }
        }

        output
    }

    /// Encode a vector of kana tokens from a Modified-Hepburn romaji sequence (https://en.wikipedia.org/wiki/Hepburn_romanization#Variants)
    pub fn from_hepburn(hepburn_sequence: &str) -> Self {
        let mut tokens: Vec<KanaToken> = vec![];

        let chars = hepburn_sequence.chars();
        let mut iter = chars.into_iter();

        // Will accumulate characters until they can form a complete kana or non-kana value
        let mut accumulator = String::with_capacity(4);

        loop {
            match iter.next() {
                Some(ch) => {
                    enum KanaScanState {
                        /// We have valid candidate-characters for a kana, but still incomplete
                        MaybeKana,
                        /// We only have a non-kana value
                        NonKana,
                        /// We have a non-kana value followed by a kana candidate
                        NonKanaThenMaybeKana,
                        /// We have a valid kana
                        IsKana(Kana),
                        IsSokuon(Kana),
                        /// We have a valid kana followed by a kana candidate
                        IsKanaThenMaybeKana(Kana),
                        /// We have a non-kana value followed by a valid kana
                        NonKanaThenKana(Kana),
                    }

                    let kana: KanaScanState = match (accumulator.as_str(), ch) {
                        (x, 'a' | 'u' | 'i' | 'e' | 'o') if x.is_empty() => {
                            // returning NonKanaThenKana after each (.., <vowel>) without checking accumulator emptiness would be also fine, but it would incur a useless string copy
                            match ch {
                                'a' => KanaScanState::IsKana(Kana::A),
                                'i' => KanaScanState::IsKana(Kana::I),
                                'u' => KanaScanState::IsKana(Kana::U),
                                'e' => KanaScanState::IsKana(Kana::E),
                                'o' => KanaScanState::IsKana(Kana::O),
                                _ => unreachable!()
                            }
                        },

                        ("k", 'a') => KanaScanState::IsKana(Kana::Ka),
                        ("kk", 'a') => KanaScanState::IsSokuon(Kana::Ka),
                        ("s", 'a') => KanaScanState::IsKana(Kana::Sa),
                        ("ss", 'a') => KanaScanState::IsSokuon(Kana::Sa),
                        ("t", 'a') => KanaScanState::IsKana(Kana::Ta),
                        ("tt", 'a') => KanaScanState::IsSokuon(Kana::Ta),
                        ("n", 'a') => KanaScanState::IsKana(Kana::Na),
                        ("h", 'a') => KanaScanState::IsKana(Kana::Ha),
                        ("m", 'a') => KanaScanState::IsKana(Kana::Ma),
                        ("y", 'a') => KanaScanState::IsKana(Kana::Ya),
                        ("r", 'a') => KanaScanState::IsKana(Kana::Ra),
                        ("w", 'a') => KanaScanState::IsKana(Kana::Wa),
                        ("g", 'a') => KanaScanState::IsKana(Kana::Ga),
                        ("gg", 'a') => KanaScanState::IsSokuon(Kana::Ga),
                        ("z", 'a') => KanaScanState::IsKana(Kana::Za),
                        ("d", 'a') => KanaScanState::IsKana(Kana::Da),
                        ("dd", 'a') => KanaScanState::IsSokuon(Kana::Da),
                        ("b", 'a') => KanaScanState::IsKana(Kana::Ba),
                        ("bb", 'a') => KanaScanState::IsSokuon(Kana::Ba),
                        ("p", 'a') => KanaScanState::IsKana(Kana::Pa),
                        ("pp", 'a') => KanaScanState::IsSokuon(Kana::Pa),
                        ("ky", 'a') => KanaScanState::IsKana(Kana::Kya),
                        ("gy", 'a') => KanaScanState::IsKana(Kana::Gya),
                        ("sh", 'a') => KanaScanState::IsKana(Kana::Sha),
                        ("j", 'a') => KanaScanState::IsKana(Kana::Ja),
                        ("ch", 'a') => KanaScanState::IsKana(Kana::Cha),
                        ("ny", 'a') => KanaScanState::IsKana(Kana::Nya),
                        ("hy", 'a') => KanaScanState::IsKana(Kana::Hya),
                        ("by", 'a') => KanaScanState::IsKana(Kana::Bya),
                        ("py", 'a') => KanaScanState::IsKana(Kana::Pya),
                        ("my", 'a') => KanaScanState::IsKana(Kana::Mya),
                        ("ry", 'a') => KanaScanState::IsKana(Kana::Rya),
                        (.., 'a') => KanaScanState::NonKanaThenKana(Kana::A),

                        ("k", 'i') => KanaScanState::IsKana(Kana::Ki),
                        ("kk", 'i') => KanaScanState::IsSokuon(Kana::Ki),
                        ("sh", 'i') => KanaScanState::IsKana(Kana::Shi),
                        ("ssh", 'i') => KanaScanState::IsSokuon(Kana::Shi),
                        ("ch", 'i') => KanaScanState::IsKana(Kana::Chi),
                        ("tch", 'i') => KanaScanState::IsSokuon(Kana::Chi),
                        ("n", 'i') => KanaScanState::IsKana(Kana::Ni),
                        ("h", 'i') => KanaScanState::IsKana(Kana::Hi),
                        ("m", 'i') => KanaScanState::IsKana(Kana::Mi),
                        ("r", 'i') => KanaScanState::IsKana(Kana::Ri),
                        ("g", 'i') => KanaScanState::IsKana(Kana::Gi),
                        ("j", 'i') => KanaScanState::IsKana(Kana::Ji),
                        ("b", 'i') => KanaScanState::IsKana(Kana::Bi),
                        ("bb", 'i') => KanaScanState::IsSokuon(Kana::Bi),
                        ("p", 'i') => KanaScanState::IsKana(Kana::Pi),
                        ("pp", 'i') => KanaScanState::IsSokuon(Kana::Pi),
                        (.., 'i') => KanaScanState::NonKanaThenKana(Kana::I),

                        ("k", 'u') => KanaScanState::IsKana(Kana::Ku),
                        ("kk", 'u') => KanaScanState::IsSokuon(Kana::Ku),
                        ("s", 'u') => KanaScanState::IsKana(Kana::Su),
                        ("ss", 'u') => KanaScanState::IsSokuon(Kana::Su),
                        ("ts", 'u') => KanaScanState::IsKana(Kana::Tsu),
                        ("tts", 'u') => KanaScanState::IsSokuon(Kana::Tsu),
                        ("n", 'u') => KanaScanState::IsKana(Kana::Nu),
                        ("f", 'u') => KanaScanState::IsKana(Kana::Fu),
                        ("m", 'u') => KanaScanState::IsKana(Kana::Mu),
                        ("y", 'u') => KanaScanState::IsKana(Kana::Yu),
                        ("r", 'u') => KanaScanState::IsKana(Kana::Ru),
                        ("g", 'u') => KanaScanState::IsKana(Kana::Gu),
                        ("gg", 'u') => KanaScanState::IsSokuon(Kana::Gu),
                        ("z", 'u') => KanaScanState::IsKana(Kana::Zu),
                        ("b", 'u') => KanaScanState::IsKana(Kana::Bu),
                        ("bb", 'u') => KanaScanState::IsSokuon(Kana::Bu),
                        ("p", 'u') => KanaScanState::IsKana(Kana::Pu),
                        ("pp", 'u') => KanaScanState::IsSokuon(Kana::Pu),
                        ("ky", 'u') => KanaScanState::IsKana(Kana::Kyu),
                        ("gy", 'u') => KanaScanState::IsKana(Kana::Gyu),
                        ("sh", 'u') => KanaScanState::IsKana(Kana::Shu),
                        ("j", 'u') => KanaScanState::IsKana(Kana::Ju),
                        ("ch", 'u') => KanaScanState::IsKana(Kana::Chu),
                        ("ny", 'u') => KanaScanState::IsKana(Kana::Nyu),
                        ("hy", 'u') => KanaScanState::IsKana(Kana::Hyu),
                        ("by", 'u') => KanaScanState::IsKana(Kana::Byu),
                        ("py", 'u') => KanaScanState::IsKana(Kana::Pyu),
                        ("my", 'u') => KanaScanState::IsKana(Kana::Myu),
                        ("ry", 'u') => KanaScanState::IsKana(Kana::Ryu),
                        (.., 'u') => KanaScanState::NonKanaThenKana(Kana::U),

                        ("k", 'e') => KanaScanState::IsKana(Kana::Ke),
                        ("kk", 'e') => KanaScanState::IsSokuon(Kana::Ke),
                        ("s", 'e') => KanaScanState::IsKana(Kana::Se),
                        ("ss", 'e') => KanaScanState::IsSokuon(Kana::Se),
                        ("t", 'e') => KanaScanState::IsKana(Kana::Te),
                        ("tt", 'e') => KanaScanState::IsSokuon(Kana::Te),
                        ("n", 'e') => KanaScanState::IsKana(Kana::Ne),
                        ("h", 'e') => KanaScanState::IsKana(Kana::He),
                        ("m", 'e') => KanaScanState::IsKana(Kana::Me),
                        ("r", 'e') => KanaScanState::IsKana(Kana::Re),
                        ("g", 'e') => KanaScanState::IsKana(Kana::Ge),
                        ("gg", 'e') => KanaScanState::IsSokuon(Kana::Ge),
                        ("z", 'e') => KanaScanState::IsKana(Kana::Ze),
                        ("d", 'e') => KanaScanState::IsKana(Kana::De),
                        ("dd", 'e') => KanaScanState::IsSokuon(Kana::De),
                        ("b", 'e') => KanaScanState::IsKana(Kana::Be),
                        ("bb", 'e') => KanaScanState::IsSokuon(Kana::Be),
                        ("p", 'e') => KanaScanState::IsKana(Kana::Pe),
                        ("pp", 'e') => KanaScanState::IsSokuon(Kana::Pe),
                        (.., 'e') => KanaScanState::NonKanaThenKana(Kana::E),

                        ("k", 'o') => KanaScanState::IsKana(Kana::Ko),
                        ("kk", 'o') => KanaScanState::IsSokuon(Kana::Ko),
                        ("s", 'o') => KanaScanState::IsKana(Kana::So),
                        ("ss", 'o') => KanaScanState::IsSokuon(Kana::So),
                        ("t", 'o') => KanaScanState::IsKana(Kana::To),
                        ("tt", 'o') => KanaScanState::IsSokuon(Kana::To),
                        ("n", 'o') => KanaScanState::IsKana(Kana::No),
                        ("h", 'o') => KanaScanState::IsKana(Kana::Ho),
                        ("m", 'o') => KanaScanState::IsKana(Kana::Mo),
                        ("y", 'o') => KanaScanState::IsKana(Kana::Yo),
                        ("r", 'o') => KanaScanState::IsKana(Kana::Ro),
                        ("w", 'o') => KanaScanState::IsKana(Kana::Wo),
                        ("g", 'o') => KanaScanState::IsKana(Kana::Go),
                        ("gg", 'o') => KanaScanState::IsSokuon(Kana::Go),
                        ("z", 'o') => KanaScanState::IsKana(Kana::Zo),
                        ("d", 'o') => KanaScanState::IsKana(Kana::Do),
                        ("dd", 'o') => KanaScanState::IsSokuon(Kana::Do),
                        ("b", 'o') => KanaScanState::IsKana(Kana::Bo),
                        ("bb", 'o') => KanaScanState::IsSokuon(Kana::Bo),
                        ("p", 'o') => KanaScanState::IsKana(Kana::Po),
                        ("pp", 'o') => KanaScanState::IsSokuon(Kana::Po),
                        ("ky", 'o') => KanaScanState::IsKana(Kana::Kyo),
                        ("gy", 'o') => KanaScanState::IsKana(Kana::Gyo),
                        ("sh", 'o') => KanaScanState::IsKana(Kana::Sho),
                        ("j", 'o') => KanaScanState::IsKana(Kana::Jo),
                        ("ch", 'o') => KanaScanState::IsKana(Kana::Cho),
                        ("ny", 'o') => KanaScanState::IsKana(Kana::Nyo),
                        ("hy", 'o') => KanaScanState::IsKana(Kana::Hyo),
                        ("by", 'o') => KanaScanState::IsKana(Kana::Byo),
                        ("py", 'o') => KanaScanState::IsKana(Kana::Pyo),
                        ("my", 'o') => KanaScanState::IsKana(Kana::Myo),
                        ("ry", 'o') => KanaScanState::IsKana(Kana::Ryo),
                        (.., 'o') => KanaScanState::NonKanaThenKana(Kana::O),

                        (x, 'k' | 's' | 't' | 'p' | 'c' | 'n' | 'h' | 'f' | 'm' | 'y' | 'r' | 'w' | 'g' | 'z' | 'd' | 'b' | 'j') => {
                            if x.is_empty() {
                                KanaScanState::MaybeKana
                            } else {
                                match (x, ch) {
                                    // for chi, shi, and yoon derived from those (cha, sha, etc.)
                                    ("s", 'h') => KanaScanState::MaybeKana,
                                    ("c", 'h') => KanaScanState::MaybeKana,

                                    // for tsu and derived (tsa, tse, etc.)
                                    ("t", 's') => KanaScanState::MaybeKana,

                                    // for sokuon
                                    ("k", 'k') => KanaScanState::MaybeKana,
                                    ("s", 's') => KanaScanState::MaybeKana,
                                    ("ss", 'h') => KanaScanState::MaybeKana,
                                    ("t", 't') => KanaScanState::MaybeKana,
                                    ("t", 'c') => KanaScanState::MaybeKana, // needed for the following line (tc+h) to be handled, otherwise the 't' would be discarded early
                                    ("tc", 'h') => KanaScanState::MaybeKana,
                                    ("p", 'p') => KanaScanState::MaybeKana,
                                    ("g", 'g') => KanaScanState::MaybeKana,
                                    ("d", 'd') => KanaScanState::MaybeKana,
                                    ("b", 'b') => KanaScanState::MaybeKana,

                                    // for other yoon
                                    // TODO: add the extended set (for loanwords)
                                    ("k", 'y') => KanaScanState::MaybeKana,
                                    ("g", 'y') => KanaScanState::MaybeKana,
                                    ("n", 'y') => KanaScanState::MaybeKana,
                                    ("h", 'y') => KanaScanState::MaybeKana,
                                    ("b", 'y') => KanaScanState::MaybeKana,
                                    ("p", 'y') => KanaScanState::MaybeKana,
                                    ("m", 'y') => KanaScanState::MaybeKana,
                                    ("r", 'y') => KanaScanState::MaybeKana,

                                    //("ts", 'y') => KanaScanState::MaybeKana, // for tsyu, but it adds problems: 'tsya' leads to {tsy(NonKana)}{a} instead of {ts(NonKana)}{ya}, would need to split the accumulator instead of considering as one kana or non-kana

                                    ("n", ..) => KanaScanState::IsKanaThenMaybeKana(Kana::N),
                                    _ => KanaScanState::NonKanaThenMaybeKana,
                                }
                            }
                        }
                        _ => {
                            KanaScanState::NonKana
                        }
                    };

                    match kana {
                        // We have a valid kana, we can push it and clear the accumulator
                        KanaScanState::IsKana(x) => {
                            tokens.push(KanaToken::Kana(x));
                            accumulator.clear();
                        },
                        // We have a valid sokuon, we can push it and clear the accumulator
                        KanaScanState::IsSokuon(x) => {
                            tokens.push(KanaToken::Kana(Kana::LittleTsu));
                            tokens.push(KanaToken::Kana(x));
                            accumulator.clear();
                        },
                        // We have a valid kana, and the new character may also make up a valid kana, so we push and clear the accumulator, then add the new character to it
                        KanaScanState::IsKanaThenMaybeKana(x) => {
                            tokens.push(KanaToken::Kana(x));
                            accumulator.clear();
                            accumulator.push(ch);
                        }
                        // The new character won't make part of valid kana at all, so we push it together with the accumulator, and clear the latter
                        KanaScanState::NonKana => {
                            accumulator.push(ch);
                            accumulator.as_bytes().iter().for_each(|x| tokens.push(KanaToken::NonKana(*x)));
                            accumulator.clear();
                        },
                        // The accumulator won't make up any valid kana, but the new character may do, so we push and clear the accumulator, then add this character to it
                        KanaScanState::NonKanaThenMaybeKana => {
                            accumulator.as_bytes().iter().for_each(|x| tokens.push(KanaToken::NonKana(*x)));
                            accumulator.clear();
                            accumulator.push(ch);
                        },
                        // The accumulator may form a valid kana, we add the new character to it
                        KanaScanState::MaybeKana => {
                            accumulator.push(ch);
                        },
                        // The accumulator cannot form a valid kana but we have an already-valid kana after it, thus we push both values and clear the accumulator
                        KanaScanState::NonKanaThenKana(x) => {
                            accumulator.as_bytes().iter().for_each(|x| tokens.push(KanaToken::NonKana(*x)));
                            tokens.push(KanaToken::Kana(x));
                            accumulator.clear();
                        }
                    }
                },
                None => {
                    // handle remaining character(s), if any
                    if !accumulator.is_empty() {
                        match accumulator.as_str() {
                            "n" => tokens.push(KanaToken::Kana(Kana::N)),
                            x => {
                                x.as_bytes().iter().for_each(|x| tokens.push(KanaToken::NonKana(*x)));
                            }
                        }
                    }

                    break
                }
            }
        }

        KanaSequence(tokens)
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let iteration_count = args.nth(0).unwrap_or(String::from("1")).parse().unwrap_or(1);

    let romaji = args.collect::<Vec<String>>().join(" ");
    let mut kanas = Option::<KanaSequence>::None;

    println!("iteration count: {}", iteration_count);

    let start = std::time::Instant::now();

    for _ in 0..iteration_count {
        kanas = Some(KanaSequence::from_hepburn(&romaji));
    }

    println!("{} ms", start.elapsed().as_nanos() as f32 / 1000000f32);

    if let Some(x) = kanas {
        println!("{}", x.to_hiragana());
        println!("{}", x.to_katakana());
    }

    println!("Byte array representation of romaji input: {:02X?}", romaji.as_bytes());
}
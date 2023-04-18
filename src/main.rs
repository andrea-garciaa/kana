use std::{env};

enum KanaToken {
    // For simplicity, we can include anything non-kana
    NonKana(String),

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

impl KanaToken {
    pub fn to_hiragana(&self) -> &str {
        match &self {
            KanaToken::NonKana(x) => x,
            KanaToken::A => "あ",
            KanaToken::I => "い",
            KanaToken::U => "う",
            KanaToken::E => "え",
            KanaToken::O => "お",
            KanaToken::Ka => "か",
            KanaToken::Ki => "き",
            KanaToken::Ku => "く",
            KanaToken::Ke => "け",
            KanaToken::Ko => "こ",
            KanaToken::Sa => "さ",
            KanaToken::Shi => "し",
            KanaToken::Su => "す",
            KanaToken::Se => "せ",
            KanaToken::So => "そ",
            KanaToken::Ta => "た",
            KanaToken::Chi => "ち",
            KanaToken::Tsu => "つ",
            KanaToken::Te => "て",
            KanaToken::To => "と",
            KanaToken::Na => "な",
            KanaToken::Ni => "に",
            KanaToken::Nu => "ぬ",
            KanaToken::Ne => "ね",
            KanaToken::No => "の",
            KanaToken::Ha => "は",
            KanaToken::Hi => "ひ",
            KanaToken::Fu => "ふ",
            KanaToken::He => "へ",
            KanaToken::Ho => "ほ",
            KanaToken::Ma => "ま",
            KanaToken::Mi => "み",
            KanaToken::Mu => "む",
            KanaToken::Me => "め",
            KanaToken::Mo => "も",
            KanaToken::Ya => "や",
            KanaToken::Yu => "ゆ",
            KanaToken::Yo => "よ",
            KanaToken::Ra => "ら",
            KanaToken::Ri => "り",
            KanaToken::Ru => "る",
            KanaToken::Re => "れ",
            KanaToken::Ro => "ろ",
            KanaToken::Wa => "わ",
            KanaToken::Wo => "を",
            KanaToken::N => "ん",
            KanaToken::Ga => "が",
            KanaToken::Gi => "ぎ",
            KanaToken::Gu => "ぐ",
            KanaToken::Ge => "げ",
            KanaToken::Go => "ご",
            KanaToken::Za => "ざ",
            KanaToken::Ji => "じ",
            KanaToken::Zu => "ず",
            KanaToken::Ze => "ぜ",
            KanaToken::Zo => "ぞ",
            KanaToken::Da => "だ",
            KanaToken::De => "で",
            KanaToken::Do => "ど",
            KanaToken::Ba => "ば",
            KanaToken::Bi => "び",
            KanaToken::Bu => "ぶ",
            KanaToken::Be => "べ",
            KanaToken::Bo => "ぼ",
            KanaToken::Pa => "ぱ",
            KanaToken::Pi => "ぴ",
            KanaToken::Pu => "ぷ",
            KanaToken::Pe => "ぺ",
            KanaToken::Po => "ぽ",
            KanaToken::Kya => "きゃ",
            KanaToken::Kyu => "きゅ",
            KanaToken::Kyo => "きょ",
            KanaToken::Gya => "ぎゃ",
            KanaToken::Gyu => "ぎゅ",
            KanaToken::Gyo => "ぎょ",
            KanaToken::Sha => "しゃ",
            KanaToken::Shu => "しゅ",
            KanaToken::Sho => "しょ",
            KanaToken::Ja => "じゃ",
            KanaToken::Ju => "じゅ",
            KanaToken::Jo => "じょ",
            KanaToken::Cha => "ちゃ",
            KanaToken::Chu => "ちゅ",
            KanaToken::Cho => "ちょ",
            KanaToken::Nya => "にゃ",
            KanaToken::Nyu => "にゅ",
            KanaToken::Nyo => "にょ",
            KanaToken::Hya => "ひゃ",
            KanaToken::Hyu => "ひゅ",
            KanaToken::Hyo => "ひょ",
            KanaToken::Bya => "びゃ",
            KanaToken::Byu => "びゅ",
            KanaToken::Byo => "びょ",
            KanaToken::Pya => "ぴゃ",
            KanaToken::Pyu => "ぴゅ",
            KanaToken::Pyo => "ぴょ",
            KanaToken::Mya => "みゃ",
            KanaToken::Myu => "みゅ",
            KanaToken::Myo => "みょ",
            KanaToken::Rya => "りゃ",
            KanaToken::Ryu => "りゅ",
            KanaToken::Ryo => "りょ",
            KanaToken::LittleTsu => "っ",

            _ => "?"
        }
    }

    pub fn to_katakana(&self) -> &str {
        match &self {
            KanaToken::NonKana(x) => x,
            KanaToken::A => "ア",
            KanaToken::I => "イ",
            KanaToken::U => "ウ",
            KanaToken::E => "エ",
            KanaToken::O => "オ",
            KanaToken::Ka => "カ",
            KanaToken::Ki => "キ",
            KanaToken::Ku => "ク",
            KanaToken::Ke => "ケ",
            KanaToken::Ko => "コ",
            KanaToken::Sa => "サ",
            KanaToken::Shi => "シ",
            KanaToken::Su => "ス",
            KanaToken::Se => "セ",
            KanaToken::So => "ソ",
            KanaToken::Ta => "タ",
            KanaToken::Chi => "チ",
            KanaToken::Tsu => "ツ",
            KanaToken::Te => "テ",
            KanaToken::To => "ト",
            KanaToken::Na => "ナ",
            KanaToken::Ni => "ニ",
            KanaToken::Nu => "ヌ",
            KanaToken::Ne => "ネ",
            KanaToken::No => "ノ",
            KanaToken::Ha => "ハ",
            KanaToken::Hi => "ヒ",
            KanaToken::Fu => "フ",
            KanaToken::He => "ヘ",
            KanaToken::Ho => "ホ",
            KanaToken::Ma => "マ",
            KanaToken::Mi => "ミ",
            KanaToken::Mu => "ム",
            KanaToken::Me => "メ",
            KanaToken::Mo => "モ",
            KanaToken::Ya => "ヤ",
            KanaToken::Yu => "ユ",
            KanaToken::Yo => "ヨ",
            KanaToken::Ra => "ラ",
            KanaToken::Ri => "リ",
            KanaToken::Ru => "ル",
            KanaToken::Re => "レ",
            KanaToken::Ro => "ロ",
            KanaToken::Wa => "ワ",
            KanaToken::Wo => "ヲ",
            KanaToken::N => "ン",
            KanaToken::Ga => "ガ",
            KanaToken::Gi => "ギ",
            KanaToken::Gu => "グ",
            KanaToken::Ge => "ゲ",
            KanaToken::Go => "ゴ",
            KanaToken::Za => "ザ",
            KanaToken::Ji => "ジ",
            KanaToken::Zu => "ズ",
            KanaToken::Ze => "ゼ",
            KanaToken::Zo => "ゾ",
            KanaToken::Da => "ダ",
            KanaToken::De => "デ",
            KanaToken::Do => "ド",
            KanaToken::Ba => "バ",
            KanaToken::Bi => "ビ",
            KanaToken::Bu => "ブ",
            KanaToken::Be => "ベ",
            KanaToken::Bo => "ボ",
            KanaToken::Pa => "パ",
            KanaToken::Pi => "ピ",
            KanaToken::Pu => "プ",
            KanaToken::Pe => "ペ",
            KanaToken::Po => "ポ",
            KanaToken::Kya => "キャ",
            KanaToken::Kyu => "キュ",
            KanaToken::Kyo => "キョ",
            KanaToken::Gya => "ギャ",
            KanaToken::Gyu => "ギュ",
            KanaToken::Gyo => "ギョ",
            KanaToken::Sha => "シャ",
            KanaToken::Shu => "シュ",
            KanaToken::Sho => "ショ",
            KanaToken::Ja => "ジャ",
            KanaToken::Ju => "ジュ",
            KanaToken::Jo => "ジョ",
            KanaToken::Cha => "チャ",
            KanaToken::Chu => "チュ",
            KanaToken::Cho => "チョ",
            KanaToken::Nya => "ニャ",
            KanaToken::Nyu => "ニュ",
            KanaToken::Nyo => "ニョ",
            KanaToken::Hya => "ヒャ",
            KanaToken::Hyu => "ヒュ",
            KanaToken::Hyo => "ヒョ",
            KanaToken::Bya => "ビャ",
            KanaToken::Byu => "ビュ",
            KanaToken::Byo => "ビョ",
            KanaToken::Pya => "ピャ",
            KanaToken::Pyu => "ピュ",
            KanaToken::Pyo => "ピョ",
            KanaToken::Mya => "ミャ",
            KanaToken::Myu => "ミュ",
            KanaToken::Myo => "ミョ",
            KanaToken::Rya => "リャ",
            KanaToken::Ryu => "リュ",
            KanaToken::Ryo => "リョ",
            KanaToken::LittleTsu => "ッ",

            _ => "?"
        }
    }
}

pub struct Kanas(Vec<KanaToken>);

impl Kanas {
    pub fn to_hiraganas(&self) -> String {
        let mut output = String::new();

        for kana in &self.0 {
            output.push_str(kana.to_hiragana());
        }

        output
    }
    
    pub fn to_katakanas(&self) -> String {
        let mut output = String::new();

        for kana in &self.0 {
            output.push_str(kana.to_katakana());
        }

        output
    }

    /// Encode a vector of kana tokens from a Modified-Hepburn romaji sequence (https://en.wikipedia.org/wiki/Hepburn_romanization#Variants)
    pub fn from_hepburn(hepburn_sequence: &str) -> Self {
        let mut kanas = vec![];

        let chars = hepburn_sequence.chars();
        let mut iter = chars.into_iter();

        // Will accumulate characters until they can form a complete kana or non-kana value
        let mut accumulator = String::with_capacity(4);

        loop {
            match iter.next() {
                None => break,
                Some(ch) => {
                    enum KanaScanState {
                        /// We have valid candidate-characters for a kana, but still incomplete
                        MaybeKana,
                        /// We only have a non-kana value
                        NonKana,
                        /// We have a non-kana value followed by a kana candidate
                        NonKanaThenMaybeKana,
                        /// We have a valid kana
                        IsKana(KanaToken),
                        IsSokuon(KanaToken),
                        /// We have a valid kana followed by a kana candidate
                        IsKanaThenMaybeKana(KanaToken),
                        /// We have a non-kana value followed by a valid kana
                        NonKanaThenKana(KanaToken),
                    }

                    let kana: KanaScanState = match (accumulator.as_str(), ch) {
                        (x, 'a' | 'u' | 'i' | 'e' | 'o') if x.is_empty() => {
                            // returning NonKanaThenKana after each (.., <vowel>) without checking accumulator emptiness would be also fine, but it would incur a useless string copy
                            match ch {
                                'a' => KanaScanState::IsKana(KanaToken::A),
                                'i' => KanaScanState::IsKana(KanaToken::I),
                                'u' => KanaScanState::IsKana(KanaToken::U),
                                'e' => KanaScanState::IsKana(KanaToken::E),
                                'o' => KanaScanState::IsKana(KanaToken::O),
                                _ => unreachable!()
                            }
                        },

                        // TODO: add yoon
                        ("k", 'a') => KanaScanState::IsKana(KanaToken::Ka),
                        ("kk", 'a') => KanaScanState::IsSokuon(KanaToken::Ka),
                        ("s", 'a') => KanaScanState::IsKana(KanaToken::Sa),
                        ("ss", 'a') => KanaScanState::IsSokuon(KanaToken::Sa),
                        ("t", 'a') => KanaScanState::IsKana(KanaToken::Ta),
                        ("tt", 'a') => KanaScanState::IsSokuon(KanaToken::Ta),
                        ("n", 'a') => KanaScanState::IsKana(KanaToken::Na),
                        ("h", 'a') => KanaScanState::IsKana(KanaToken::Ha),
                        ("m", 'a') => KanaScanState::IsKana(KanaToken::Ma),
                        ("y", 'a') => KanaScanState::IsKana(KanaToken::Ya),
                        ("r", 'a') => KanaScanState::IsKana(KanaToken::Ra),
                        ("w", 'a') => KanaScanState::IsKana(KanaToken::Wa),
                        ("g", 'a') => KanaScanState::IsKana(KanaToken::Ga),
                        ("gg", 'a') => KanaScanState::IsSokuon(KanaToken::Ga),
                        ("z", 'a') => KanaScanState::IsKana(KanaToken::Za),
                        ("d", 'a') => KanaScanState::IsKana(KanaToken::Da),
                        ("dd", 'a') => KanaScanState::IsSokuon(KanaToken::Da),
                        ("b", 'a') => KanaScanState::IsKana(KanaToken::Ba),
                        ("bb", 'a') => KanaScanState::IsSokuon(KanaToken::Ba),
                        ("p", 'a') => KanaScanState::IsKana(KanaToken::Pa),
                        ("pp", 'a') => KanaScanState::IsSokuon(KanaToken::Pa),
                        (.., 'a') => KanaScanState::NonKanaThenKana(KanaToken::A),

                        // TODO: add yoon
                        ("k", 'i') => KanaScanState::IsKana(KanaToken::Ki),
                        ("kk", 'i') => KanaScanState::IsSokuon(KanaToken::Ki),
                        ("sh", 'i') => KanaScanState::IsKana(KanaToken::Shi),
                        ("ssh", 'i') => KanaScanState::IsSokuon(KanaToken::Shi),
                        ("ch", 'i') => KanaScanState::IsKana(KanaToken::Chi),
                        ("tch", 'i') => KanaScanState::IsSokuon(KanaToken::Chi),
                        ("n", 'i') => KanaScanState::IsKana(KanaToken::Ni),
                        ("h", 'i') => KanaScanState::IsKana(KanaToken::Hi),
                        ("m", 'i') => KanaScanState::IsKana(KanaToken::Mi),
                        ("r", 'i') => KanaScanState::IsKana(KanaToken::Ri),
                        ("g", 'i') => KanaScanState::IsKana(KanaToken::Gi),
                        ("j", 'i') => KanaScanState::IsKana(KanaToken::Ji),
                        ("b", 'i') => KanaScanState::IsKana(KanaToken::Bi),
                        ("bb", 'i') => KanaScanState::IsSokuon(KanaToken::Bi),
                        ("p", 'i') => KanaScanState::IsKana(KanaToken::Pi),
                        ("pp", 'i') => KanaScanState::IsSokuon(KanaToken::Pi),
                        (.., 'i') => KanaScanState::NonKanaThenKana(KanaToken::I),

                        // TODO: add yoon
                        ("k", 'u') => KanaScanState::IsKana(KanaToken::Ku),
                        ("kk", 'u') => KanaScanState::IsSokuon(KanaToken::Ku),
                        ("s", 'u') => KanaScanState::IsKana(KanaToken::Su),
                        ("ss", 'u') => KanaScanState::IsSokuon(KanaToken::Su),
                        ("ts", 'u') => KanaScanState::IsKana(KanaToken::Tsu),
                        ("tts", 'u') => KanaScanState::IsSokuon(KanaToken::Tsu),
                        ("n", 'u') => KanaScanState::IsKana(KanaToken::Nu),
                        ("f", 'u') => KanaScanState::IsKana(KanaToken::Fu),
                        ("m", 'u') => KanaScanState::IsKana(KanaToken::Mu),
                        ("y", 'u') => KanaScanState::IsKana(KanaToken::Yu),
                        ("r", 'u') => KanaScanState::IsKana(KanaToken::Ru),
                        ("g", 'u') => KanaScanState::IsKana(KanaToken::Gu),
                        ("gg", 'u') => KanaScanState::IsSokuon(KanaToken::Gu),
                        ("z", 'u') => KanaScanState::IsKana(KanaToken::Zu),
                        ("b", 'u') => KanaScanState::IsKana(KanaToken::Bu),
                        ("bb", 'u') => KanaScanState::IsSokuon(KanaToken::Bu),
                        ("p", 'u') => KanaScanState::IsKana(KanaToken::Pu),
                        ("pp", 'u') => KanaScanState::IsSokuon(KanaToken::Pu),
                        (.., 'u') => KanaScanState::NonKanaThenKana(KanaToken::U),

                        // TODO: add yoon
                        ("k", 'e') => KanaScanState::IsKana(KanaToken::Ke),
                        ("kk", 'e') => KanaScanState::IsSokuon(KanaToken::Ke),
                        ("s", 'e') => KanaScanState::IsKana(KanaToken::Se),
                        ("ss", 'e') => KanaScanState::IsSokuon(KanaToken::Se),
                        ("t", 'e') => KanaScanState::IsKana(KanaToken::Te),
                        ("tt", 'e') => KanaScanState::IsSokuon(KanaToken::Te),
                        ("n", 'e') => KanaScanState::IsKana(KanaToken::Ne),
                        ("h", 'e') => KanaScanState::IsKana(KanaToken::He),
                        ("m", 'e') => KanaScanState::IsKana(KanaToken::Me),
                        ("r", 'e') => KanaScanState::IsKana(KanaToken::Re),
                        ("g", 'e') => KanaScanState::IsKana(KanaToken::Ge),
                        ("gg", 'e') => KanaScanState::IsSokuon(KanaToken::Ge),
                        ("z", 'e') => KanaScanState::IsKana(KanaToken::Ze),
                        ("d", 'e') => KanaScanState::IsKana(KanaToken::De),
                        ("dd", 'e') => KanaScanState::IsSokuon(KanaToken::De),
                        ("b", 'e') => KanaScanState::IsKana(KanaToken::Be),
                        ("bb", 'e') => KanaScanState::IsSokuon(KanaToken::Be),
                        ("p", 'e') => KanaScanState::IsKana(KanaToken::Pe),
                        ("pp", 'e') => KanaScanState::IsSokuon(KanaToken::Pe),
                        (.., 'e') => KanaScanState::NonKanaThenKana(KanaToken::E),

                        // TODO: add yoon
                        ("k", 'o') => KanaScanState::IsKana(KanaToken::Ko),
                        ("kk", 'o') => KanaScanState::IsSokuon(KanaToken::Ko),
                        ("s", 'o') => KanaScanState::IsKana(KanaToken::So),
                        ("ss", 'o') => KanaScanState::IsSokuon(KanaToken::So),
                        ("t", 'o') => KanaScanState::IsKana(KanaToken::To),
                        ("tt", 'o') => KanaScanState::IsSokuon(KanaToken::To),
                        ("n", 'o') => KanaScanState::IsKana(KanaToken::No),
                        ("h", 'o') => KanaScanState::IsKana(KanaToken::Ho),
                        ("m", 'o') => KanaScanState::IsKana(KanaToken::Mo),
                        ("y", 'o') => KanaScanState::IsKana(KanaToken::Yo),
                        ("r", 'o') => KanaScanState::IsKana(KanaToken::Ro),
                        ("w", 'o') => KanaScanState::IsKana(KanaToken::Wo),
                        ("g", 'o') => KanaScanState::IsKana(KanaToken::Go),
                        ("gg", 'o') => KanaScanState::IsSokuon(KanaToken::Go),
                        ("z", 'o') => KanaScanState::IsKana(KanaToken::Zo),
                        ("d", 'o') => KanaScanState::IsKana(KanaToken::Do),
                        ("dd", 'o') => KanaScanState::IsSokuon(KanaToken::Do),
                        ("b", 'o') => KanaScanState::IsKana(KanaToken::Bo),
                        ("bb", 'o') => KanaScanState::IsSokuon(KanaToken::Bo),
                        ("p", 'o') => KanaScanState::IsKana(KanaToken::Po),
                        ("pp", 'o') => KanaScanState::IsSokuon(KanaToken::Po),
                        (.., 'o') => KanaScanState::NonKanaThenKana(KanaToken::O),

                        (x, 'k' | 's' | 't' | 'p' | 'c' | 'n' | 'h' | 'm' | 'y' | 'r' | 'w' | 'g' | 'z' | 'd' | 'b' | 'j') => {
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

                                    // for some other yoon
                                    ("k", 'y') => KanaScanState::MaybeKana,
                                    ("g", 'y') => KanaScanState::MaybeKana,
                                    ("n", 'y') => KanaScanState::MaybeKana,
                                    ("h", 'y') => KanaScanState::MaybeKana,
                                    ("b", 'y') => KanaScanState::MaybeKana,
                                    ("p", 'y') => KanaScanState::MaybeKana,
                                    ("m", 'y') => KanaScanState::MaybeKana,
                                    ("r", 'y') => KanaScanState::MaybeKana,

                                    ("ts", 'y') => KanaScanState::MaybeKana, // for tsyu

                                    ("n", ..) => KanaScanState::IsKanaThenMaybeKana(KanaToken::N),
                                    _ => KanaScanState::NonKanaThenMaybeKana
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
                            kanas.push(x);
                            accumulator.clear();
                        },
                        // We have a valid sokuon, we can push it and clear the accumulator
                        KanaScanState::IsSokuon(x) => {
                            kanas.push(KanaToken::LittleTsu);
                            kanas.push(x);
                            accumulator.clear();
                        },
                        // We have a valid kana, and the new character may also make up a valid kana, so we push and clear the accumulator, then add the new character to it
                        KanaScanState::IsKanaThenMaybeKana(x) => {
                            kanas.push(x);
                            accumulator.clear();
                            accumulator.push(ch);
                        }
                        // The new character won't make part of valid kana at all, so we push it together with the accumulator, and clear the latter
                        KanaScanState::NonKana => {
                            let mut x = String::from(accumulator.as_str());
                            x.push(ch);
                            kanas.push(KanaToken::NonKana(x));
                            accumulator.clear();
                        },
                        // The accumulator won't make up any valid kana, but the new character may do, so we push and clear the accumulator, then add this character to it
                        KanaScanState::NonKanaThenMaybeKana => {
                            kanas.push(KanaToken::NonKana(String::from(accumulator.as_str())));
                            accumulator.clear();
                            accumulator.push(ch);
                        },
                        // The accumulator may form a valid kana, we add the new character to it
                        KanaScanState::MaybeKana => {
                            accumulator.push(ch);
                        },
                        // The accumulator cannot form a valid kana but we have an already-valid kana after it, thus we push both values and clear the accumulator
                        KanaScanState::NonKanaThenKana(x) => {
                            kanas.push(KanaToken::NonKana(String::from(accumulator.as_str())));
                            kanas.push(x);
                            accumulator.clear();
                        }
                    }
                }
            }
        }

        Kanas(kanas)
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>().join(" ");

    let kanas = Kanas::from_hepburn(args.as_str());

    println!("{}", kanas.to_hiraganas());
    println!("{}", kanas.to_katakanas());
}

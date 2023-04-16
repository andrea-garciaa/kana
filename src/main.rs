use std::{env, str::Chars};

enum KanaToken {
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

    // Yoon
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

    // Extended (katakana only)
    
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
                        /// We have a non-kana value followed by a valid kana
                        NonKanaThenKana(KanaToken),
                    }

                    let kana: KanaScanState = match ch {
                        'a' => {
                            match accumulator.as_str() {
                                "k" => KanaScanState::IsKana(KanaToken::Ka),
                                "s" => KanaScanState::IsKana(KanaToken::Sa),
                                "t" => KanaScanState::IsKana(KanaToken::Ta),
                                "n" => KanaScanState::IsKana(KanaToken::Na),
                                "h" => KanaScanState::IsKana(KanaToken::Ha),
                                "m" => KanaScanState::IsKana(KanaToken::Ma),
                                "y" => KanaScanState::IsKana(KanaToken::Ya),
                                "r" => KanaScanState::IsKana(KanaToken::Ra),
                                "w" => KanaScanState::IsKana(KanaToken::Wa),
                                _ => KanaScanState::NonKanaThenKana(KanaToken::A)
                            }
                        },
                        'i' => {
                            match accumulator.as_str() {
                                "k" => KanaScanState::IsKana(KanaToken::Ki),
                                "sh" => KanaScanState::IsKana(KanaToken::Shi),
                                "ch" => KanaScanState::IsKana(KanaToken::Chi),
                                "n" => KanaScanState::IsKana(KanaToken::Ni),
                                "h" => KanaScanState::IsKana(KanaToken::Hi),
                                "m" => KanaScanState::IsKana(KanaToken::Mi),
                                "r" => KanaScanState::IsKana(KanaToken::Ri),
                                
                                x => { 
                                    if x.is_empty() {
                                        KanaScanState::IsKana(KanaToken::I)
                                    } else {
                                        KanaScanState::NonKanaThenKana(KanaToken::I)
                                    }
                                }
                            }
                        },
                        'u' => {
                            match accumulator.as_str() {
                                "k" => KanaScanState::IsKana(KanaToken::Ku),
                                "s" => KanaScanState::IsKana(KanaToken::Su),
                                "ts" => KanaScanState::IsKana(KanaToken::Tsu),
                                "n" => KanaScanState::IsKana(KanaToken::Nu),
                                "f" => KanaScanState::IsKana(KanaToken::Fu),
                                "m" => KanaScanState::IsKana(KanaToken::Mu),
                                "y" => KanaScanState::IsKana(KanaToken::Yu),
                                "r" => KanaScanState::IsKana(KanaToken::Ru),

                                x => { 
                                    if x.is_empty() {
                                        KanaScanState::IsKana(KanaToken::U)
                                    } else {
                                        KanaScanState::NonKanaThenKana(KanaToken::U)
                                    }
                                }
                            }
                        },
                        'e' => {
                            match accumulator.as_str() {
                                "k" => KanaScanState::IsKana(KanaToken::Ke),
                                "s" => KanaScanState::IsKana(KanaToken::Se),
                                "t" => KanaScanState::IsKana(KanaToken::Te),
                                "n" => KanaScanState::IsKana(KanaToken::Ne),
                                "h" => KanaScanState::IsKana(KanaToken::He),
                                "m" => KanaScanState::IsKana(KanaToken::Me),
                                "r" => KanaScanState::IsKana(KanaToken::Re),

                                x => { 
                                    if x.is_empty() {
                                        KanaScanState::IsKana(KanaToken::E)
                                    } else {
                                        KanaScanState::NonKanaThenKana(KanaToken::E)
                                    }
                                }
                            }
                        },
                        'o' => {
                            match accumulator.as_str() {
                                "k" => KanaScanState::IsKana(KanaToken::Ko),
                                "s" => KanaScanState::IsKana(KanaToken::So),
                                "t" => KanaScanState::IsKana(KanaToken::To),
                                "n" => KanaScanState::IsKana(KanaToken::No),
                                "h" => KanaScanState::IsKana(KanaToken::Ho),
                                "m" => KanaScanState::IsKana(KanaToken::Mo),
                                "y" => KanaScanState::IsKana(KanaToken::Yo),
                                "r" => KanaScanState::IsKana(KanaToken::Ro),
                                "w" => KanaScanState::IsKana(KanaToken::Wo),
                                _ => KanaScanState::NonKanaThenKana(KanaToken::O)
                            }
                        },
                        'k' | 's' | 't' | 'c' | 'n' | 'h' | 'm' | 'y' | 'r' | 'w' | 'g' | 'z' | 'd' | 'b' | 'p' => {
                            KanaScanState::MaybeKana
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
                        // The new character won't make part of valid kana at all, so push it together with the accumulator, and clear the latter
                        KanaScanState::NonKana => {
                            let mut x = String::from(accumulator.as_str());
                            x.push(ch);
                            kanas.push(KanaToken::NonKana(x));
                            accumulator.clear();
                        },
                        // The accumulator won't make up any valid kana, but the new character may do, so push and clear the accumulator, and add the new character to it
                        KanaScanState::NonKanaThenMaybeKana => {
                            kanas.push(KanaToken::NonKana(String::from(accumulator.as_str())));
                            accumulator.clear();
                            accumulator.push(ch);
                        },
                        // The accumulator may form a valid kana, add the new character to it
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

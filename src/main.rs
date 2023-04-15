use std::{env, str::Chars};

enum KanaToken {
    NonKana(String),

    // Base
    A,  I,   U,   E,  O,
    Ka, Ki,  Ku,  Ke, Ko, 
    Sa, Shi, Su,  Se, So,       // Variant
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

    // Combo
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
    pub fn to_hiragana(&self) -> String {
        String::from(match &self {
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
            
            _ => "?"
        })
    }

    pub fn to_katakana(&self) -> String {
        String::from(match &self {
            KanaToken::A => "き",
            KanaToken::NonKana(x) => x,
            _ => "?"
        })
    }
}

pub struct Kanas(Vec<KanaToken>);

impl Kanas {
    pub fn to_hiraganas(&self) -> String {
        let mut output = String::new();

        for kana in &self.0 {
            output.push_str(&kana.to_hiragana());
        }

        output
    }
    
    pub fn to_katakanas(&self) -> String {
        let mut output = String::new();

        for kana in &self.0 {
            output.push_str(&kana.to_katakana());
        }

        output
    }

    /// Encode a vector of kana tokens from a Modified-Hepburn romaji sequence (https://en.wikipedia.org/wiki/Hepburn_romanization#Variants)
    pub fn from_hepburn(hepburn_sequence: &String) -> Self {
        let chars = hepburn_sequence.as_str().chars();
        let mut kanas = vec![];

        for ch in chars {
            match ch {
                'a' | 'i' | 'u' | 'e' | 'o' => {
                    kanas.push(match ch {
                        'a' => KanaToken::A,
                        'i' => KanaToken::I,
                        'u' => KanaToken::U,
                        'e' => KanaToken::E,
                        'o' => KanaToken::O,
                        _ => KanaToken::NonKana(String::from(ch)),
                    });
                },
                _ => { kanas.push(KanaToken::NonKana(String::from(ch))); }
            }
        }

        Kanas(kanas)
    }
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>().join(" ");

    let kanas = Kanas::from_hepburn(&args);

    println!("{}", kanas.to_hiraganas());
    println!("{}", kanas.to_katakanas());
}

use std::{env, str::Chars};

enum KanaToken {
    // Base
    A,  I,   U,   E,  O,
    Ka, Ki,  Ku,  Ke, Ko, 
    Sa, Shi, Su,  Se, So, 
    Ta, Chi, Tsu, Te, To, 
    Na, Ni,  Nu,  Ne, No, 
    Ha, Hi,  Fu,  He, Ho, 
    Ma, Mi,  Mu,  Me, Mo, 
    Ya, Yi,  Yu,  Ye, Yo, 
    Ra, Ri,  Ry,  Re, Ro, 
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

    NonKana(String),
}

impl KanaToken {
    pub fn to_hiragana(&self) -> String {
        String::from(match &self {
            KanaToken::A => "あ",
            KanaToken::NonKana(x) => x,
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
    pub fn to_hiragana(&self) -> String {
        let mut output = String::new();

        for kana in &self.0 {
            output.push_str(&kana.to_hiragana());
        }

        output
    }
    
    pub fn to_katakana(&self) -> String {
        let mut output = String::new();

        for kana in &self.0 {
            output.push_str(&kana.to_katakana());
        }

        output
    }

    pub fn from_romaji(romaji: &String) -> Self {
        let chars = romaji.as_str().chars();
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

    let kanas = Kanas::from_romaji(&args);

    println!("{}", kanas.to_hiragana());
    println!("{}", kanas.to_katakana());
}

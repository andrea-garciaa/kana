# kana
Practicing Rust by making an alphabetic transcription program between Hepburn romaji and hiragana/katakana strings.

## Usage
First argument is number of iterations (or 1 if not found / misformatted), then all other arguments make up a romaji sequence. Invalid characters (in kana alphabet) will be normally outputted together with valid kanas, in input order. There may be some pitfalls, such as sokuon being valid even without prefix kana (`ttsu` being transformed to `っつ` no matter what comes before).

### Example
```shell
kana 1000000 aiueosashisusesonaninunenokakikukeko gya gyu pyu wa suzuki tsuzuru furanssu nonsense
```
### Output
```shell
iteration count: 1000000
679.3186 ms
あいうえおさしすせそなにぬねのかきくけこ ぎゃ ぎゅ ぴゅ わ すずき つずる ふらんっす のんせんせ
アイウエオサシスセソナニヌネノカキクケコ ギャ ギュ ピュ ワ スズキ ツズル フランッス ノンセンセ
```

## License
See the LICENSE file.
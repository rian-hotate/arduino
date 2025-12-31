# 環境構築
- VSCodeでの開発を想定

## 拡張子のインスール
1. `Arduino Community EditionPreview`のインストール.  
1. 完了後、⌘+Shft+PでReloadWindowを選択して読み込み

## Boadの選択
1. VSCodeのUI右下からSelect Boardを押下
1. Arduino Unoを選択
1. VSCodeのUI右下からserial portを押下
1. usbmodem212201を選択
1. ⌘+Shift+P後、Arduino: Rebuild IntelliSense Configurationを選択

## RUST環境構築
1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - 途中の質問は「1」を選択でOK
1. `rustc --version`と`cargo --version`でバージョンが表示されることを確認
1. `xcode-select --install`
    - インストール済みならOK
1. `brew tap osx-cross/avr`
1. `brew install avr-gcc avrdude`
1. `cargo install cargo-generate`
1. `cargo install ravedude`

### Rust + ArduinoのPJ作成
1. `cargo generate --git https://github.com/Rahix/avr-hal-template.git`
    - PJ名とBoadを選択
    - 今回は「arduino-uno」
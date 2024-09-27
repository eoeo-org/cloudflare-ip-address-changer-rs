# Cloudflare IP Address Changer - Rust

IPv4 / IPV6 アドレスを自動で取得して、Cloudflare の設定を自動変更するツールです。
このプログラムは、定期的に自動設定するようにするために作られた物です。

## 使い方

1. [リリースページ](https://github.com/Zel9278/cloudflare-ip-address-changer-rs/releases/latest)から対象の実行ファイルをダウンロードします
2.  `config.toml` を編集します
    ```
    zone_id = "ドメインページの右側にAPI ゾーン IDがあるのでそれを持ってきます"
    dns_record = "対象の DNS レコードを指定します 例：<サブドメイン>.c30.life, 例2: c30.life"
    dns_type = "AAAAかAを指定します、AAAAの場合はipv6が自動で割り当てられます、Aの場合はipv4が自動で割り当てられます"
    dns_proxy = trueかfalseを指定します、この設定はCloudFlareのプロキシを使用するかの設定です
    auth_key = "CloudFlareAPIキーを指定します"
    ```
  API キーは以下の手順で入手できます。
  1. アカウントページの `ホーム`
  2. 左側の下にある `アカウントの管理 > APIトークン`
  3. `トークンを生成する`
  4. ゾーン DNS を編集する の`テンプレートを使用する`
  5. `ゾーン リソース`の`Select...`を対象のドメインを指定します。
  6. `概要に進む`
  7. `トークンを作成する`
  9. トークンが生成されるので、それをコピーしてauth_keyにコピーします。
3. プログラムを実行すると自動でcloudflareに割り降られるようになります。

## ビルド方法

1. Rust がインストールされていることを確認します。
   - Rust がインストールされていない場合、[公式サイト](https://rust-lang.org/)の手順に従ってインストールしてください。
   - Rust が正しくインストールされていることを確認するには、以下のコマンドをターミナルで実行します。

    ```bash
    rustc --version
    ```
    バージョン情報が表示されれば、インストールが成功しています。
2. このリポジトリをクローンします。

    ```bash
    git clone https://github.com/eoeo-org/cloudflare-ip-address-changer-rs.git
    cd cloudflare-ip-address-changer-rs
    ```
3. 必要な依存関係をインストールします。Cargo.toml ファイルに定義されている依存関係は、自動でインストールされます。

    ```bash
    cargo build --release
    ```
4. ビルドが成功すると、target/release/ ディレクトリに実行ファイルが生成されます。

    ```bash
    target/release/cloudflare-ip-address-changer-rs
    ```
5. 実行ファイルを実行するために、config.toml ファイルをプロジェクトのルートディレクトリに配置して、前述の設定を行います。

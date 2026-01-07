# cargo-clean-all

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

Rustプロジェクトの`target`ディレクトリを定期的にクリーンアップするCLIツール。

ログ記録、macOS通知、自動実行のスケジューリングをサポートし、ディスク容量を効率的に管理できます。

## ✨ 機能

- 🔍 **自動検出**: 指定ディレクトリ下のすべてのRust targetディレクトリを検索
- 🏃 **ドライランモード**: 削除前にプレビュー可能
- 🔔 **macOS通知**: クリーンアップ完了時に通知
- 📝 **詳細ログ**: すべての操作をログに記録
- ⚙️ **設定ファイル**: TOML形式で柔軟にカスタマイズ
- 🤖 **自動実行**: LaunchAgentによる週次スケジュール実行
- 🛡️ **安全**: `Cargo.toml`が存在するプロジェクトのみクリーンアップ
- ⚠️ **パス検証**: 外部ドライブが未マウント時に警告

## 🚀 クイックスタート

### 簡単インストール（推奨）

```bash
git clone https://github.com/yourusername/cargo-clean-all.git
cd cargo-clean-all
./install.sh
```

インストーラーが以下を自動実行します：
1. バイナリのビルドとインストール（`~/.cargo/bin/`）
2. デフォルト設定ファイルの作成
3. 自動週次クリーンアップの設定（オプション）

### 手動インストール

```bash
# ビルド & インストール
cargo install --path .

# 設定ファイルのセットアップ
mkdir -p ~/.config/cargo-clean-all
cp config.example.toml ~/.config/cargo-clean-all/config.toml

# 設定ファイルを編集してスキャンパスを設定
nano ~/.config/cargo-clean-all/config.toml
```

## 📖 使い方

### 基本コマンド

```bash
# ドライラン（削除せず確認のみ、初回実行推奨）
cargo-clean-all --dry-run

# クリーンアップ実行
cargo-clean-all

# 詳細出力
cargo-clean-all --verbose
```

### 実行例

```
🔍 Scanning /path/to/projects...
   Found 3 target directories
🧹 Cleaning 3 target directories...
  [1/3] ✅
  [2/3] ✅
  [3/3] ✅

✨ Cleanup complete!
  Success: 3
  Errors: 0
  Freed: 2.75 GB
```

## 🤖 自動クリーンアップ（macOSのみ）

インストーラーで設定した場合、**毎週日曜日の深夜2時**に自動実行されます。

### 手動でLaunchAgentを設定

```bash
# plistをテンプレートから生成
sed "s|{{HOME}}|$HOME|g" com.user.cargo-clean-all.plist.template > ~/Library/LaunchAgents/com.user.cargo-clean-all.plist

# LaunchAgentをロード
launchctl load ~/Library/LaunchAgents/com.user.cargo-clean-all.plist
```

### LaunchAgent管理コマンド

```bash
# 状態確認
launchctl list | grep cargo-clean-all

# 手動トリガー
launchctl start com.user.cargo-clean-all

# 停止とアンロード
launchctl unload ~/Library/LaunchAgents/com.user.cargo-clean-all.plist
```

### 📊 なぜ週1回なのか？

デフォルトで週1回のスケジュールになっているのは、以下をバランスさせるためです：

- **SSD寿命**: 毎日実行すると年間約1TBの書き込みが発生しますが、週1回にすることで約143GB/年に削減
- **ディスク容量**: ビルド成果物の過剰な蓄積を防止
- **ファイルシステムの健全性**: 大規模なビルドディレクトリによるファイルシステムの破損リスクを軽減

実行頻度を変更したい場合は、plistファイルの`StartCalendarInterval`を編集してください。

## 設定ファイル

設定ファイル: `~/.config/cargo-clean-all/config.toml`

```toml
[paths]
# クリーンアップ対象のルートディレクトリ
scan_roots = ["/Volumes/Dev-SSD/dev"]

# 除外するディレクトリ名
exclude_dirs = ["node_modules", ".git"]

[cleanup]
# targetディレクトリのみをクリーンアップ
target_only = true

# 最小サイズ（MB）これ以下は削除しない
min_size_mb = 10

[logging]
# ログファイルのパス
log_file = "~/.local/share/cargo-clean-all/clean.log"

# ログレベル (debug, info, warn, error)
level = "info"

# ログローテーション（ファイル数）
max_files = 10

[notification]
# 通知を有効化
enabled = true

# 通知のタイトル
title = "Cargo Clean All"

# エラー時のみ通知
error_only = false
```

## ログ

- **実行ログ**: `~/.local/share/cargo-clean-all/clean.log`
- **LaunchAgent出力**: `~/.local/share/cargo-clean-all/launchd.out.log`
- **LaunchAgentエラー**: `~/.local/share/cargo-clean-all/launchd.err.log`

ログを確認:

```bash
tail -f ~/.local/share/cargo-clean-all/clean.log
```

## アンインストール

### 1. LaunchAgentの停止と削除

```bash
launchctl unload ~/Library/LaunchAgents/com.user.cargo-clean-all.plist
rm ~/Library/LaunchAgents/com.user.cargo-clean-all.plist
```

### 2. バイナリの削除

```bash
cargo uninstall cargo-clean-all
```

### 3. 設定とログの削除（オプション）

```bash
rm -rf ~/.config/cargo-clean-all
rm -rf ~/.local/share/cargo-clean-all
```

## トラブルシューティング

### LaunchAgentが動作しない

ログを確認:

```bash
cat ~/.local/share/cargo-clean-all/launchd.err.log
```

手動実行してエラーを確認:

```bash
launchctl start com.user.cargo-clean-all
```

### 権限エラー

Dev-SSDへのアクセス権限を確認:

```bash
ls -la /Volumes/Dev-SSD/dev
```

### 外部SSDが外れている場合

**動作**: 外部ディスクがマウントされていない場合、ツールは以下のように動作します：

1. **警告表示**: 見つからないパスが警告として表示されます
2. **通知送信**: macOS通知センターで警告通知が送られます
3. **ログ記録**: `ERROR: Path not found: /Volumes/...` がログに記録されます
4. **他のパスを続行**: 他にアクセス可能なパスがあれば、それらのクリーンアップは続行されます

**出力例**:

```
⚠️  Warning: Path not found: /Volumes/Dev-SSD/dev

⚠️  Warning: 1 path(s) not accessible
This may happen if external drives are not mounted.
```

**対処法**:

- 外部SSDを接続してマウントする
- または、設定ファイル (`~/.config/cargo-clean-all/config.toml`) から該当パスを削除/コメントアウト

## 🗑️ アンインストール

```bash
# LaunchAgentの停止と削除
launchctl unload ~/Library/LaunchAgents/com.user.cargo-clean-all.plist
rm ~/Library/LaunchAgents/com.user.cargo-clean-all.plist

# バイナリのアンインストール
cargo uninstall cargo-clean-all

# 設定とログの削除（オプション）
rm -rf ~/.config/cargo-clean-all
rm -rf ~/.local/share/cargo-clean-all
```

## 🛡️ 安全性

- **ソースコード保護**: `target/`ディレクトリのみ削除、`src/`や`Cargo.toml`は絶対に削除しません
- **検証**: 削除前に親ディレクトリに`Cargo.toml`が存在することを確認
- **ドライランモード**: 実際の削除前に変更内容をプレビュー可能
- **詳細ログ**: すべての操作を監査用にログ記録

## 💡 このツールが生まれた背景

Rustプロジェクトの`target`ディレクトリは、ビルド成果物が蓄積すると数GBに達することがあります。特に外部SSDを使用している場合：

- **ディスク容量の圧迫**: 複数のプロジェクトで急速にストレージを消費
- **ファイルシステムの不整合**: 大量の小ファイルによる破損リスク
- **SSD寿命**: 頻繁なビルドとクリーンアップの繰り返しによる書き込み量の増加

このツールは、これらの問題をバランス良く解決するために、定期的かつ自動的なクリーンアップを提供します。

## 🤝 貢献

Pull Requestを歓迎します！バグ報告や機能リクエストは、GitHubのIssuesでお願いします。

## 📄 ライセンス

このプロジェクトはMITライセンスの下で公開されています - 詳細は [LICENSE](LICENSE) ファイルをご覧ください。

---

**注意**: このツールはmacOS向けに開発されましたが、コア機能（スキャンとクリーンアップ）はUnix系システムでも動作します。LaunchAgent機能はmacOS専用です。

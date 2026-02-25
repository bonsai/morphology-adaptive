# Morphology Adaptive Game - Rust Implementation

## 概要
このプロジェクトは、WASM（WebAssembly）としてコンパイルされるRustコードを使用して、形態適応型のレースゲームを実装しています。このゲームは、異なる身体構造（二足歩行、四足歩行、六足歩行）を持つキャラクターを操作し、地形に応じて最適な移動方法を選択する能力をシミュレートします。

## ファイル構造
```
/workspace/
├── Cargo.toml          # Rustプロジェクトのマニフェストファイル
├── src/
│   └── lib.rs          # メインのRustソースコード（WASM向け）
├── README.Rust.MD      # このドキュメント
├── README.WASM.MD      # WASM実装に関するドキュメント
└── (その他のファイル)
```

## 技術スタック
- **言語**: Rust
- **ターゲット**: WebAssembly (WASM)
- **バインディング**: wasm-bindgen
- **数学ライブラリ**: nalgebra (3Dベクトル計算)
- **Web APIアクセス**: web-sys, js-sys
- **乱数生成**: rand

## 主要な構造体

### Morphology
ゲーム内でサポートされている身体構造の列挙型：
- `Biped`: 二足歩行
- `Quadruped`: 四足歩行
- `Hexapod`: 六足歩行

### Muscle
筋肉構造を表す構造体：
- `origin`: 始点位置（Vector3<f32>）
- `insertion`: 終点位置（Vector3<f32>）
- `max_force`: 最大出力（f32）
- `activation`: 筋収縮率（f32）

### Limb
四肢構造を表す構造体：
- `position`: 位置（Vector3<f32>）
- `rotation`: 回転（Vector3<f32>）
- `length`: 長さ（f32）
- `muscles`: 関連する筋肉のリスト（Vec<Muscle>）

### Creature
ゲーム内のキャラクターを表す構造体：
- `morphology`: 身体構造（Morphology）
- `position`: 位置（Vector3<f32>）
- `rotation`: 回転（Vector3<f32>）
- `limbs`: 四肢のリスト（Vec<Limb>）
- `velocity`: 速度（Vector3<f32>）
- `acceleration`: 加速度（Vector3<f32>）
- `mass`: 質量（f32）

## 主要な機能

### Creature::new(morphology)
指定された身体構造に基づいて新しいキャラクターを作成します。各身体構造に応じた四肢と質量が自動的に設定されます。

### Creature::update(dt, forces)
物理演算を更新します：
- 重力と外部力を適用
- ニュートンの運動方程式（F = ma）を使用して加速度を計算
- 速度と位置を更新
- 地面との衝突判定（簡単な実装）

### Creature::apply_muscle_activation(activations)
筋肉の収縮率を設定します。これにより、キャラクターの移動が制御されます。

### GameWorld構造体
ゲーム全体の状態を管理するWASM用の構造体：
- 複数のキャラクターを保持
- 地形情報を保存（HashMap）
- WASMバインディングを通じてJavaScriptからアクセス可能

## WASMエクスポート関数

### new()
新しいゲームワールドを作成します。

### add_creature(morphology)
指定された身体構造のキャラクターを追加し、そのインデックスを返します。

### update_creature(idx, dt, forces_json)
指定されたキャラクターを物理演算で更新します。力をJSON形式で受け取ります。

### activate_muscles(idx, activations_json)
指定されたキャラクターの筋肉活性を更新します。

### get_creature_position(idx), get_creature_rotation(idx), 他
JavaScript側でキャラクター情報を取得するためのgetter関数群。

## 使用方法

このRustコードは、wasm-packを使用してWASMにコンパイルすることを想定しています：

```bash
wasm-pack build --target web
```

生成されたWASMモジュールは、JavaScriptからインポートして使用できます。

## 特徴

- **形態適応性**: 異なる身体構造（二足、四足、六足）に対応
- **物理ベースの移動**: 筋肉駆動の運動モデル
- **WASM互換**: ブラウザで高速に動作
- **JavaScript統合**: wasm-bindgenを使用した双方向通信
- **拡張性**: 新しい身体構造や地形タイプを簡単に追加可能

この実装は、元のPythonリポジトリの形態適応の研究コンセプトをWASM環境に移植し、インタラクティブなゲーム体験として再現しています。
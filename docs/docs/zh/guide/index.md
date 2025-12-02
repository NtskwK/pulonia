# 快速开始

## 使用方法

Pulonia 是一个命令行工具。你可以使用它来生成两个压缩文件之间的差分补丁。

```bash
pulonia --before <old_archive> --after <new_archive> [options]
```

### 选项

- `-b, --before <PATH>`: 旧版本压缩文件的路径（必需）。
- `-a, --after <PATH>`: 新版本压缩文件的路径（必需）。
- `-o, --output <PATH>`: 生成的补丁文件的输出路径（默认值：`ota`）。
- `--temp <PATH>`: 解压缩的临时目录路径。
- `--format <FORMAT>`: 补丁文件格式（例如：bsdiff、zstd）。

## 示例

```bash
pulonia -b app-v1.zip -a app-v2.zip -o update.patch
```

## 支持的格式

Pulonia 支持多种压缩格式：

- **ZIP** - 最常见的压缩格式
- **TAR** - Unix 传统打包格式
- **GZIP** (.tar.gz) - TAR + GZIP 压缩
- **XZ** (.tar.xz) - TAR + XZ 压缩
- **BZIP2** (.tar.bz2) - TAR + BZIP2 压缩
- **LZ4** (.tar.lz4) - TAR + LZ4 压缩
- **7Z** - 7-Zip 格式

## 工作原理

Pulonia 通过以下步骤生成差分补丁：

1. **解压缩**: 将旧版本和新版本的压缩文件解压到临时目录
2. **对比**: 使用 SHA-256 哈希算法比较文件内容，精确检测所有变更
3. **分析**: 识别以下三种变更类型：
   - 新增文件
   - 修改文件
   - 删除文件
4. **生成报告**: 创建 JSON 格式的迁移报告（遵循 Migration Protocol v1）
5. **创建补丁**: 生成只包含已更改和新增文件的补丁文件

## 实际应用

### 软件更新场景

```bash
# 为应用程序版本 1.0 到 2.0 的升级生成补丁
pulonia -b my-app-v1.0.zip -a my-app-v2.0.zip -o my-app-update.zip
```

### CI/CD 集成

```bash
#!/bin/bash
# 在 CI/CD 流程中自动生成更新补丁
pulonia \
  --before ./builds/release-v${OLD_VERSION}.zip \
  --after ./builds/release-v${NEW_VERSION}.zip \
  --output ./releases/update-${OLD_VERSION}-to-${NEW_VERSION}.zip
```


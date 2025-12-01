---
pageType: home

hero:
  name: Pulonia
  text: 软件更新补丁生成工具
  tagline: 一个用于比较两个压缩文件并生成更新补丁的开源程序
  actions:
    - theme: brand
      text: 快速开始
      link: zh/guide/
    - theme: alt
      text: GitHub
      link: https://github.com/NtskwK/pulonia
features:
  - title: 多格式压缩文件支持
    details: 支持 ZIP、TAR、GZIP、XZ、BZIP2、LZ4、7Z 等多种常见压缩格式的解压和比对。
    icon: 📦
  - title: 高性能 Rust 实现
    details: 基于 Rust 编写，提供极高的性能和内存安全保证，适合处理大型压缩文件。
    icon: 🚀
  - title: 智能文件差异检测
    details: 使用 SHA-256 哈希算法精确识别文件变化，自动区分新增、修改和删除的文件。
    icon: 🔍
  - title: 结构化迁移报告
    details: 生成符合迁移协议 v1 标准的 JSON 报告，清晰记录所有文件变更信息。
    icon: 📋
  - title: 灵活的命令行接口
    details: 提供简洁直观的 CLI 工具，支持自定义输出路径和临时目录等配置选项。
    icon: ⚙️
  - title: 完善的日志系统
    details: 内置详细的日志记录功能，方便追踪补丁生成过程中的每一步操作。
    icon: 📜
---

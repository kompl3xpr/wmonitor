# WMonitor - wplace 领地监控 Discord 机器人

[![Unlicense](https://img.shields.io/badge/license-Unlicense-yellow.svg)](https://github.com/kompl3xpr/wmonitor/blob/master/LICENSE)
[![Check](https://github.com/kompl3xpr/wmonitor/actions/workflows/ci.yaml/badge.svg)](https://github.com/kompl3xpr/wmonitor/actions/workflows/ci.yaml)
![GitHub Issues](https://img.shields.io/github/issues/kompl3xpr/wmonitor)
![Language Count](https://img.shields.io/github/languages/count/kompl3xpr/wmonitor)
![Repo Size](https://img.shields.io/github/repo-size/kompl3xpr/wmonitor)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange)

一个专为 [wplace.live](https://wplace.live) 设计的智能领地监控 Discord 机器人，自动检测领地变化并及时通知相关成员。

## ✨ 特性

- 🔍 **自动监控** - 定期检查领地状态变化
- 🎯 **精确检测** - 支持遮罩区域设置，只关注重要区域
- 👥 **成员管理** - 灵活的权限系统和成员通知
- ⚡ **高性能** - 使用 Rust 编写，响应迅速
- 🔧 **易于使用** - 直观的 Discord 斜杠命令

## 🚀 快速开始

### 系统要求

- Git
- Rust 1.70 或更高版本
- Discord 机器人令牌
- Discord 频道 ID

### 安装步骤

1. **克隆仓库**
   ```bash
   git clone https://github.com/kompl3xpr/wmonitor.git
   cd wmonitor
   ```

2. **构建项目**
   - **Windows:**
     ```cmd
     .\build.bat
     ```
   - **Linux & macOS:**
     ```bash
     ./build.sh
     ```

3. **配置环境变量**
   创建 `.env` 文件并设置：
   ```env
   DISCORD_TOKEN=你的Discord机器人令牌
   DATABASE_URL=sqlite://db/wmonitor.db
   NOTIFICATION_CHANNEL_ID=bot监听命令和发送的Discord频道ID
   ```

4. **运行机器人**
   - **Windows:**
     ```cmd
     cd .\bin
     .\wmonitor.exe
     ```
   - **Linux & macOS:**
     ```bash
     cd ./bin
     ./wmonitor
     ```

## 📖 使用指南

### 基础配置流程

1. **创建领地**
   ```
   /wmfief add 旗帜
   ```

2. **设置检查间隔**
   ```
   /wmfief settime 旗帜 60
   ```

3. **添加监控区块**
   ```
   /wmchunk add 旗帜 西区 500,500
   /wmchunk refnow 旗帜 西区
   /wmchunk setmask 旗帜 西区
   ```
   *然后上传遮罩图片定义监控区域*

4. **添加成员**（可选）
   ```
   /wmuser join @用户名 旗帜
   /wmuser allow @用户名 CHUNK_EDIT
   ```

### 获取坐标信息

在 Blue Marble 插件中查找 `t_x` 和 `t_y` 坐标值，这些将用于区块监控设置。

## 🛠 命令参考

### 基本信息
| 命令 | 描述 |
|------|------|
| `/wmhelp` | 显示帮助信息 |
| `/wmfetch <x> <y>` | 从 wplace.live 获取指定坐标的区块图片 |
| `/wmpermissions` | 查看权限类型说明 |

### 领地管理
| 命令 | 描述 |
|------|------|
| `/wmfief add <名称>` | 创建新领地 |
| `/wmfief remove <名称>` | 删除领地 |
| `/wmfief check <名称>` | 手动检查领地状态 |
| `/wmfief rename <旧名> <新名>` | 重命名领地 |
| `/wmfief settime <名称> <分钟>` | 设置自动检查间隔 |
| `/wmfief enable/disable <名称>` | 启用/禁用自动检查 |
| `/wmfief info <名称>` | 查看领地信息 |

### 区块管理
| 命令 | 描述 |
|------|------|
| `/wmchunk add <领地> <区块名> <x,y>` | 添加区块 |
| `/wmchunk remove <领地> <区块名>` | 删除区块 |
| `/wmchunk rename <领地> <旧名> <新名>` | 重命名区块 |
| `/wmchunk setref <领地> <区块名>` | 上传参考图片 |
| `/wmchunk refnow <领地> <区块名>` | 设置当前状态为参考图 |
| `/wmchunk setmask <领地> <区块名>` | 设置监控区域遮罩 |
| `/wmchunk setpos <领地> <区块名> <x,y>` | 修改区块坐标 |
| `/wmchunk info <领地> <区块名>` | 查看区块信息 |

### 用户管理
| 命令 | 描述 |
|------|------|
| `/wmuser join <@用户> <领地>` | 添加用户到领地 |
| `/wmuser leave <@用户> <领地>` | 从领地移除用户 |
| `/wmuser allow <@用户> <权限>` | 授予用户权限 |
| `/wmuser deny <@用户> <权限>` | 撤销用户权限 |
| `/wmuser info <@用户>` | 查看用户信息 |

### 管理员命令
| 命令 | 描述 |
|------|------|
| `/wmop op <@用户>` | 添加管理员 |
| `/wmop deop <@用户>` | 移除管理员 |
| `/wmop listop` | 显示所有管理员 |
| `/wmop stop/start` | 停止/启动机器人 |
| `/wmop fiefs` | 列出所有领地 |

## 🤝 贡献指南

我们欢迎各种形式的贡献！请参阅以下指南：

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/kompl3xpr/wmonitor.git
cd wmonitor

# 安装依赖
cargo build

# 运行测试
cargo test
```

## 📝 许可证

本项目采用 [The Unlicense](LICENSE) 许可证。

## 👥 贡献者

感谢所有为本项目做出贡献的开发者：

<a href="https://github.com/kompl3xpr/wmonitor/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=kompl3xpr/wmonitor" />
</a>

## ❓ 常见问题

**Q: 机器人没有响应命令？**
A: 确保机器人已获得正确的权限，并且命令前缀正确。

**Q: 如何获取准确的区块坐标？**
A: 在 wplace.live 上使用 Blue Marble 插件的调试信息获取 `t_x` 和 `t_y` 值。

**Q: 遮罩图片有什么要求？**
A: 遮罩图片应为黑白图片，白色区域表示需要监控的区域，黑色区域表示忽略。

---

如有问题，请通过 [GitHub Issues](https://github.com/kompl3xpr/wmonitor/issues) 报告。
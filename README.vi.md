# Prism - Phần mềm dịch thuật AI

<div align="center">

**[English](./README.md) | [中文](./README.zh.md) | [Español](./README.es.md) | [日本語](./README.ja.md) | [한국어](./README.ko.md) | [Tiếng Việt](./README.vi.md)**

🚀 Ứng dụng dịch đa nền tảng mạnh mẽ, sử dụng mô hình ngôn ngữ tiên tiến và công nghệ OCR.

[Tải xuống](#tải-xuống) • [Tính-năng](#tính-năng) • [Bắt-đầu-nhanh](#bắt-đầu-nhanh) • [Tài-liệu](#tài-liệu)

</div>

---
<img src="icon.png" alt="Biểu tượng" width="350" height="350">

## Tính năng

- **🌍 Đa nền tảng** - Hỗ trợ đầy đủ Windows, macOS và Linux với chức năng thống nhất
- **🤖 Dịch AI nâng cao** - Dựa trên Tencent Hunyuan-MT-7B cho dịch thuật chính xác theo ngữ cảnh
- **📸 OCR tích hợp** - Trích xuất và dịch văn bản trực tiếp từ ảnh chụp màn hình với Qwen3-VL-8B-Instruct
- **⚡ Tốc độ cao** - Dịch thời gian thực với độ trễ tối thiểu
- **🎯 Giao diện thân thiện** - UI trực quan trên Vue 3 với trải nghiệm mượt mà
- **🔗 Phím tắt toàn cục** - Tùy chỉnh phím tắt (đang phát triển)
- **💾 Lịch sử cục bộ** - Lưu lịch sử dịch trong SQLite
- **🎨 Kiến trúc hiện đại** - Xây dựng bằng Tauri + Rust cho hiệu năng và bảo mật cao

---

## Công nghệ

### Frontend

- **Vue 3** (3.5.13) - Framework JavaScript hiện đại
- **Vite** (6.0.3) - Công cụ build thế hệ mới
- **Thành phần UI Tauri** - Trải nghiệm ứng dụng desktop gần gũi native

### Backend

- **Rust** (2021 edition) - Ngôn ngữ hệ thống hiệu năng cao
- **Tauri** (2.9.3) - Framework desktop nhẹ
- **Tokio** (1.48.0) - Runtime bất đồng bộ

### AI & xử lý

- **Mô hình dịch** - Tencent Hunyuan-MT-7B
- **Mô hình OCR** - Qwen3-VL-8B-Instruct
- **Nhà cung cấp API** - SiliconFlow
- **Hỗ trợ đầy đủ API tương thích OpenAI cho mô hình tùy chỉnh**

### Lưu trữ & thư viện

- **SQLite** (rusqlite 0.37.0) - CSDL cục bộ
- **Reqwest** (0.12.24) - HTTP client
- **Xử lý ảnh** (0.25.9) - Chụp và xử lý ảnh
- **Phím tắt toàn cục** (2.3.1) - Plugin phím tắt

---

## Bắt đầu nhanh

### Yêu cầu

- Rust 1.91.0 trở lên
- Node.js 18+ và pnpm
- Git

### Cài đặt

**1. Nhân bản kho mã**
```bash
git clone https://github.com/qyzhg/prism.git
cd prism
```

**2. Cài phụ thuộc**
#### Phụ thuộc frontend
```bash
pnpm install
```

#### Phụ thuộc Rust được quản lý bởi Cargo

**3. Lấy API Key**
- Dùng Base URL tương thích OpenAI và API Key của bạn để bắt đầu.
- Đăng ký SiliconFlow qua link mời để nhận tín dụng miễn phí: [https://cloud.siliconflow.cn/i/QhM7Qyuq](https://cloud.siliconflow.cn/i/QhM7Qyuq)

**4. Chạy chế độ phát triển**
```bash
pnpm tauri dev
```

**5. Build bản sản xuất**
```bash
pnpm tauri build
```

---

## Tải xuống

| Nền tảng | Liên kết |
|---------|----------|
| 🪟 Windows | [Bản mới nhất](https://github.com/qyzhg/prism/releases) |
| 🍎 macOS | [Bản mới nhất](https://github.com/qyzhg/prism/releases) |
| 🐧 Linux | Sắp ra mắt |

### Ghi chú cài đặt macOS

Prism dùng chữ ký ad-hoc (không có chứng chỉ Developer ID trả phí), nên Gatekeeper sẽ cảnh báo ở lần mở đầu tiên.

1. Di chuyển `Prism.app` vào `/Applications`.
2. Mở **Terminal** và chạy:
   ```bash
   xattr -cr /Applications/prism.app
   sudo spctl --add --label Prism /Applications/prism.app
   ```
3. Nhấp chuột phải vào app, chọn **Open** và xác nhận một lần. Những lần sau mở bình thường.

---

## Tài liệu

### Cấu hình

Quản lý tùy chỉnh trong bảng cài đặt:

- Chọn cặp ngôn ngữ mặc định
- Quản lý API Key
- Tùy chỉnh phím tắt (đang phát triển)

### Phím tắt

Đang phát triển - Sẽ sớm có

### Mô hình AI

- **Mô hình dịch** - `tencent/Hunyuan-MT-7B` dịch đa ngôn ngữ cấp doanh nghiệp
- **Mô hình OCR** - `Qwen/Qwen3-VL-8B-Instruct` thị giác-ngôn ngữ nâng cao

---

## Lộ trình

- [x] Chức năng dịch cốt lõi
- [x] Tích hợp OCR cho ảnh chụp màn hình
- [x] Cấu hình phím tắt tùy chỉnh
- [ ] Bộ nhớ dịch và quản lý thuật ngữ
- [ ] Dịch hàng loạt tệp
- [ ] Hệ sinh thái plugin
- [ ] Ứng dụng di động đi kèm

---

## Câu hỏi thường gặp (FAQ)

**Hỏi: Có dùng miễn phí được không?**  
Có. Đăng ký SiliconFlow qua link mời để nhận tín dụng miễn phí đủ dùng lâu dài.

**Hỏi: Hỗ trợ những ngôn ngữ nào?**  
Tencent Hunyuan-MT-7B hỗ trợ nhiều ngôn ngữ chính (Trung, Anh, Nhật, Hàn...). Bạn cũng có thể dùng mô hình ưa thích.

**Hỏi: Dữ liệu có được lưu lại không?**  
Lịch sử dịch lưu cục bộ trong SQLite và không tải lên máy chủ. Quyền riêng tư được bảo vệ.

**Hỏi: Có dùng offline được không?**  
Mô hình online cần kết nối. Nếu dùng mô hình cục bộ, có thể làm việc offline.

**Hỏi: Khi nào có phím tắt?**  
Đang phát triển và sẽ phát hành sớm.

---

## Đóng góp

Chào đón Issue và Pull Request. Rất mong đóng góp của bạn!

---

## Giấy phép

Phần mềm theo giấy phép MIT - xem [LICENSE](LICENSE) để biết chi tiết.

---

## Lời cảm ơn

- Xây dựng với [Tauri](https://tauri.app/)
- Dịch vụ dịch trong quá trình phát triển do [SiliconFlow](https://siliconflow.cn/) cung cấp
- UI dùng [Vue 3](https://vuejs.org/)

---

## Trợ giúp

- 🐛 Báo lỗi: [GitHub Issues](https://github.com/qyzhg/prism/issues)

---

<div align="center">

❤️ Phát triển bởi đội Prism@pity

**[⬆ Lên đầu trang](#prism---phần-mềm-dịch-thuật-ai)**

</div>

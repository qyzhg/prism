# Prism - AI 번역 소프트웨어

<div align="center">

**[English](./README.md) | [中文](./README.zh.md) | [Español](./README.es.md) | [日本語](./README.ja.md) | [한국어](./README.ko.md) | [Tiếng Việt](./README.vi.md)**

🚀 고급 언어 모델과 OCR 기술로 구동되는 강력한 크로스플랫폼 AI 번역 애플리케이션.

[다운로드](#다운로드) • [기능](#기능) • [빠른-시작](#빠른-시작) • [문서](#문서)

</div>

---
<img src="icon.png" alt="아이콘" width="350" height="350">

## 기능

- **🌍 크로스플랫폼 지원** - Windows, macOS, Linux 모두 동일한 기능 제공
- **🤖 고급 AI 번역** - Tencent Hunyuan-MT-7B 기반의 문맥 인지 번역
- **📸 내장 OCR** - Qwen3-VL-8B-Instruct로 스크린샷에서 텍스트 추출 및 번역
- **⚡ 초고속 번역** - 실시간, 저지연 번역
- **🎯 직관적 UI** - Vue 3 기반의 부드러운 인터페이스
- **🔗 글로벌 핫키** - 커스텀 가능한 단축키(개발 중)
- **💾 로컬 기록** - SQLite에 번역 내역 저장
- **🎨 현대적 아키텍처** - Tauri + Rust로 높은 성능과 보안

---

## 기술 스택

### 프론트엔드

- **Vue 3** (3.5.13) - 현대적 프로그레시브 프레임워크
- **Vite** (6.0.3) - 차세대 빌드 도구
- **Tauri UI 컴포넌트** - 네이티브급 데스크톱 경험

### 백엔드

- **Rust** (2021 edition) - 고성능 시스템 언어
- **Tauri** (2.9.3) - 경량 데스크톱 프레임워크
- **Tokio** (1.48.0) - 비동기 런타임

### AI 및 처리

- **번역 모델** - Tencent Hunyuan-MT-7B
- **OCR 모델** - Qwen3-VL-8B-Instruct
- **API 제공자** - SiliconFlow
- **OpenAI 호환 커스텀 모델 완전 지원**

### 저장소 및 라이브러리

- **SQLite** (rusqlite 0.37.0) - 로컬 데이터베이스
- **Reqwest** (0.12.24) - HTTP 클라이언트
- **이미지 처리** (0.25.9) - 스크린샷 및 이미지 처리
- **글로벌 핫키** (2.3.1) - 키보드 단축키 플러그인

---

## 빠른 시작

### 필요 환경

- Rust 1.91.0 이상
- Node.js 18+ 및 pnpm
- Git

### 설치

**1. 리포지토리 클론**
```bash
git clone https://github.com/qyzhg/prism.git
cd prism
```

**2. 의존성 설치**
#### 프론트엔드 의존성
```bash
pnpm install
```

#### Rust 의존성은 Cargo가 관리

**3. API Key 준비**
- OpenAI 호환 Base URL과 API Key를 입력하면 바로 사용 가능.
- 초대 링크로 SiliconFlow 가입 시 무료 크레딧 제공: [https://cloud.siliconflow.cn/i/QhM7Qyuq](https://cloud.siliconflow.cn/i/QhM7Qyuq)

**4. 개발 모드 실행**
```bash
pnpm tauri dev
```

**5. 프로덕션 빌드**
```bash
pnpm tauri build
```

---

## 다운로드

| 플랫폼 | 링크 |
|-------|------|
| 🪟 Windows | [최신 버전](https://github.com/qyzhg/prism/releases) |
| 🍎 macOS | [최신 버전](https://github.com/qyzhg/prism/releases) |
| 🐧 Linux | 곧 제공 |

### macOS 설치 메모

Prism는 ad-hoc 서명으로 배포되므로 첫 실행 시 Gatekeeper 경고가 뜰 수 있습니다.

1. `Prism.app`을 `/Applications`로 이동합니다.
2. **Terminal**에서 다음을 실행:
   ```bash
   xattr -cr /Applications/prism.app
   sudo spctl --add --label Prism /Applications/prism.app
   ```
3. Finder에서 우클릭(또는 Control+클릭) 후 **Open**을 선택해 한 번 허용하면 이후엔 정상 실행됩니다.

---

## 문서

### 설정

설정 패널에서 다음을 관리할 수 있습니다:

- 기본 언어 쌍 선택
- API Key 관리
- 단축키 커스터마이즈(개발 중)

### 핫키

개발 중 - 곧 제공

### AI 모델

- **번역 모델** - `tencent/Hunyuan-MT-7B` 엔터프라이즈 다국어 번역
- **OCR 모델** - `Qwen/Qwen3-VL-8B-Instruct` 고급 비전-언어 모델

---

## 로드맵

- [x] 핵심 번역 기능
- [x] 스크린샷 OCR 통합
- [x] 커스텀 단축키 설정
- [ ] 번역 메모리 및 용어집
- [ ] 파일 일괄 번역
- [ ] 플러그인 생태계
- [ ] 모바일 동반 앱

---

## 자주 묻는 질문(FAQ)

**Q: 무료로 사용할 수 있나요?**  
네. SiliconFlow 초대 링크로 가입하면 장기간 쓸 수 있는 무료 크레딧을 받을 수 있습니다.

**Q: 어떤 언어를 지원하나요?**  
Tencent Hunyuan-MT-7B는 중국어, 영어, 일본어, 한국어 등 주요 언어의 상호 번역을 지원합니다. 원하는 모델을 선택해도 됩니다.

**Q: 데이터가 저장되나요?**  
번역 기록은 로컬 SQLite에 저장되며 서버로 전송되지 않습니다. 개인정보는 보호됩니다.

**Q: 오프라인 사용이 가능한가요?**  
온라인 모델은 연결이 필요합니다. 로컬 모델을 사용하면 오프라인도 가능합니다.

**Q: 핫키는 언제쯤 사용 가능한가요?**  
현재 개발 중이며 곧 제공될 예정입니다.

---

## 기여

Issue와 Pull Request를 환영합니다. 기여를 기다립니다!

---

## 라이선스

MIT 라이선스. 자세한 내용은 [LICENSE](LICENSE) 참고.

---

## 감사의 말

- [Tauri](https://tauri.app/)로 제작
- 개발 중 번역 서비스는 [SiliconFlow](https://siliconflow.cn/) 제공
- UI 프레임워크는 [Vue 3](https://vuejs.org/)

---

## 도움말

- 🐛 버그 신고: [GitHub Issues](https://github.com/qyzhg/prism/issues)

---

<div align="center">

❤️ Prism 팀@pity 개발

**[⬆ 맨 위로](#prism---ai-번역-소프트웨어)**

</div>

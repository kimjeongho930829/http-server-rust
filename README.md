# HTTP Server in Rust

Rust 표준 라이브러리만 사용하여 구현한 간단한 HTTP 웹 서버입니다.

## 주요 기능

- HTTP GET 요청 처리
- 정적 파일 제공 (HTML, CSS 등)
- 커스텀 라우팅 지원
- Path Traversal 공격 방어
- HTTP 상태 코드 관리

## 실행 방법

```bash
cargo build
cargo run
```

서버가 `http://127.0.0.1:8080`에서 실행됩니다.

**테스트:**
```bash
curl http://127.0.0.1:8080/
curl http://127.0.0.1:8080/hello
```

## 프로젝트 구조

```
src/
├── main.rs              # 진입점
├── server.rs            # TCP 서버 및 Handler trait
├── wensite_handler.rs   # 요청 핸들러
└── http/                # HTTP 프로토콜 구현
    ├── method.rs
    ├── request.rs
    ├── response.rs
    ├── status_code.rs
    └── query_string.rs
```

## 설정

환경 변수 `PUBLIC_PATH`로 정적 파일 디렉토리를 지정할 수 있습니다 (기본값: `{프로젝트}/public`).

```bash
# Windows
$env:PUBLIC_PATH="C:\my-site\public"; cargo run

# Linux/macOS
PUBLIC_PATH=/path/to/public cargo run
```

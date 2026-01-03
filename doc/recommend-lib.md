# 게임 서버 개발을 위한 Rust 패키지 추천

게임 서버 개발에 자주 사용되는 Rust 패키지들을 정리했습니다.

## 네트워킹

| 패키지 | 용도 | 링크 |
|--------|------|------|
| **tokio** | 비동기 런타임 (필수) | [crates.io](https://crates.io/crates/tokio) |
| **tokio-tungstenite** | WebSocket 서버/클라이언트 | [crates.io](https://crates.io/crates/tokio-tungstenite) |
| **quinn** | QUIC 프로토콜 (UDP 기반 신뢰성) | [crates.io](https://crates.io/crates/quinn) |
| **axum** | HTTP/REST API 서버 (tokio 기반) | [crates.io](https://crates.io/crates/axum) |
| **actix-web** | HTTP/REST API 서버 (고성능) | [crates.io](https://crates.io/crates/actix-web) |
| **tonic** | gRPC 서버/클라이언트 | [crates.io](https://crates.io/crates/tonic) |

## 직렬화

| 패키지 | 용도 | 링크 |
|--------|------|------|
| **serde** | 직렬화 프레임워크 (필수) | [crates.io](https://crates.io/crates/serde) |
| **bincode** | 바이너리 직렬화 (빠름, 게임에 적합) | [crates.io](https://crates.io/crates/bincode) |
| **rmp-serde** | MessagePack (효율적인 바이너리) | [crates.io](https://crates.io/crates/rmp-serde) |
| **flatbuffers** | 제로카피 직렬화 (C++과 호환) | [crates.io](https://crates.io/crates/flatbuffers) |
| **serde_json** | JSON 직렬화 | [crates.io](https://crates.io/crates/serde_json) |

## 데이터베이스

| 패키지 | 용도 | 링크 |
|--------|------|------|
| **sqlx** | 비동기 SQL (PostgreSQL, MySQL, SQLite) | [crates.io](https://crates.io/crates/sqlx) |
| **redis** | Redis 클라이언트 (캐시, 세션, pub/sub) | [crates.io](https://crates.io/crates/redis) |
| **deadpool** | 커넥션 풀 | [crates.io](https://crates.io/crates/deadpool) |
| **mongodb** | MongoDB 드라이버 | [crates.io](https://crates.io/crates/mongodb) |

## 동시성 / ECS

| 패키지 | 용도 | 링크 |
|--------|------|------|
| **dashmap** | 동시성 HashMap | [crates.io](https://crates.io/crates/dashmap) |
| **crossbeam** | 채널, 큐, 동시성 유틸리티 | [crates.io](https://crates.io/crates/crossbeam) |
| **parking_lot** | 빠른 Mutex/RwLock | [crates.io](https://crates.io/crates/parking_lot) |
| **bevy_ecs** | Entity Component System | [crates.io](https://crates.io/crates/bevy_ecs) |
| **rayon** | 데이터 병렬 처리 | [crates.io](https://crates.io/crates/rayon) |

## 유틸리티

| 패키지 | 용도 | 링크 |
|--------|------|------|
| **uuid** | UUID 생성 | [crates.io](https://crates.io/crates/uuid) |
| **rand** | 난수 생성 | [crates.io](https://crates.io/crates/rand) |
| **chrono** | 시간/날짜 처리 | [crates.io](https://crates.io/crates/chrono) |
| **tracing** | 구조화된 로깅/추적 | [crates.io](https://crates.io/crates/tracing) |
| **tracing-subscriber** | tracing 출력 설정 | [crates.io](https://crates.io/crates/tracing-subscriber) |
| **anyhow** | 간편한 에러 처리 (애플리케이션용) | [crates.io](https://crates.io/crates/anyhow) |
| **thiserror** | 커스텀 에러 타입 (라이브러리용) | [crates.io](https://crates.io/crates/thiserror) |
| **config** | 설정 파일 로드 (TOML, YAML, JSON) | [crates.io](https://crates.io/crates/config) |
| **dotenv** | 환경 변수 로드 | [crates.io](https://crates.io/crates/dotenv) |

## 추천 조합 (Cargo.toml)

```toml
[package]
name = "game-server"
version = "0.1.0"
edition = "2021"

[dependencies]
# 비동기 런타임
tokio = { version = "1", features = ["full"] }

# 네트워킹
axum = "0.7"                    # REST API
tokio-tungstenite = "0.21"      # WebSocket

# 직렬화
serde = { version = "1", features = ["derive"] }
bincode = "1"                   # 게임 패킷용
serde_json = "1"                # 설정/API용

# 데이터베이스
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
redis = { version = "0.24", features = ["tokio-comp"] }

# 동시성
dashmap = "5"
parking_lot = "0.12"

# 유틸리티
uuid = { version = "1", features = ["v4", "serde"] }
rand = "0.8"
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1"
thiserror = "1"
config = "0.14"
```

## 장르별 추가 추천

### MMORPG / 실시간 액션
- **quinn**: UDP 기반 QUIC (낮은 레이턴시)
- **bevy_ecs**: 게임 로직용 ECS
- **nalgebra**: 선형대수/물리 계산

### 턴제 / 캐주얼
- **axum** + **tokio-tungstenite**: HTTP + WebSocket 조합
- **sqlx**: 상태 저장용 DB

### 매치메이킹 서버
- **redis**: pub/sub, 대기열 관리
- **tonic**: 마이크로서비스 간 gRPC 통신

## C++과의 비교

| C++ | Rust | 비고 |
|-----|------|------|
| Boost.Asio | tokio | 비동기 I/O |
| Protobuf | prost/tonic | 직렬화/gRPC |
| FlatBuffers | flatbuffers | 제로카피 직렬화 |
| EnTT | bevy_ecs | Entity Component System |
| spdlog | tracing | 로깅 |
| folly::ConcurrentHashMap | dashmap | 동시성 해시맵 |

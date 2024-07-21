# 번호 맞추기 스무고개 게임

이 프로젝트는 사용자가 1부터 100 사이의 랜덤한 번호를 맞추는 게임입니다. 사용자는 최대 20번의 시도 내에 번호를 맞추어야 하며, 번호를 맞추면 승리하고, 20번의 시도 내에 맞추지 못하면 패배하게 됩니다.

## 설명 및 기능

이 프로젝트는 Rust 프로그래밍 언어를 학습하기 위한 목적으로 공식 가이드 문서를 참조하여 클론 코딩한 프로젝트 입니다.
추가적으로 주석을 통해 각 코드에 대한 설명을 기입하였으며 게임에 몇 가지 추가 규칙을 정하여 기능을 추가한 코드 입니다.

## 규칙
* 1에서 100 사이의 랜덤한 번호를 생성합니다.
* 사용자가 입력한 번호가 정답 번호보다 높은지 낮은지 알려줍니다.
* 최대 20번의 시도 내에 번호를 맞추면 승리합니다. (추가된 규칙)
* 20번의 시도 내에 번호를 맞추지 못하면 패배합니다. (추가된 규칙)

## 사용 방법

### 설치

1. Rust 설치: [Rust 공식 웹사이트의 install 메뉴](https://www.rust-lang.org/tools/install)에서 설치합니다.

### 실행

1. 프로젝트 디렉토리로 이동합니다.
2. 아래 명령어를 실행하여 게임을 시작합니다.

```sh
cargo run
```

## 참조 문서
* [Rust Programing The Unstable Book - 2. Programming a Guessing Game](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html)
* [Rust Programing The Unstable Book 한국 러스트 사용자 그룹 공식 번역 문서 - 2. 추리 게임](https://doc.rust-kr.org/ch02-00-guessing-game-tutorial.html)

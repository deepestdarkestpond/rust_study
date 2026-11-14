# Rust CM55 학습

Rust 기초부터 커스텀 Cortex-M55 SoC의 Embassy 및 드라이버 구현까지 진행하는 학습 저장소입니다.

- `src/main.rs`: 사용자가 직접 작성하는 현재 실습 코드
- `doc/index.html`: 단계별 설명, 질문과 답변, 확인한 실행 결과
- `.github/workflows/pages.yml`: 학습 문서를 GitHub Pages에 게시하는 설정

현재 코드는 PC용 입문 프로그램입니다. CM55용 빌드와 보드 실행은 아직 진행하지 않았습니다.
실습 중에는 의도적으로 컴파일 오류를 만드는 코드가 저장될 수 있습니다.

## PC에서 실행

```bash
cargo run
```

## 학습 문서 게시

GitHub 저장소의 **Settings → Pages → Build and deployment → Source**를
**GitHub Actions**로 설정합니다.

`main` 브랜치의 `doc/` 또는 Pages 워크플로가 변경되면 문서를 자동으로 게시합니다.
**Actions → Publish study notes to GitHub Pages → Run workflow**에서도 수동 실행할 수 있습니다.
Rust 코드를 빌드하지 않고 `doc/`의 정적 파일만 게시하므로 의도적인 컴파일 오류 실습과 별개로 동작합니다.

게시가 성공하면 기본 주소는 다음과 같습니다.

https://deepestdarkestpond.github.io/rust_study/

워크플로 구성 참고: [GitHub Pages 공식 문서](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages).

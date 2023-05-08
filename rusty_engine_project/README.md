
# Rusty Engine Project

러스티 엔진이란 [BEVY](https://bevyengine.org/) 을 기반으로 단순화 계층을 래핑하여 만든 오픈 소스 게임 엔진 입니다.

러스티 엔진의 목적은 단순한 Rust Interface 를 제공하는 데 있습니다.


### Installation
- Collider 
```shell
cargo install rusty_engine --example collider
```
```shell
collider $image-path
```
- collider 확장자 파일을 생성했다면, 0.5픽셀로 반올림해야 합니다.
- 0.5 픽셀보다 더 세밀한 것은 전혀 의미 없기 때문입니다.
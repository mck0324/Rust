콘솔에서 사용자의 입력을 읽고 대답을 출력하는 간단한 명령줄 애플리케이션
-Functions
-Basic Data Types
-Standard Library (표준 라이브러리와 가장 중요한 메모리 소유권 규칙)
-Memory Ownership

내가 해볼건 화성에서의 무게 계산기이다.
사용자가 지구에서의 본인 무게를 제공하면 프로그램은 화성의 무게로 출력해줄것이다.


1.Basic Data Types
-Booleans
-Characters
-Integers
-Floats

데이터 유형이 - u로 시작하면 무부호 정수 (Only 양수)
           - i로 시작하면 유부호 정수 (양수 / 음수 둘다 됨)

u8,i8 = 8bit 숫자 또는 1byte
u16, i16 
u32, i32 
u64, i64 
u128, i128  = 128bit


8bit = 1byte -> 알파벳,숫자,기본 특수 문자
한글은 보통 한글자에 3바이트 정도 됨

러스트에는 usize, isize가 있음 -> 아키텍쳐에 종속된 타입으로 32bit 아키텍쳐환경에선 32bit , 64bit 아키텍쳐환경에선 64bit
러스트에는 두 종류의 부동 소수점 숫자가 존재,32bit인 f32 64bit인 f64가 존재
러스트에서 bool은 1byte
러스트에서 char은 하나의 유니코드 값을 갖는다.(4byte) -> ASCII 문자집합을 char로 저장한다면 메모리를 낭비하게 됨
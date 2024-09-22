use std::io;

fn main() {
    let mut input = String::new();
    some_fn(input);
    //input은 문자열 String의 소유자 및 문자열에 대한 포인터
    //컴파일 시점에 문자열의 크기를 알지 못하기 때문에 힙에 저장 -> 스택에는 힙을 가리키는 포인터를 저장.문자열의 크기 같은 메타데이터를 추가로 저장
    //input이 범위를 벗어나면,힙에 있는 문자열은 해제
    // let mut s = input;
    // 이럴경우 두 소유자가 생기는데
    //범위에 벗어나다 = 함수가 끝나다로 봤을때 -> 문자열을 해제 -> 그럼 이중 해제를 하게되는데 메모리 손상을 일으킬 수 있음 그래서 심각한 보안 취약점이 될 수 있음
    io:: stdin().read_line(&mut input);
    
    let mut mars_weight = calculate_weight_on_mars(100.0);
    // println!("Number: {}, String: {}", 100, "abc");
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}kg", mars_weight);

}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn some_fn(s: String) {}

// some_fn을 만들어서 input을 호출했을때 이 문자열의 소유권이 some_fn의 변수 s로 되었기에
// some_fn함수가 끝날때 문자열은 해제됨으로 이 포인터는 더이상 유효하지 않게됨 -> 메모리주소에 있지 않고 그 안에 뭐가 있는지 모르게 됨 -> 메모리 손상되고 프로그램은 동작하지 않는다. 
// 그래서 러스트 컴파일러는 이런 오류를 막기 위해 단일 소유자 규칙을 강제 적용하고 있음
// 그러므로 매개 변수를 함수에 전달하고 이 함수 반환 이후에도 계속 사용할 수 있도록 한다는 뜻 -> 이것만이 아니라 이걸 보완하기위해 차용이라는 기능이 있음
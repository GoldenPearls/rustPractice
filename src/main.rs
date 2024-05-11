fn main() {
    println!("Hello, world!");
}

// 부호 없는 64비트 정수인 u64 타입의 두 매개변수 n과 m를 받아  반환
fn gcd(mut n: u64, mut m: u64) -> u64{ //fn 키워드 : 함수 정의
    assert!(n!=0 && m!=0);
    while m !=0 {
        if m<n{
            let t = m;
            m = n;
            n = t;
        }
        m= m % n;
    }
    n
}

#[test] //어트리뷰트의 예, 테스트 함수는 어디에 넣어도 위치 상관 무
fn test_gcd(){
    assert_eq!(gcd(14, 15),1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

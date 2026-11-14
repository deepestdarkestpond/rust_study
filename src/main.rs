use std::println as print_line;

fn main() {
    // 1. 공유 참조: 주소가 복사되고 같은 값을 가리킨다.
    print_line!("[1] Shared reference: copy");
    let value: u32 = 10;
    let p: &u32 = &value;
    let q = p;
    print_line!("*p = {}, *q = {}", *p, *q);
    print_line!("Same target: {}", std::ptr::eq(p, q));

    // 2. 가변 참조: 이 대입에서는 p가 q로 이동한다.
    print_line!("\n[2] Mutable reference: move");
    let mut value: u32 = 10;
    let p: &mut u32 = &mut value;
    let q = p;
    *q = 20;
    // *p = 30; // 주석을 해제하면 이동한 p를 사용해서 컴파일 오류가 난다.
    print_line!("value = {value}");

    // 3. 원시 포인터: 주소 복사 후에도 p와 q를 모두 사용할 수 있다.
    print_line!("\n[3] Raw pointer: copy");
    let mut value: u32 = 10;
    let p: *mut u32 = &raw mut value;
    let q = p;
    // SAFETY: p와 q는 살아 있는 지역 변수 value를 가리킨다.
    // 정렬과 초기화가 올바르며, 두 쓰기는 같은 스레드에서 순서대로 수행한다.
    // 이 접근과 충돌하는 참조는 없다.
    unsafe {
        *q = 20;
        *p += 1;
    }
    print_line!("Same target: {}", std::ptr::eq(p, q));
    print_line!("value = {value}");

    // 4. Box: 대입은 소유권 이동, clone은 별도 할당과 값 복제다.
    print_line!("\n[4] Box: move and clone");
    let p: Box<u32> = Box::new(10);
    let mut q = p;
    // print_line!("{}", *p); // 주석을 해제하면 이동한 p를 사용해서 오류가 난다.
    let mut r = q.clone();
    *q = 20;
    *r = 30;
    print_line!("*q = {}, *r = {}", *q, *r);
    print_line!("Same target: {}", std::ptr::eq(&*q, &*r));
    // q와 r은 각각 자신의 힙 메모리를 자동으로 해제한다.
}

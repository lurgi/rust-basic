// ========================================
// 과제 3: 복잡한 생명주기 관계
// ========================================
// 여러 생명주기 매개변수를 사용하는 구조체를 만드세요.
//
// 요구사항:
// - Context<'a, 'b> 구조체
//   - title: &'a str
//   - body: &'b str
//
// - impl<'a, 'b> Context<'a, 'b>
//   - new(title: &'a str, body: &'b str) -> Context<'a, 'b>
//
//   - title(&self) -> &'a str
//     * title 필드 반환
//
//   - body(&self) -> &'b str
//     * body 필드 반환
//
//   - display(&self)
//     * "제목: {title}\n내용: {body}" 형식으로 출력
//
// - create_summary<'a>(ctx: &Context<'a, '_>) -> &'a str
//   * Context의 title만 반환
//   * body의 생명주기는 무관 ('_로 생략)

// TODO: Context 구조체를 정의하세요

// TODO: Context의 모든 메서드를 구현하세요

// TODO: create_summary 함수를 구현하세요

pub fn run() {
    // println!("=== 과제 3: 복잡한 생명주기 관계 ===");

    // let title = String::from("Rust 프로그래밍");

    // {
    //     let body = String::from("생명주기는 참조의 유효 범위를 나타냅니다.");
    //     let ctx = Context::new(&title, &body);

    //     ctx.display();

    //     let summary = create_summary(&ctx);
    //     println!("\n요약: {}", summary);
    // } // body는 여기서 해제

    // // title은 여전히 유효
    // println!("\n제목은 여전히 유효: {}", title);
}

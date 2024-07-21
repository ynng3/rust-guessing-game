use std::io; // Rust Standard(std) Library 크레이트에서 표준 입출력 기능을 사용합니다.
use rand::Rng; // 난수 생성을 위해 rand 크레이트를 가져옵니다. (외부 라이브러리 이므로 Cargo.toml에서 dependencies 설정을 먼저 진행해야 합니다.)
use std::cmp::Ordering; // 비교 결과를 나타내는 Ordering 열거형을 사용합니다.

fn main() {
    let goal = rand::thread_rng().gen_range(1..=100); // 1 ~ 100 중 랜덤한 정수를 goal 변수에 저장
    let mut attempts = 0; // 시도한 횟수를 담을 가변(mut)변수를 생성합니다.

    println!("번호 맞추기 스무고개!\n");
    println!("1~100까지의 랜덤한 번호를 생성합니다.");
    println!("20번 이내로 생성된 번호를 맞추면 승리하는 게임입니다.");
    println!("번호를 제시하였을 때 틀렸을 경우, High인 지 Low인 지 출력됩니다.\n");

    // 맞출 때 까지 반복
    loop {
        if attempts < 20 { // 시도 횟수가 20번 미만일 때
            println!("번호를 입력하세요 (현재 시도 횟수: {attempts})");
        } else { // 시도 횟수가 20번 이상이 되었을 때
            println!("기회를 모두 소진하여 당신이 졌습니다. (정답 번호: {goal})");
            break;
        }

        // 사용자가 입력한 값을 담을 가변(mut)변수
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // guess 변수에 사용자가 입력한 값을 저장합니다.
            .expect("입력을 읽는 데 실패했습니다."); // 예외 발생시 에러

        let guess: u32 = match guess.trim().parse() { // 사용자가 입력한 값을 trim 함수를 통해 공백을 제거하고 파싱합니다.
            Ok(num) => num, // number 타입인지 확인하고 number 타입이 맞으면 입력한 값을 number 타입으로 파싱합니다.
            Err(_) => { // number 타입으로 변환이 실패한 경우
                println!("숫자만 입력해야 합니다!");
                continue; // 이번 loop를 건너뜁니다.
            },
        };

        if guess < 1 || guess > 100 { // 입력한 값이 1보다 작거나 100보다 큰 지 확인
            println!("1 ~ 100 까지의 수만 입력할 수 있습니다.");
            break;
        }

        attempts += 1; // 위의 유효성 검사에 통과한 경우 시도한 횟수(attempts 변수)에 1을 더합니다.

        match guess.cmp(&goal) { // 사용자가 입력한 값(guess 변수)을 정답 숫자(goal 변수)와 비교합니다.
            Ordering::Less => println!("입력한 숫자가 정답 숫자 보다 작습니다.\n"), // 입력한 값(guess 변수)이 정답 숫자(goal 변수) 보다 작을 때 출력
            Ordering::Greater => println!("입력한 숫자가 정답 숫자 보다 큽니다.\n"), // 입력한 값(guess 변수)이 정답 숫자(goal 변수) 보다 클 때 출력
            Ordering::Equal => { // 추측이 비밀 숫자와 같으면
                println!("번호를 맞췄습니다. 당신이 이겼습니다!");
                println!("(정답 번호: {goal} / 시도한 횟수: {attempts})");
                break; // 반복을 종료합니다.
            }
        }
    }
}

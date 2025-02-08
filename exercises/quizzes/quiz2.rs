// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

// 다음 섹션에 대한 퀴즈입니다:
// - 문자열
// - 벡터
// - 이동 시맨틱
// - 모듈
// - 열거형
//
// 함수 형태로 작은 기계를 만들어 봅시다. 입력으로 문자열과 명령어 목록을 줄 것입니다.
// 이 명령어들은 문자열에 어떤 작업을 적용할지를 결정합니다. 작업은 다음과 같습니다:
// - 문자열을 대문자로 변환
// - 문자열을 잘라내기
// - 지정된 횟수만큼 문자열에 "bar"를 추가
//
// 정확한 형태는 다음과 같습니다:
// - 입력은 2개의 요소로 이루어진 튜플의 벡터가 될 것입니다.
// 첫 번째 요소는 문자열이고, 두 번째 요소는 명령어입니다.
// - 출력 요소는 문자열의 벡터가 될 것입니다."

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    // use std::process::Command;

    use super::Command;

    fn append(mut string: String, times: usize) -> String {
        for _ in 0..times {
            string.push_str("bar");
        }
        string
    }

    // TODO: Complete the function as described above.
    pub fn transformer(input: &[(String, Command)]) -> Vec<String> {
        let mut res = Vec::new();
        for (string, command) in input {
            match command {
                Command::Uppercase => res.push(string.to_uppercase()),
                Command::Trim => res.push(string.trim().to_string()),
                Command::Append(times) => res.push(append(string.clone(), *times)),
            }
        }
        res
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(&input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

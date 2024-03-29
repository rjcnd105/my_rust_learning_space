mod _3_5_my_enums;
mod _3_6_my_trait;
mod _3_my_file;

use crate::_3_my_file::Read;
use _3_my_file::{close, open, File};

// -- file
fn main() {
    //---  3_1_my_files

    // let f1 = File::new("f1.txt");
    let file_data = vec![114, 117, 115, 116, 33];
    let mut f2 = File::new_with_data("f2.txt", &file_data);

    let mut buffer: Vec<u8> = vec![];

    // unwrap 사용은 오류 처리에 대한 내용이 없으므로 좋은 사용이 아니게 될 수 있음.
    f2 = open(f2).unwrap();
    let f2_length = f2.read(&mut buffer).unwrap();
    f2 = open(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    // println!("{:?}", f2);
    println!("[3_1_my_files]");
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{:?}", text);
    println!();

    //---  3_5_my_enums
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    println!("[3_5_my_enums]");
    for line in log.lines() {
        let d = _3_5_my_enums::parse_log(line);

        println!("{:?}, {}", d.0, d.1)
    }
    println!();

    //--- 3_6_my_trait

    //--- deref
    // https://stackoverflow.com/questions/53132035/how-to-pattern-match-on-values-inside-a-type-implementing-deref-such-as-box-wiㄴ
    enum SomeEnum {
        SomeEntry(String),
        AnotherEntry,
    }

    // Box: 힙 영역 사용
    let boxed_value = Box::new(SomeEnum::AnotherEntry);
    // Box deref: 힙 -> 스택, moved 되어 이후로 boxed_value 사용 불가
    let deref_value = *boxed_value;

    let boxed_value2 = Box::new(SomeEnum::AnotherEntry);
    // 스택의 메모리 영역의 참조, borrowed 이므로 이후에 boxed_value2 사용 가능
    let deref_ref_value = &*boxed_value2;
}

// ------------------------------------------------
// alias type
#[allow(dead_code)]
type AliasString = String;

// newtype 패턴, 이 경우 Hostname은 String의 alias가 아니라 새로운 타입이다.
struct Hostname(String); // <1>

fn my_new_type() {
    let ordinary_string = String::new();
    // error, connect는 새로운 Hostname 타입을 받기 때문에 String은 에러남
    // connect(ordinary_string);
}

#[allow(dead_code)]
fn connect(host: Hostname) {
    // <2>
    println!("connected to {}", host.0); // <3>
}

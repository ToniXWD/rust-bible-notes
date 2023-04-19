fn string_slice() {
    let x = String::from("hello world");
    let a = &x[0..2];
    let b = &x[1..=6];
    let c = &x[1..2];
    
    println!("字符串x占用空间{}",std::mem::size_of_val(x.as_str()));
    println!("字符串'周杰伦'占用空间{}",std::mem::size_of_val("周杰伦"));
    
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = &arr1[1..3];
}

fn string_del() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let b = &a[1..3];
    assert_eq!(b, &[2, 3])
}

fn string_insert() {
    let mut s = String::from("Hello rust");
    s.insert(5, ',');
    println!("{}", s);
}

fn string_replace() {
    let mut s = String::from("Hello rust");
    let s1 = s.replace("l", "L");
    let s2 = s.replacen("l", "L", 1);
    s.replace_range(0..5, "hi");
    println!("s={}", s);
    println!("s1={}", s1);
    println!("s2={}", s2);
    s.truncate(2);
    println!("s={}", s);
}

fn string_concat() {
    let a = String::from("hello");
    let b = a + " " + "world";
    println!("{}", b);
}

fn string_loop() {
    let a = String::from("hello world");
    for ch in a.chars() {
        print!("{}", ch);
    }
    println!("");
    let b = "hello world".to_string();
    for ch in b.chars() {
        print!("{}",&ch);
    }
    println!("");
}

fn translate() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // 修改上面的行让代码工作
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"")
}

fn ch_arr() {
    use std::str;
    // 注意，这并不是 `&str` 类型了！
    let bytestring: &[u8; 21] = b"this is a byte string";


    // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
    println!("A byte string: {:?}", bytestring);

    // 字节数组也可以使用转义
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但是不支持 unicode 转义
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // raw string
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 将字节数组转成 `str` 类型可能会失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节数组可以不是 UTF-8 格式
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // 但是它们未必能转换成 `str` 类型
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}

fn utf8_slice_test() {
    use utf8_slice;

    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 7);
    // 结果是 "🚀"
    println!("{}", rocket);
}

fn main() {
    // string_slice();
    // string_del();
    // other_slices();
    // string_insert();
    // string_replace();
    // string_concat();
    // string_loop();
    // translate();
    // ch_arr();
    // utf8_slice_test();
}

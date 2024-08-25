fn main() {
    let v = vec![3, 5, 7];
    println!("{:?}", v);

    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    println!("{:?}", v2);

    let six: &i32 = &v2[1];
    println!("{:?}", six);

    let fifth: Option<&i32> = v2.get(4);

    match fifth {
        Some(fifth) => println!("Fifth element is: {}", fifth),
        None => println!("There is not fifth element"),
    }

    let mut v3 = vec![1, 2, 3, 4, 5];

    let first = &v3[0];

    /* This error is due to the way vectors work: because vectors put the values next to each
       other in memory, adding a new element onto the end of the vector might require allocating
       new memory and copying the old elements to the new space, if there isn’t enough room
       thread_local!(static STATIC: Type = init);put all the elements next to each other where
       the vector is currently stored.
    */
    // v3.push(6);
    println!("{}", first);

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{i}");
    }

    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}")
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(0.5),
        SpreadsheetCell::Text(String::from("Hello there!")),
    ];

    println!("{:?}", row);

    {
        let v = vec![1, 4, 6, 2];
        println!("{:?}", v);
    }

    println!("{:?}", v);
}

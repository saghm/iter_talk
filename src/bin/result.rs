fn main() {
    println!("into iter:");

    let ok: Result<u32, String> = Ok(12);
    let ok_as_vec: Vec<_> = ok.into_iter().collect();
    println!("    Ok into vec: {:?}", ok_as_vec);

    let err: Result<u32, String> = Err("something bad happened!".to_string());
    let err_as_vec: Vec<_> = err.into_iter().collect();
    println!("    Err into vec: {:?}", err_as_vec);

    println!("from iter:");

    let ok_results = vec![Ok(1), Ok(2), Ok(3), Ok(4)];
    let ok_final_result: Result<Vec<_>, String> = ok_results.into_iter().collect();
    println!("    vec into Ok: {:?}", ok_final_result);

    let err_results = vec![
        Ok(1),
        Ok(2),
        Err("something bad happened!".to_string()),
        Ok(4),
    ];
    let err_final_result: Result<Vec<_>, String> = err_results.into_iter().collect();
    println!("    vec into Err: {:?}", err_final_result);
}

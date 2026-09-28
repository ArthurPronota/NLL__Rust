fn main() {
    let mut v = vec![1, 2, 3, 4, 5] ;

    let r1 = &v[0] ;
    
    let r2 = &mut v ;   // до NLL ошибка, после NLL ок: r1 больше не используется
    r2.push(6);

    println!("{r2:?}") ;  // Out: [1, 2, 3, 4, 5, 6]
}
